use super::*;

const MOTION_ENERGY_COST : f64 = 0.1;

// Basic behaviour for simple movement
#[derive(Debug, Copy, Clone)]
pub struct BasicMoveBehaviour;

impl BasicMoveBehaviour {
  fn move_creature(&self, creature : &mut Creature){
    // move
    let new_pos = creature.get_position() + creature.speed * creature.get_direction().as_ref();
    creature.move_to( &new_pos );
    // energy cost
    let displacement = creature.movement_history[creature.movement_history.len() - 1];
    creature.apply_energy_cost( MOTION_ENERGY_COST * displacement.norm() )
  }
}

impl StepBehaviour for BasicMoveBehaviour {
  fn apply(&mut self, phase : Phase, generation : &mut Generation, stage : &dyn Stage){
    if let Phase::MOVE = phase {
      generation.creatures.iter_mut()
        .filter(|c| c.is_alive())
        .for_each(|c| self.move_creature(c));
    }
  }
}

// // Bigger creatures eat smaller ones
// pub struct CannibalismBehaviour {
//   // controls nutritional value of a grotesque meal
//   pub multiplier: f64,
//   // within this radius, one can eat another
//   pub interaction_range: f64,
// }
//
// impl StepBehaviour for CannibalismBehaviour {
//   fn apply(&mut self, phase : Phase, generation : &mut generation, stage : &Simulation){
//     if let Phase::ACT = phase {
//       generation.creatures.iter().copied()
//         .filter(|c| c.is_alive())
//         .for_each(|c| );
//     }
//   }
// }
