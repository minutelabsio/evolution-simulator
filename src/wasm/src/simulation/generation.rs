use na::Point2;
use super::{Simulation, Creature, Food, FoodStatus, behaviours::Phase};

pub type Step = usize;

// just to prevent infinite loops
const MAX_STEPS : usize = 1_000_000;

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
