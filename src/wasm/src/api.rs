use wasm_bindgen::prelude::*;
use super::*;
use simulation::*;
use creature::*;

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
      "StarveBehaviour" => Ok(Box::new(behaviours::StarveBehaviour)),
      "OldAgeBehaviour" => Ok(Box::new(behaviours::OldAgeBehaviour)),
      _ => Err(format!("Behaviour {} is not defined", self.name).into())
    }
  }
}

#[derive(Deserialize)]
pub struct SimulationConfig {
  pub size: f64,
  pub seed: u64,
  pub max_generations: u32,
  pub food_per_generation: u32,
  pub behaviours: Vec<BehaviourConfig>,
}

#[derive(Deserialize)]
pub struct RandomCreatureConfig {
  pub count : usize,
  pub template : Creature,
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

fn parse_creatures( creatures : Vec<JsValue> ) -> Result<Vec<Creature>, JsValue> {
  let mut results = vec![];
  for c in creatures {
    results.push(c.into_serde().map_err(|_| "Problem parsing creature json")?);
  }

  Ok(results)
}

// randomize creature starting points
#[wasm_bindgen]
pub fn init_random_creatures( cfg : &JsValue, creature_cfg_raw : &JsValue ) -> Result<Vec<JsValue>, JsValue> {
  let mut sim_cfg : SimulationConfig = cfg.into_serde().map_err(|_| "Problem parsing json")?;
  let creature_cfg : RandomCreatureConfig = creature_cfg_raw.into_serde().map_err(|_| "Problem parsing json")?;

  sim_cfg.seed += 1;
  let mut sim = create_simulation( &sim_cfg )?;

  // randomize creature locations
  let mut creatures = Vec::with_capacity( creature_cfg.count );
  for i in 0..creature_cfg.count {
    let pos = sim.get_random_location();
    let c = creature_cfg.template.with_new_position(&pos);
    creatures.push(c);
  }

  let ret = creatures.iter().map(|c| JsValue::from_serde(&c).unwrap()).collect();
  Ok(ret)
}


// run a simulation
#[wasm_bindgen]
pub fn run_simulation( cfg : &JsValue, creature_cfg: Vec<JsValue> ) -> Result<JsValue, JsValue> {
  let sim_cfg : SimulationConfig = cfg.into_serde().map_err(|_| "Problem parsing json")?;
  let creatures = parse_creatures( creature_cfg )?;
  let mut sim = create_simulation( &sim_cfg )?;

  sim.run(creatures, sim_cfg.max_generations);

  let results : SimulationResults = sim.into();
  Ok(JsValue::from_serde(&results).unwrap())
}
