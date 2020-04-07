use crate::simulation::Edible;
use crate::simulation::Step;
use crate::math::*;
use crate::na::{Point2, Unit, Vector2};
use std::cell::{RefMut};
use rand::{rngs::SmallRng};
use uuid::Uuid;

const ENERGY_COST_SCALE_FACTOR : f64 = 1. / 10_000.;

fn get_uuid() -> Uuid {
  Uuid::new_v4()
}

mod mutatable;
use mutatable::*;

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

// current target of the creature's desire
// and its intensity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objective {
  pub pos : Point2<f64>,
  pub intensity : ObjectiveIntensity,
  pub reason : String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creature {
  pub id : Uuid,
  pub species : String,
  // mutatable
  speed : PositiveNonZeroMutatable<f64>, // how far can it move in one step?
  size : PositiveNonZeroMutatable<f64>,
  sense_range : PositiveMutatable<f64>, // how far can it see?
  reach : PositiveNonZeroMutatable<f64>, // how far can it interact with something?
  flee_distance : PositiveMutatable<f64>,
  life_span: PositiveNonZeroMutatable<f64>,

  // other
  pub foods_eaten : Vec<(Step, Uuid)>,
  pub energy : f64,
  pub energy_consumed: f64,
  pub age : u32,
  pub pos : Point2<f64>,
  pub home_pos : Point2<f64>,

  // array of position vectors
  pub movement_history : Vec<Point2<f64>>,
  pub status_history : Vec<String>,

  state : CreatureState,
  objective: Option<Objective>
}

impl Edible for Creature {
  fn get_edible_id(&self) -> Uuid {
    self.id
  }
}

impl Creature {
  pub fn default( pos : &Point2<f64> ) -> Self {
    Creature {
      id: get_uuid(),
      species: "default".to_string(),
      state: CreatureState::ACTIVE,
      speed: PositiveNonZeroMutatable(1.0, 1.),
      size: PositiveNonZeroMutatable(1.0, 1.),
      sense_range: PositiveMutatable(1.0, 1.0),
      reach: PositiveNonZeroMutatable(1.0, 1.0),
      flee_distance: PositiveMutatable(1.0, 1.0),
      life_span: PositiveNonZeroMutatable(1.0, 1.0),
      energy: 500.0,

      foods_eaten: vec![],
      energy_consumed: 0.0,

      age: 0,

      pos: pos.clone(),
      home_pos: pos.clone(),
      movement_history: vec![pos.clone()],
      status_history: vec![],
      objective: None,
    }
  }

  // Instance methods
  //------------------
  pub fn with_new_position(&self, pos : &Point2<f64>) -> Self {
    let mut ret = self.clone();
    ret.id = get_uuid();
    ret.pos = pos.clone();
    ret.home_pos = pos.clone();
    ret.movement_history = vec![pos.clone()];

    ret
  }

  // mutate the creature properties and return a new instance
  pub fn mutate(&self, rng : &mut RefMut<SmallRng>) -> Self {
    Creature {
      speed: self.speed.get_mutated(rng),
      size: self.size.get_mutated(rng),
      sense_range: self.sense_range.get_mutated(rng),
      reach: self.reach.get_mutated(rng),
      flee_distance: self.flee_distance.get_mutated(rng),
      life_span: self.life_span.get_mutated(rng),
      energy: self.energy,
      species: self.species.clone(),

      ..Creature::default(&self.home_pos)
    }
  }

  // copy self, but increase age.
  pub fn grow_older(&self) -> Self {
    let Creature {
      id,
      speed,
      size,
      sense_range,
      reach,
      flee_distance,
      life_span,
      energy,
      ..
    } = *self;

    Creature {
      id,
      speed,
      size,
      sense_range,
      reach,
      flee_distance,
      life_span,
      energy,
      age: self.age + 1,
      species: self.species.clone(),

      ..Creature::default(&self.home_pos)
    }
  }

  pub fn get_size(&self) -> f64 { self.size.0 }
  pub fn get_speed(&self) -> f64 { self.speed.0 * self.size.0 / 10. }
  pub fn set_speed(&mut self, speed : f64) { self.speed.0 = speed }
  pub fn get_sense_range(&self) -> f64 { self.sense_range.0 }
  pub fn set_sense_range(&mut self, sense : f64) { self.sense_range.0 = sense }
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
    let cost = self.get_motion_energy_cost();
    self.apply_energy_cost( cost );
  }

  pub fn get_motion_energy_cost(&self) -> f64 {
    // primer's energy cost equation
    ENERGY_COST_SCALE_FACTOR *
    (self.get_size().powi(3) * self.get_speed().powi(2) + self.get_sense_range())
  }

  pub fn get_direction(&self) -> Unit<Vector2<f64>> {
    // displacement vector to objective
    let disp = self.objective.as_ref().map(|o| {
      let d = o.pos - self.pos;
      match o.intensity {
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

  pub fn add_objective(&mut self, obj : Objective){
    match &self.objective {
      // if not yet set
      None =>
        self.objective = Some(obj),
      // if the new thing is more intense
      Some(o) if obj.intensity > o.intensity =>
        self.objective = Some(obj),
      // if new thing is same intensity, choose the closer one
      Some(o) if obj.intensity == o.intensity => {
        let pos = self.get_position();
        let dist_to_old = (pos - o.pos).norm();
        let dist_to_new = (pos - obj.pos).norm();

        if dist_to_new > dist_to_old {
          self.objective = Some(obj)
        }
      },
      // stick with what you know
      Some(_) => {}
    };
  }

  pub fn get_objective(&self) -> Option<Objective> {
    self.objective.clone()
  }

  pub fn reset_objective(&mut self){
    self.objective = None;
  }

  pub fn eat_food(&mut self, step: Step, food: &dyn Edible){
    self.foods_eaten.push((step, food.get_edible_id()))
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

  pub fn within_flee_distance(&self, pt : &Point2<f64>) -> bool {
    if self.can_see(pt) {
      let d = (self.get_position() - pt).norm();
      d < self.flee_distance.0
    } else {
      false
    }
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
