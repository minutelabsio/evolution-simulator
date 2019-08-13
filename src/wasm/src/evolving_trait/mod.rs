use super::world::{WorldObject, Trigger};
use super::creature::Creature;
// speed, sense, size

pub trait EvolvingTrait<'a> {
  fn apply( &self, trigger : &Trigger, creature : &mut Creature );
  fn consumption_modifier(&self) -> Option<&'a Fn(&Creature) -> f64>;
}

// Speed
// -----
struct Speed(f64);

impl<'a> EvolvingTrait<'a> for Speed {
  fn apply( &self, trigger : &Trigger, creature : &mut Creature ){
    let speed = self.0;

    if let Trigger::Tick = *trigger {
      let new_pos = creature.last_pos() + speed * creature.direction();
      creature.move_to( &new_pos );
    }
  }

  fn consumption_modifier(&self) -> Option<&'a Fn(&Creature) -> f64> {
    None
  }
}


// Sense
// -----
struct Sense(f64);

impl<'a> EvolvingTrait<'a> for Sense {
  fn apply( &self, trigger : &Trigger, creature : &mut Creature ){
    let sense = self.0;
    if let Trigger::Interact(other) = *trigger {
      let pos = creature.get_position();

      if pos.distance(other.get_position()) <= sense {
        if creature.can_eat(other) {
          other.is_dead = true;
          // TODO
        }
      }
    }
  }

  fn consumption_modifier(&self) -> Option<&'a Fn(&Creature) -> f64> {
    None
  }
}
