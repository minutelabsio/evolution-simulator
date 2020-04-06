use super::*;

// Old age behavour. chance of death if too old
const AGE_LIMIT_VARIANCE : f64 = 1.0;
#[derive(Debug, Copy, Clone)]
pub struct OldAgeBehaviour;
impl OldAgeBehaviour {
  fn check_old_age(&self, creature : &mut Creature, sim : &Simulation){
    let lifetime = sim.get_random_gaussian(creature.get_life_span(), AGE_LIMIT_VARIANCE);
    if (creature.age as f64) > lifetime {
      creature.kill();
    }
  }
}

impl StepBehaviour for OldAgeBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, sim : &Simulation){
    if let Phase::INIT = phase {
      generation.creatures.iter_mut()
        .filter(|c| c.is_alive())
        .for_each(|c| self.check_old_age(c, sim));
    }
  }
}
