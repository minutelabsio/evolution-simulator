use wasm_bindgen::prelude::*;
use super::*;
use simulation::*;
use creature::*;

// #[derive(Deserialize)]
// pub struct BehaviourConfig {
//   pub name: String,
// }
//
// impl BehaviourConfig {
//   pub fn parse(&self) -> Result<Box<dyn StepBehaviour>, JsValue> {
//     match self.name.as_ref() {
//       "BasicMoveBehaviour" => Ok(Box::new(behaviours::BasicMoveBehaviour)),
//       "WanderBehaviour" => Ok(Box::new(behaviours::WanderBehaviour)),
//       "ScavengeBehaviour" => Ok(Box::new(behaviours::ScavengeBehaviour)),
//       "HomesickBehaviour" => Ok(Box::new(behaviours::HomesickBehaviour)),
//       "StarveBehaviour" => Ok(Box::new(behaviours::StarveBehaviour)),
//       "OldAgeBehaviour" => Ok(Box::new(behaviours::OldAgeBehaviour)),
//       _ => Err(format!("Behaviour {} is not defined", self.name).into())
//     }
//   }
// }

#[derive(Serialize, Deserialize)]
pub struct PresetConfig {
  name: String,
  foods_before_home: i32,
}

#[derive(Deserialize)]
pub struct SimulationConfig {
  pub size: f64,
  pub seed: u64,
  pub max_generations: u32,
  pub food_per_generation: u32,
  pub preset: PresetConfig,
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

fn use_preset( sim : &mut Simulation, preset : &PresetConfig ){
  match preset.name {
    _ => {
      // default
      sim.add_behavour(Box::new(behaviours::BasicMoveBehaviour));
      sim.add_behavour(Box::new(behaviours::WanderBehaviour));
      sim.add_behavour(Box::new(behaviours::ScavengeBehaviour));
      sim.add_behavour(Box::new(behaviours::SatisfiedBehaviour));
      sim.add_behavour(Box::new(behaviours::EdgeHomeBehaviour));
      sim.add_behavour(Box::new(behaviours::StarveBehaviour));
    }
  }
}

fn create_simulation( sim_cfg : &SimulationConfig ) -> Result<Simulation, JsValue> {
  let stage = Box::new(stage::SquareStage(sim_cfg.size));

  let mut sim = Simulation::new(stage, sim_cfg.seed, sim_cfg.food_per_generation);

  use_preset(&mut sim, &sim_cfg.preset);

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
// #[wasm_bindgen]
// pub fn init_random_creatures( cfg : &JsValue, creature_cfg_raw : &JsValue ) -> Result<Vec<JsValue>, JsValue> {
//   let mut sim_cfg : SimulationConfig = cfg.into_serde().map_err(|_| "Problem parsing json")?;
//   let creature_cfg : RandomCreatureConfig = creature_cfg_raw.into_serde().map_err(|_| "Problem parsing json")?;
//
//   sim_cfg.seed += 1;
//   let sim = create_simulation( &sim_cfg )?;
//
//   // randomize creature locations
//   let mut creatures = Vec::with_capacity( creature_cfg.count );
//   for _i in 0..creature_cfg.count {
//     let pos = sim.get_random_location();
//     let c = creature_cfg.template.with_new_position(&pos);
//     creatures.push(c);
//   }
//
//   let ret = creatures.iter().map(|c| JsValue::from_serde(&c).unwrap()).collect();
//   Ok(ret)
// }


// run a simulation
#[wasm_bindgen]
pub fn run_simulation( cfg : &JsValue, random_creature_cfg: &JsValue ) -> Result<JsValue, JsValue> {
  let sim_cfg : SimulationConfig = cfg.into_serde().map_err(|_| "Problem parsing json")?;
  let creature_cfg : RandomCreatureConfig = random_creature_cfg.into_serde().map_err(|_| "Problem parsing json")?;

  let mut sim = create_simulation( &sim_cfg )?;

  // randomize creature locations
  let mut creatures = Vec::with_capacity( creature_cfg.count );
  for _i in 0..creature_cfg.count {
    let pos = sim.stage.get_nearest_edge_point(&sim.get_random_location());
    let c = creature_cfg.template.with_new_position(&pos);
    creatures.push(c);
  }

  sim.run(creatures, sim_cfg.max_generations);

  let results : SimulationResults = sim.into();
  Ok(JsValue::from_serde(&results).unwrap())
}

// continue a simulation
#[wasm_bindgen]
pub fn continue_simulation( cfg : &JsValue, creature_cfg: Vec<JsValue> ) -> Result<JsValue, JsValue> {
  let sim_cfg : SimulationConfig = cfg.into_serde().map_err(|_| "Problem parsing json")?;
  let creatures = parse_creatures( creature_cfg )?;
  let mut sim = create_simulation( &sim_cfg )?;

  sim.run(creatures, sim_cfg.max_generations);

  let results : SimulationResults = sim.into();
  Ok(JsValue::from_serde(&results).unwrap())
}
