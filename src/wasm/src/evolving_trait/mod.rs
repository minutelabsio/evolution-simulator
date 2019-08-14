use crate::na::{distance};
use super::world::{Phase};
use super::creature::Creature;
// speed, sense, size

pub trait EvolvingTrait {
  fn apply( &self, phase : &Phase, creature : &mut Creature );
  fn consumption_modifier(&self, predator: &Creature) -> Option<f64>;
}

// Speed
// -----
#[derive(Debug, Copy, Clone)]
struct Speed(f64);

impl EvolvingTrait for Speed {
  fn apply( &self, phase : &Phase, creature : &mut Creature ){
    let speed = self.0;

    if let Phase::Tick = *phase {
      let new_pos = creature.get_position() + speed * creature.get_direction().as_ref();
      creature.move_to( &new_pos );
    }
  }

  fn consumption_modifier(&self, _c : &Creature) -> Option<f64> { None }
}


// Sense
// -----
#[derive(Debug, Copy, Clone)]
struct Sense(f64);

impl EvolvingTrait for Sense {
  fn apply( &self, phase : &Phase, creature : &mut Creature ){
    let sense = self.0;
    if let Phase::Interact(other) = *phase {
      let pos = creature.get_position();

      if distance(&pos, &other.get_position()) <= sense {
        if creature.can_eat(&*other) {

          // TODO
        }
      }
    }
  }

  fn consumption_modifier(&self, _c : &Creature) -> Option<f64> { None }
}
