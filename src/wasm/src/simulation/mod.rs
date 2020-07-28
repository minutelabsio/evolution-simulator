extern crate rand;
use crate::na::{Point2};
use std::cell::{RefCell};
use std::rc::Rc;
use rand::{SeedableRng, Rng, rngs::SmallRng};
use rand::distributions::{Normal, Distribution};

use crate::math::Interpolator;
use crate::creature::*;
use crate::stage::*;
// use crate::timer::Timer;

mod edible;
pub use edible::*;
mod food;
pub use food::*;
mod generation;
pub use generation::*;

pub mod behaviours;
use behaviours::Phase;

// behaviour to reset parameters on step
#[derive(Debug, Copy, Clone)]
pub struct ResetBehaviour;
impl behaviours::StepBehaviour for ResetBehaviour {
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
  pub behaviours : Vec<Box<dyn behaviours::StepBehaviour>>,
  pub reproduction_behaviour : Box<dyn behaviours::ReproductionBehaviour>,
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

  pub fn add_behaviour(&mut self, b : Box<dyn behaviours::StepBehaviour>){
    self.behaviours.push(b)
  }

  pub fn set_reproduction_behaviour(&mut self, b : Box<dyn behaviours::ReproductionBehaviour>){
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

