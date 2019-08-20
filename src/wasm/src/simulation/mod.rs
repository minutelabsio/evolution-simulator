extern crate rand;
use crate::na::{Point2};
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
  fn apply(&self, phase : Phase, generation : &mut Generation, sim : &dyn Stage);
}

pub struct Simulation {
  pub rng : SmallRng,
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
      behaviours : Vec::new(),
      // prepare a deterministic generator:
      rng: SmallRng::seed_from_u64(seed)
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

  pub fn generate_food(&mut self) -> Vec<Point2<f64>> {
    (0..self.food_per_generation).map(|_n|
      self.stage.get_random_location(&mut self.rng)
    ).collect()
  }
}

// Each generation of the simulation. A collection of creatures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generation {
  pub steps : usize, // total steps this generation took to complete
  pub creatures : Vec<Creature>,
  pub food_history : Vec<(usize, Point2<f64>)>, // tuple showing the step the food was eaten
  pub food_locations : Vec<Point2<f64>>,
}

impl Generation {
  pub fn new( sim : &mut Simulation, food_locations: Vec<Point2<f64>>, creature_count : u32 ) -> Self {

    let creatures = (0..creature_count).map(|_n| {
      // random creature starting position
      // TODO: original started creatures on edges
      let pos = (*sim.stage).get_random_location(&mut sim.rng);

      Creature::new( &pos )
    }).collect();

    let mut gen = Generation {
      creatures,
      food_locations,
      food_history: Vec::new(),
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

  // advance the generation to its end
  fn step_to_completion(&mut self, sim : &mut Simulation) {
    // break when all asleep or dead
    while self.has_living_creatures() {
      self.step(sim);
    }
  }

  fn run_phase(&mut self, phase : Phase, sim : &Simulation){
    sim.behaviours.iter().for_each(
      |b| b.apply(phase, self, &*sim.stage)
    );
  }

  fn step(&mut self, sim : &mut Simulation){
    self.steps += 1;

    assert!(self.steps < MAX_STEPS);

    self.run_phase(Phase::PRE, sim);
    self.run_phase(Phase::ORIENT, sim);
    self.run_phase(Phase::MOVE, sim);
    self.run_phase(Phase::ACT, sim);
    self.run_phase(Phase::POST, sim);
  }
}
