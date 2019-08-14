use crate::na::{Point2, Unit, Vector2};
use super::world::{
  Phase,
};
use super::evolving_trait::EvolvingTrait;

const MOTION_ENERGY_COST : f64 : 0.1;

enum CreatureState {
  DEAD,
  ASLEEP,
  ACTIVE,
}

pub struct Creature {
  pub will_reproduce : bool,
  pub pos : Point2<f64>,
  pub start_pos : Point2<f64>,
  pub home_pos : Point2<f64>,
  pub energy : f64,

  // array of displacement vectors
  pub movement_history : Vec<Vector2<f64>>,

  pub traits : Vec<Box<EvolvingTrait>>,
  state : CreatureState,
}

impl Creature {
  pub fn new( x : f64, y : f64 ) -> Self {
    Creature {
      state: CreatureState::ACTIVE,
      energy: 1.0,

      will_reproduce: false,
      pos: Point2::new( x, y ),
      start_pos : Point2::new( x, y ),
      home_pos: Point2::new( x, y ),
      movement_history: Vec::new(),

      traits: Vec::new(),
    }
  }

  // Instance methods
  //------------------
  pub fn is_alive(&self) -> bool {
    match self.state {
      CreatureState::DEAD => false,
      _ => true,
    }
  }

  pub fn is_active(&self) -> bool {
    match self.state {
      CreatureState::ACTIVE => true,
      _ => false,
    }
  }

  // move the creature, record its motion in history,
  // apply an energy cost.
  pub fn move_to( &mut self, pos : &Point2<f64> ){
    let disp = pos - self.pos;
    self.apply_energy_cost( MOTION_ENERGY_COST * disp.norm() );

    self.pos = pos;
    movement_history.push(disp);
  }

  pub fn get_direction(&self) -> Unit<Vector2<f64>> {
    Unit::new_normalize(Vector2::x())
  }

  pub fn apply_phase( &mut self, phase : &Phase ){
    // do nothing if it's dead
    if !self.is_alive() { return; }

    let traits = std::mem::replace(&mut self.traits, vec![]);

    let mut iter = traits.iter();
    while let Some(t) = iter.next() {
      t.apply( phase, self );
    };

    std::mem::replace(&mut self.traits, traits);
  }

  // get the position of this creature at time
  pub fn get_position( &self ) -> Point2<f64> {
    self.pos
  }

  pub fn hunger_fulfillment_value( &self, predator : &Creature ) -> f64 {
    self.traits.iter()
      .filter_map(|t| t.consumption_modifier(predator))
      .nth(0)
      .unwrap()
  }

  // can this creature eat that other creature
  pub fn can_eat( &self, other : &Creature ) -> bool {
    other.hunger_fulfillment_value( self ) > 0.
  }

  pub fn apply_energy_cost( &mut self, cost : f64 ){
    self.energy -= cost;

    if self.energy <= 0. {
      self.state = CreatureState::DEAD;
    }
  }
}
