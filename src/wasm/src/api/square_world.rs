use super::*;

#[wasm_bindgen]
pub struct SquareWorld {
  sim: Simulation,
  starting_creatures: usize,
  creature_template: Creature,
}

#[wasm_bindgen]
impl SquareWorld {
  pub fn new(size : f64, seed : u32, food_per_generation : u32, creature_cfg: &JsValue, preset_cfg : &JsValue) -> Result<SquareWorld, JsValue> {
    let stage = Box::new(stage::SquareStage(size));

    let preset_cfg = preset_cfg.into_serde().map_err(|e| e.to_string())?;
    let creature_cfg : RandomCreatureConfig = creature_cfg.into_serde().map_err(|e| e.to_string())?;

    let mut sim = Simulation::new(stage, seed as u64, food_per_generation);
    use_preset(&mut sim, &preset_cfg);

    Ok(Self {
      sim,
      starting_creatures: creature_cfg.count,
      creature_template: creature_cfg.template,
    })
  }

  pub fn run(&mut self, max_generations_to_run : u32) {
    let last_gen = self.sim.generations.last();
    let creatures = last_gen.map(|g|
      // mutate last generation if already started
      self.sim.exec_reproduction(&g.creatures)
    ).unwrap_or_else(|| {
      // otherwise create new creatures array
      let count = self.starting_creatures;
      let mut creatures = Vec::with_capacity( count );

      for _i in 0..count {
        let pos = self.sim.stage.get_nearest_edge_point(&self.sim.get_random_location());
        let c = self.creature_template.with_new_position(&pos);
        creatures.push(c);
      }

      creatures
    });

    self.sim.run(creatures, max_generations_to_run);
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

  pub fn get_statistics(&self) -> JsValue {
    get_statistics(&self.sim)
  }
}
