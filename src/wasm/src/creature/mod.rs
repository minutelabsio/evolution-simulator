use crate::na::{Vector2};
use super::world::{
  Trigger,
  WorldObjectType,
  WorldObject,
};
use super::evolving_trait::EvolvingTrait;

pub struct Creature<'a> {
  pub is_dead: bool,
  pub will_reproduce: bool,
  pub start_pos : Vector2<f64>,

  pub traits : Vec<&'a EvolvingTrait<'a>>,
}

impl<'a> Creature<'a> {
  pub fn new( x : f64, y : f64 ) -> Self {
    Creature {
      is_dead: false,
      will_reproduce: false,
      start_pos: Vector2::new( x, y ),

      traits: Vec::new(),
    }
  }

  // Instance methods
  //------------------
  pub fn move_to( &self, pos : &Vector2<f64> ){
    unimplemented!();
  }

  pub fn direction(&self) -> Vector2<f64> {
    Vector2::x()
  }

  pub fn last_pos(&self) -> Vector2<f64> {
    self.start_pos
  }
}

impl<'a> WorldObject for Creature<'a> {

  fn get_type( &self ) -> WorldObjectType { WorldObjectType::Creature }

  fn apply_trigger( &mut self, trigger : &Trigger ){
    self.traits.iter().for_each(|t| {
      t.apply( trigger, self );
    });
  }

  // get the position of this creature at time
  fn get_position( &self ) -> Vector2<f64> {
    self.start_pos
  }

  fn hunger_fulfillment_value( &self, predator : &Creature ) -> f64 {
    self.traits.iter()
      .filter_map(|t| t.consumption_modifier())
      .nth(0)
      .map_or(0., |func| func(self))
  }

  fn can_eat( &self, other : &WorldObject ) -> bool {
    other.hunger_fulfillment_value( self ) > 0.
  }
}
