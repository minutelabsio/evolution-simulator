use super::*;
use std::f64::consts::FRAC_PI_4;

pub trait StepBehaviour {
  // if you need &mut self, use Cell or RefCell
  fn apply(&self, phase : Phase, generation : &mut Generation, sim : &Simulation);
}

pub trait ReproductionBehaviour {
  fn reproduce(&self, creatures : &Vec<Creature>, sim : &Simulation) -> Vec<Creature>;
}

mod reproduction;
pub use reproduction::*;

mod wander;
pub use wander::*;

mod homesick;
pub use homesick::*;

mod satisfied;
pub use satisfied::*;

// move is a reserved word
mod r#move;
pub use r#move::*;

mod scavenge;
pub use scavenge::*;

mod starve;
pub use starve::*;

mod old_age;
pub use old_age::*;

mod edge_home;
pub use edge_home::*;

mod cannibalism;
pub use cannibalism::*;

// doesn't work the way i wanted it to
// mod event;
// pub use event::*;
