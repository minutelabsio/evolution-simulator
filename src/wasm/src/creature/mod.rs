use crate::na::{Point2, Unit, Vector2};
use std::cell::{RefMut};
use rand::{rngs::SmallRng};

mod mutatable;
use mutatable::*;

// r1, r2. Points defining line segment
// https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Vector_formulation
fn distance_to_line(r1 : &Point2<f64>, r2 : &Point2<f64>, p : &Point2<f64>) -> Option<f64> {
  let a = r1;
  let v = r2 - r1;
  let n = v.normalize();
  // vector from the point to first point on line
  let pa = a - p;
  let pb = r2 - p;
  // this means we're outside the line segment
  if pa.dot(&pb) > 0. {
    return None
  }

  // projection of pa onto the line
  let z = pa.dot(&n) * n;

  let d = pa - z;
  Some(d.norm())
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
enum CreatureState {
  DEAD,
  ASLEEP,
  ACTIVE,
}

// automatically ordered top to bottom
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq, PartialOrd)]
pub enum ObjectiveIntensity {
  // Meh level
  MinorCraving,
  MinorAversion,
  // Kind of want this
  ModerateCraving,
  ModerateAversion,
  // Seriously starving
  MajorCraving,
  MajorAversion,
  // Will die unless this happens
  VitalCraving,
  VitalAversion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creature {
  // mutatable
  speed : PositiveMutatable<f64>, // how far can it move in one step?
  sense_range : PositiveMutatable<f64>, // how far can it see?
  reach : PositiveMutatable<f64>, // how far can it interact with something?
  life_span: PositiveMutatable<f64>,

  // other
  pub foods_eaten : u32,
  pub energy : f64,
  pub energy_consumed: f64,
  pub age : u32,
  pub pos : Point2<f64>,
  pub home_pos : Point2<f64>,

  // array of position vectors
  pub movement_history : Vec<Point2<f64>>,

  state : CreatureState,
  // current target of the creature's desire
  // and its weight
  target: Option<(Point2<f64>, ObjectiveIntensity)>
}

impl Creature {
  pub fn new( pos : &Point2<f64> ) -> Self {
    Creature {
      state: CreatureState::ACTIVE,
      speed: PositiveMutatable(5.0, 0.1),
      sense_range: PositiveMutatable(50.0, 10.0),
      reach: PositiveMutatable(5.0, 1.0),
      life_span: PositiveMutatable(4.0, 1.0),
      energy: 500.0,

      foods_eaten: 0,
      energy_consumed: 0.0,

      age: 0,

      pos: pos.clone(),
      home_pos: pos.clone(),
      movement_history: vec![pos.clone()],
      target: None,
    }
  }

  // Instance methods
  //------------------
  pub fn with_new_position(&self, pos : &Point2<f64>) -> Self {
    let mut ret = self.clone();
    ret.pos = pos.clone();
    ret.home_pos = pos.clone();
    ret.movement_history = vec![pos.clone()];

    ret
  }

  // mutate the creature properties and return a new instance
  pub fn mutate(&self, rng : &mut RefMut<SmallRng>) -> Self {
    Creature {
      speed: self.speed.get_mutated(rng),
      sense_range: self.sense_range.get_mutated(rng),
      reach: self.reach.get_mutated(rng),
      life_span: self.life_span.get_mutated(rng),
      energy: self.energy,

      ..Creature::new(&self.home_pos)
    }
  }

  // copy self, but increase age.
  pub fn grow_older(&self) -> Self {
    let Creature {
      speed,
      sense_range,
      reach,
      life_span,
      energy,
      ..
    } = *self;

    Creature {
      speed,
      sense_range,
      reach,
      life_span,
      energy,
      age: self.age + 1,

      ..Creature::new(&self.home_pos)
    }
  }

  pub fn get_speed(&self) -> f64 { self.speed.0 }
  pub fn get_sense_range(&self) -> f64 { self.sense_range.0 }
  pub fn get_reach(&self) -> f64 { self.reach.0 }
  pub fn get_life_span(&self) -> f64 { self.life_span.0 }

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
  pub fn move_to( &mut self, pos : Point2<f64> ){
    self.pos = pos.clone();
    self.movement_history.push(pos);

    // // energy cost
    // let last = self.get_last_position().expect("Can not get last position.");
    // let displacement = self.pos - last;
    // the cost of moving
    let cost = self.get_motion_energy_cost();
    self.apply_energy_cost( cost );
  }

  pub fn get_motion_energy_cost(&self) -> f64 {
    0.5 * self.get_speed().powi(2) + 0.5
  }

  pub fn get_direction(&self) -> Unit<Vector2<f64>> {
    // displacement vector to target
    let disp = self.target.map(|t| {
      let d = t.0 - self.pos;
      match t.1 {
        ObjectiveIntensity::MinorAversion|
        ObjectiveIntensity::ModerateAversion|
        ObjectiveIntensity::MajorAversion|
        ObjectiveIntensity::VitalAversion => -1. * d, // other way
        _ => d,
      }
    }).filter(|d| d.norm() != 0.).unwrap_or_else(|| {
      // or the direction it was traveling before
      self.get_last_position()
        .map(|last| self.pos - last)
        .unwrap_or_else(|| Vector2::x()) // or the x axis
    });

    Unit::new_normalize(disp)
  }

  pub fn add_objective(&mut self, target_pos : Point2<f64>, intensity : ObjectiveIntensity){
    if self.target.map(|t| intensity > t.1).unwrap_or(true) {
      self.target = Some((target_pos, intensity));
    }
  }

  pub fn reset_objective(&mut self){
    self.target = None;
  }

  pub fn eat_food(&mut self){
    self.foods_eaten += 1;
  }

  pub fn sleep(&mut self){
    self.state = CreatureState::ASLEEP;
  }

  pub fn kill(&mut self){
    self.state = CreatureState::DEAD;
  }

  // get the position of this creature at time
  pub fn get_position( &self ) -> Point2<f64> {
    self.pos
  }

  pub fn get_last_position( &self ) -> Option<Point2<f64>> {
    let len = self.movement_history.len();
    if len <= 1 { return None; }

    Some(self.movement_history[len - 2])
  }

  pub fn can_see(&self, pt : &Point2<f64>) -> bool {
    (pt - self.pos).norm() <= self.get_sense_range()
  }

  pub fn can_reach(&self, pt : &Point2<f64>) -> bool {
    if self.can_reach_now(&pt) { return true; }

    let last = self.get_last_position();
    if last.is_none() { return false; }

    distance_to_line(&self.get_position(), &last.unwrap(), pt)
      .map(|d| d <= self.get_reach())
      .unwrap_or(false)
  }

  pub fn can_reach_now(&self, pt : &Point2<f64>) -> bool {
    (pt - self.pos).norm() <= self.get_reach()
  }

  pub fn get_energy_left(&self) -> f64 {
    (self.energy - self.energy_consumed).max(0.)
  }

  pub fn apply_energy_cost( &mut self, cost : f64 ){
    self.energy_consumed += cost;

    if self.get_energy_left() <= 0. {
      self.state = CreatureState::DEAD;
    }
  }
}
