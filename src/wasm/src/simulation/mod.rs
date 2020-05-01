extern crate rand;
use crate::na::{Point2};
use std::cell::{RefCell};
use std::rc::Rc;
use rand::{SeedableRng, Rng, rngs::SmallRng};
use rand::distributions::{Normal, Distribution};
use uuid::Uuid;
use crate::math::Interpolator;
use crate::creature::*;
use crate::stage::*;
// use crate::timer::Timer;

pub mod behaviours;

// just to prevent infinite loops
const MAX_STEPS : usize = 1_000_000;

pub type Step = usize;

// what phase of the time step is it?
#[derive(Debug, Copy, Clone)]
pub enum Phase {
  REPRODUCE, // only called when going from previous step
  INIT, // setup
  PRE, // before each step
  ORIENT,
  MOVE,
  ACT,
  POST, // after each step
  FINAL,
}

pub trait StepBehaviour {
  // if you need &mut self, use Cell or RefCell
  fn apply(&self, phase : Phase, generation : &mut Generation, sim : &Simulation);
}

pub trait ReproductionBehaviour {
  fn reproduce(&self, creatures : &Vec<Creature>, sim : &Simulation) -> Vec<Creature>;
}

// behaviour to reset parameters on step
#[derive(Debug, Copy, Clone)]
pub struct ResetBehaviour;
impl StepBehaviour for ResetBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, _sim : &Simulation){
    // setup
    if let Phase::PRE = phase {
      generation.creatures.iter_mut()
        .filter(|c| c.is_alive())
        .for_each(|c| {
          match c.get_objective() {
            Some(o) => c.status_history.push(o.reason),
            _ => {},
          };
          c.reset_objective();
        });
    }
  }
}

pub struct Simulation {
  pub rng : Rc<RefCell<SmallRng>>,
  // Area this simulation occurs in
  pub stage : Box<dyn Stage>,
  pub food_per_generation : Interpolator,
  pub generations : Vec<Generation>,
  pub behaviours : Vec<Box<dyn StepBehaviour>>,
  pub reproduction_behaviour : Box<dyn ReproductionBehaviour>,
  callbacks : Vec<Box<dyn FnMut(&mut Simulation) -> ()>>,
}

// Starting point for creating simulations
impl Simulation {
  pub fn new( stage : Box<dyn Stage>, seed: u64, food_per_generation: Interpolator ) -> Self {

    let generations = Vec::new();

    Simulation {
      stage,
      generations,
      food_per_generation,
      behaviours : vec![Box::new(ResetBehaviour)],
      reproduction_behaviour : Box::new(behaviours::BasicReproductionBehaviour),
      // prepare a deterministic generator:
      rng: Rc::new(RefCell::new(SmallRng::seed_from_u64(seed))),
      callbacks: vec![],
    }
  }

  pub fn set_food_per_generation(&mut self, food_per_generation: Interpolator) {
    self.food_per_generation = food_per_generation;
  }

  pub fn add_behavour(&mut self, b : Box<dyn StepBehaviour>){
    self.behaviours.push(b)
  }

  pub fn set_reproduction_behaviour(&mut self, b : Box<dyn ReproductionBehaviour>){
    self.reproduction_behaviour = b;
  }

  pub fn add_generation_callback<F: 'static>(&mut self, f : F)
  where F : FnMut(&mut Simulation) -> () {
    self.callbacks.push(Box::new(f));
  }

  fn call_callbacks(&mut self){
    let mut cbs = vec![];
    std::mem::swap(&mut self.callbacks, &mut cbs);
    cbs.iter_mut().for_each(|f| f(self));
    std::mem::swap(&mut self.callbacks, &mut cbs);
  }

  pub fn run(&mut self, creatures: Vec<Creature>, max_generations : u32){
    let mut generation = Generation::new(self, creatures, self.generate_food());
    let mut keep_going = generation.has_living_creatures();

    for _gen in 1..max_generations {
      if !keep_going { break; }

      let creatures = self.exec_reproduction(&generation.creatures);
      self.generations.push(generation);
      self.call_callbacks();
      let next = Generation::new( self, creatures, self.generate_food() );
      generation = next;
      keep_going = generation.has_living_creatures();
    }

    self.generations.push(generation);
    self.call_callbacks();
  }

  pub fn exec_reproduction(&self, creatures : &Vec<Creature>) -> Vec<Creature> {
    self.reproduction_behaviour.reproduce(&creatures, &self)
  }

  pub fn get_random_location(&self) -> Point2<f64> {
    self.stage.get_random_location(&mut self.rng.borrow_mut())
  }

  pub fn get_random_float(&self, from : f64, to : f64) -> f64 {
    let mut rng = self.rng.borrow_mut();
    rng.gen_range(from, to)
  }

  pub fn get_random_gaussian(&self, mean : f64, var : f64) -> f64 {
    let normal = Normal::new(mean, var);
    normal.sample(&mut *self.rng.borrow_mut())
  }

  pub fn generate_food(&self) -> Vec<Point2<f64>> {
    let gen = self.generations.len();
    let num_foods = self.food_per_generation.get(gen as f64).round() as u32;
    (0..num_foods).map(|_n|
      self.get_random_location()
    ).collect()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FoodStatus {
  Available,
  Eaten(usize), // step the food was eaten at
}

pub trait Edible {
  fn get_edible_id(&self) -> Uuid;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Food {
  pub id : Uuid,
  pub position: Point2<f64>,
  pub status: FoodStatus,
}
impl Food {
  pub fn new(position: Point2<f64>) -> Self {
    Self {
      id: Uuid::new_v4(),
      position,
      status: FoodStatus::Available
    }
  }
  pub fn is_eaten(&self) -> bool { self.status != FoodStatus::Available }
}

impl Edible for Food {
  fn get_edible_id(&self) -> Uuid {
    self.id
  }
}

// Each generation of the simulation. A collection of creatures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generation {
  pub steps : Step, // total steps this generation took to complete
  pub creatures : Vec<Creature>,
  pub food : Vec<Food>, // tuple showing the step the food was eaten
}

impl Generation {
  pub fn new( sim : &Simulation, creatures: Vec<Creature>, food_locations: Vec<Point2<f64>> ) -> Self {

    Generation::generate(sim, creatures, food_locations)
  }

  fn generate(sim : &Simulation, creatures: Vec<Creature>, food_locations: Vec<Point2<f64>>) -> Self {
    let food = food_locations.iter().map(|p| {
      Food::new(*p)
    }).collect();

    let mut gen = Generation {
      creatures,
      food,
      steps: 1,
    };

    gen.run_phase(Phase::INIT, sim);
    gen.step_to_completion(sim);
    gen.run_phase(Phase::FINAL, sim);

    gen
  }

  pub fn has_living_creatures(&self) -> bool {
    self.creatures.iter().any(|c| c.is_alive())
  }

  pub fn has_active_creatures(&self) -> bool {
    self.creatures.iter().any(|c| c.is_active())
  }

  pub fn get_available_food(&self) -> Vec<Food> {
    self.food.iter().filter(|f| {
      !f.is_eaten()
    }).map(|f| f.clone()).collect()
  }

  pub fn mark_food_eaten(&mut self, food : &Food){
    if let Some(index) = self.food.iter().position(|f| *f == *food) {
      self.food[index].status = FoodStatus::Eaten(self.steps);
    }
  }

  // advance the generation to its end
  fn step_to_completion(&mut self, sim : &Simulation) {
    // break when all asleep or dead
    while self.has_active_creatures() {
      self.step(sim);
    }
  }

  fn run_phase(&mut self, phase : Phase, sim : &Simulation){
    // let _timer = Timer::new(format!("phase {:?}", phase));
    sim.behaviours.iter().enumerate().for_each(
      |(_i, b)| {
        // let _timer = Timer::new(format!("behaviour {}", i));
        b.apply(phase, self, &sim)
      }
    );
  }

  fn step(&mut self, sim : &Simulation){
    assert!(self.steps < MAX_STEPS);

    // let _timer = Timer::new(String::from("Step"));
    self.run_phase(Phase::PRE, sim);
    self.run_phase(Phase::ORIENT, sim);
    self.run_phase(Phase::MOVE, sim);
    self.run_phase(Phase::ACT, sim);
    self.run_phase(Phase::POST, sim);

    self.steps += 1;
  }
}
