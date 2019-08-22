use wasm_bindgen::prelude::*;
use super::*;
use simulation::*;

#[derive(Deserialize)]
pub struct BehaviourConfig {
  pub name: String,
}

impl BehaviourConfig {
  pub fn parse(&self) -> Result<Box<dyn StepBehaviour>, JsValue> {
    match self.name.as_ref() {
      "BasicMoveBehaviour" => Ok(Box::new(behaviours::BasicMoveBehaviour)),
      "WanderBehaviour" => Ok(Box::new(behaviours::WanderBehaviour)),
      "ScavengeBehaviour" => Ok(Box::new(behaviours::ScavengeBehaviour)),
      "HomesickBehaviour" => Ok(Box::new(behaviours::HomesickBehaviour)),
      _ => Err(format!("Behaviour {} is not defined", self.name).into())
    }
  }
}

#[derive(Deserialize)]
pub struct SimulationConfig {
  pub size: f64,
  pub seed: u64,
  pub creature_count: u32,
  pub max_generations: u32,
  pub food_per_generation: u32,
  pub behaviours: Vec<BehaviourConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct SimulationResults {
  generations: Vec<Generation>
}

impl From<Simulation> for SimulationResults {
  fn from(sim : Simulation) -> Self {
    SimulationResults {
      generations: sim.generations.clone()
    }
  }
}

fn create_simulation( sim_cfg : &SimulationConfig ) -> Result<Simulation, JsValue> {
  let stage = Box::new(stage::SquareStage(sim_cfg.size));

  let mut sim = Simulation::new(stage, sim_cfg.seed, sim_cfg.food_per_generation);

  for cfg in &sim_cfg.behaviours {
    let b = cfg.parse()?;
    sim.add_behavour(b);
  }

  Ok(sim)
}

// run a simulation
#[wasm_bindgen]
pub fn run_simulation( cfg : &JsValue ) -> Result<JsValue, JsValue> {
  let sim_cfg : SimulationConfig = cfg.into_serde().map_err(|_| "Problem parsing json")?;
  let mut sim = create_simulation( &sim_cfg )?;

  sim.run(sim_cfg.creature_count, sim_cfg.max_generations);

  let results : SimulationResults = sim.into();
  Ok(JsValue::from_serde(&results).unwrap())
}
