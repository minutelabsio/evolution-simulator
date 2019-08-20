use crate::na::{Point2};
use rand::{SeedableRng, Rng, rngs::SmallRng};
use super::creature::*;
// The stage defines the borders of the simulation
// It's the area the creatures can evolve inside

pub trait Stage {
  // can this creature move here?
  fn can_move_to(&self, to : &Point2<f64>, creature : &Creature ) -> bool;
  // generate a location from a u64 seed. used to randomly place food within boundaries
  fn get_random_location(&self, rng : &mut SmallRng) -> Point2<f64>;
}


// simple square
pub struct SquareStage(pub f64);

impl Stage for SquareStage {
  fn can_move_to(&self, to : &Point2<f64>, creature : &Creature ) -> bool {
    to.x >= 0. &&
    to.y >= 0. &&
    to.x < self.0 &&
    to.y < self.0
  }

  fn get_random_location(&self, rng : &mut SmallRng) -> Point2<f64> {
    let x = rng.gen_range(0., self.0);
    let y = rng.gen_range(0., self.0);

    Point2::new(x, y)
  }
}
