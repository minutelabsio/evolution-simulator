use super::*;

#[wasm_bindgen]
pub struct SquareWorld {
  sim: Simulation,
  creatures: Vec<Creature>,
}

#[wasm_bindgen]
impl SquareWorld {
  pub fn new(size : f64, seed : u32, food_per_generation : &JsValue, preset_cfg : &JsValue) -> Result<SquareWorld, JsValue> {
    let stage = Box::new(stage::SquareStage(size));

    let food_per_generation = food_per_generation.into_serde::<Vec<(f64, f64)>>().map_err(|e| e.to_string())?;
    let preset_cfg = preset_cfg.into_serde().map_err(|e| e.to_string())?;

    let mut sim = Simulation::new(stage, seed as u64, Interpolator::new(&food_per_generation));
    use_preset(&mut sim, &preset_cfg);

    Ok(Self {
      sim,
      creatures: vec![],
    })
  }

  pub fn add_creatures(&mut self, creature_cfg : &JsValue) -> Result<(), JsValue> {
    let creature_cfg : RandomCreatureConfig = creature_cfg.into_serde().map_err(|e| e.to_string())?;
    // otherwise create new creatures array
    let count = creature_cfg.count;

    for _i in 0..count {
      let pos = self.sim.stage.get_nearest_edge_point(&self.sim.get_random_location());
      let c = creature_cfg.template.with_new_position(&pos);

      self.creatures.push(c);
    }

    Ok(())
  }

  pub fn run(&mut self, max_generations_to_run : u32) {
    let mut creatures = vec![];
    std::mem::swap(&mut creatures, &mut self.creatures);
    self.sim.run(creatures, max_generations_to_run);
    self.creatures = self.sim.exec_reproduction(&self.sim.generations.last().unwrap().creatures);
  }

  pub fn can_continue(&self) -> bool {
    let l = self.sim.generations.last();
    if l.is_none() {
      true
    } else {
      let l = l.unwrap();
      l.creatures.iter().any(|c| c.is_alive())
    }
  }

  pub fn get_results(&self) -> Result<JsValue, JsValue> {
    let results = SimulationResults {
      generations: self.sim.generations.clone()
    };

    Ok(JsValue::from_serde(&results).unwrap())
  }

  pub fn get_generation(&self, index : usize) -> Result<JsValue, JsValue> {
    let gen = &self.sim.generations[index];

    Ok(JsValue::from_serde(gen).unwrap())
  }

  pub fn get_statistics(&self, species_filter : Option<String>) -> JsValue {
    get_statistics(&self.sim, species_filter)
  }
}
