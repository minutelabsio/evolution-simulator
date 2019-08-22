extern crate rand;
use crate::na::{Point2};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use rand::{SeedableRng, Rng, rngs::SmallRng};
use crate::creature::*;
use crate::stage::*;

pub mod behaviours;

// just to prevent infinite loops
const MAX_STEPS : usize = 1000;

// what phase of the time step is it?
#[derive(Debug, Copy, Clone)]
pub enum Phase {
  INIT, // setup
  PRE, // before each step
  ORIENT,
  MOVE,
  ACT,
  POST, // after each step
}

pub trait StepBehaviour {
  // if you need &mut self, use Cell or RefCell
  fn apply(&self, phase : Phase, generation : &mut Generation, sim : &Simulation);
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
          c.reset_objective();
        });
    }
  }
}

pub struct Simulation {
  rng : Rc<RefCell<SmallRng>>,
  // Area this simulation occurs in
  pub stage : Box<dyn Stage>,
  pub food_per_generation : u32,
  pub generations : Vec<Generation>,
  pub behaviours : Vec<Box<dyn StepBehaviour>>
}

// Starting point for creating simulations
impl Simulation {
  pub fn new( stage : Box<dyn Stage>, seed: u64, food_per_generation: u32 ) -> Self {

    let generations = Vec::new();

    Simulation {
      stage,
      generations,
      food_per_generation,
      behaviours : vec![Box::new(ResetBehaviour)],
      // prepare a deterministic generator:
      rng: Rc::new(RefCell::new(SmallRng::seed_from_u64(seed)))
    }
  }

  pub fn add_behavour(&mut self, b : Box<StepBehaviour>){
    self.behaviours.push(b)
  }

  pub fn run(&mut self, creature_count: u32, max_generations : u32){
    let food_locations = self.generate_food();
    let mut generation = Generation::new(self, food_locations, creature_count);
    let mut keep_going = generation.has_living_creatures();

    for _gen in 1..max_generations {
      if !keep_going { break; }

      let food_locations = self.generate_food();
      let next = Generation::from_previous( &generation, food_locations );
      self.generations.push(generation);
      generation = next;
      keep_going = generation.has_living_creatures();
    }

    self.generations.push(generation);
  }

  pub fn get_random_location(&self) -> Point2<f64> {
    self.stage.get_random_location(&mut self.rng.borrow_mut())
  }

  pub fn get_random_float(&self, from : f64, to : f64) -> f64 {
    let mut rng = self.rng.borrow_mut();
    rng.gen_range(from, to)
  }

  pub fn generate_food(&self) -> Vec<Point2<f64>> {
    (0..self.food_per_generation).map(|_n|
      self.get_random_location()
    ).collect()
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FoodStatus {
  Available,
  Eaten(usize), // step the food was eaten at
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Food {
  position: Point2<f64>,
  status: FoodStatus,
}

// Each generation of the simulation. A collection of creatures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generation {
  pub steps : usize, // total steps this generation took to complete
  pub creatures : Vec<Creature>,
  pub food : Vec<Food>, // tuple showing the step the food was eaten
}

impl Generation {
  pub fn new( sim : &mut Simulation, food_locations: Vec<Point2<f64>>, creature_count : u32 ) -> Self {

    let creatures = (0..creature_count).map(|_n| {
      // random creature starting position
      // TODO: original started creatures on edges
      let pos = sim.get_random_location();

      Creature::new( &pos )
    }).collect();

    let food = food_locations.iter().map(|p| {
      Food {
        position: *p,
        status: FoodStatus::Available,
      }
    }).collect();

    let mut gen = Generation {
      creatures,
      food,
      steps: 0,
    };

    gen.run_phase(Phase::INIT, sim);

    gen.step_to_completion(sim);

    gen
  }

  pub fn from_previous( previous : &Generation, food_locations: Vec<Point2<f64>> ) -> Self {
    unimplemented!()
  }

  pub fn has_living_creatures(&self) -> bool {
    self.creatures.iter().any(|c| c.is_alive())
  }

  pub fn has_active_creatures(&self) -> bool {
    self.creatures.iter().any(|c| c.is_active())
  }

  pub fn get_available_food(&self) -> Vec<Food> {
    self.food.iter().filter(|f| {
      if let FoodStatus::Available = f.status { true } else { false }
    }).map(|f| f.clone()).collect()
  }

  pub fn mark_food_eaten(&mut self, food : &Food){
    if let Some(index) = self.food.iter().position(|f| *f == *food) {
      self.food[index].status = FoodStatus::Eaten(self.steps);
    }
  }

  // advance the generation to its end
  fn step_to_completion(&mut self, sim : &mut Simulation) {
    // break when all asleep or dead
    while self.has_living_creatures() {
      self.step(sim);
    }
  }

  fn run_phase(&mut self, phase : Phase, sim : &Simulation){
    sim.behaviours.iter().for_each(
      |b| b.apply(phase, self, &sim)
    );
  }

  fn step(&mut self, sim : &mut Simulation){
    assert!(self.steps < MAX_STEPS);

    self.run_phase(Phase::PRE, sim);
    self.run_phase(Phase::ORIENT, sim);
    self.run_phase(Phase::MOVE, sim);
    self.run_phase(Phase::ACT, sim);
    self.run_phase(Phase::POST, sim);

    self.steps += 1;
  }
}
