use crate::na::{Point2, Unit, Vector2};
use super::simulation::{
  Phase,
  Generation,
};

const MOTION_ENERGY_COST : f64 = 0.1;

#[derive(Debug, Copy, Clone)]
enum CreatureState {
  DEAD,
  ASLEEP,
  ACTIVE,
}

#[derive(Debug, Clone)]
pub struct Creature {
  // mutatable
  pub speed : f64,

  // other
  pub energy : f64,
  pub will_reproduce : bool,
  pub pos : Point2<f64>,
  pub start_pos : Point2<f64>,
  pub home_pos : Point2<f64>,

  // array of displacement vectors
  pub movement_history : Vec<Vector2<f64>>,

  state : CreatureState,
}

impl Creature {
  pub fn new( pos : &Point2<f64> ) -> Self {
    Creature {
      state: CreatureState::ACTIVE,
      speed: 1.0,
      energy: 1.0,

      will_reproduce: false,
      pos: pos.clone(),
      start_pos : pos.clone(),
      home_pos: pos.clone(),
      movement_history: Vec::new(),
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
    self.pos = pos.clone();
    self.movement_history.push(disp);
  }

  pub fn get_direction(&self) -> Unit<Vector2<f64>> {
    Unit::new_normalize(Vector2::x())
  }

  // get the position of this creature at time
  pub fn get_position( &self ) -> Point2<f64> {
    self.pos
  }

  pub fn apply_energy_cost( &mut self, cost : f64 ){
    self.energy -= cost;

    if self.energy <= 0. {
      self.state = CreatureState::DEAD;
    }
  }
}
