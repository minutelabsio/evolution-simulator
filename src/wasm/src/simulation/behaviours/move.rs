use super::*;

// Basic behaviour for simple movement
#[derive(Debug, Copy, Clone)]
pub struct BasicMoveBehaviour;
impl BasicMoveBehaviour {
  fn move_creature(&self, creature : &mut Creature, stage : &dyn Stage){
    // move
    let pos = creature.get_position();
    let new_pos = pos + creature.get_speed() * creature.get_direction().as_ref();
    let constrained = stage.constrain_within(&new_pos);

    creature.move_to( constrained );
  }
}

impl StepBehaviour for BasicMoveBehaviour {
  fn apply(&self, phase : Phase, generation : &mut Generation, sim : &Simulation){
    if let Phase::MOVE = phase {
      generation.creatures.iter_mut()
        .filter(|c| c.is_active())
        .for_each(|c| self.move_creature(c, &*sim.stage));
    }
  }
}
