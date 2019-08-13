use crate::creature::*;
use crate::na::{Vector2};

pub enum WorldObjectType {
  Food,
  Creature,
}

pub trait WorldObject {
  fn get_type(&self) -> WorldObjectType;
  fn get_position(&self) -> Vector2<f64>;
  fn can_eat(&self, other : &WorldObject) -> bool;
  fn hunger_fulfillment_value(&self, predator : &Creature) -> f64;
  fn apply_trigger( &mut self, trigger : &Trigger );
}

pub enum Trigger<'a> {
  Tick,
  Interact(&'a dyn WorldObject),
}


pub struct World<'a> {
  // how big is it in one dimension
  pub size: i32,

  objects: Vec<&'a WorldObject>,
}

impl<'a> World<'a> {
  pub fn new( size: i32 ) -> Self {

    let objects = Vec::new();

    World {
      size,
      objects,
    }
  }


  fn next_tick( &self, creature: &mut Creature ){
    // tick phase
    creature.apply_trigger( &Trigger::Tick );

    // check interaction with other objects
    self.objects.iter().for_each(|other| {
      creature.apply_trigger( &Trigger::Interact(other) )
    });
  }
}
