use crate::na::{Point2};
use std::cell::{RefMut};
use rand::{Rng, rngs::SmallRng};
use super::creature::*;
// The stage defines the borders of the simulation
// It's the area the creatures can evolve inside

pub struct Edge(pub Point2<f64>, pub Point2<f64>);

pub trait Stage {
  // counter clockwise
  fn get_edges(&self) -> Vec<Edge>;
  // can this creature move here?
  fn can_move_to(&self, to : &Point2<f64>, creature : &Creature ) -> bool;
  fn get_center(&self) -> Point2<f64>;
  // generate a location from a u64 seed. used to randomly place food within boundaries
  fn get_random_location(&self, rng : &mut RefMut<SmallRng>) -> Point2<f64>;
  fn get_nearest_edge_point(&self, pos : &Point2<f64>) -> Point2<f64>;
  fn constrain_within(&self, pos : &Point2<f64>) -> Point2<f64>;
}


// simple square
pub struct SquareStage(pub f64);

impl Stage for SquareStage {

  fn get_edges(&self) -> Vec<Edge> {
    vec![
      Edge(Point2::new(0., 0.), Point2::new(self.0, 0.)),
      Edge(Point2::new(self.0, 0.), Point2::new(self.0, self.0)),
      Edge(Point2::new(self.0, self.0), Point2::new(0., self.0)),
      Edge(Point2::new(0., self.0), Point2::new(0., 0.)),
    ]
  }

  fn can_move_to(&self, to : &Point2<f64>, _creature : &Creature ) -> bool {
    to.x >= 0. &&
    to.y >= 0. &&
    to.x <= self.0 &&
    to.y <= self.0
  }

  fn get_center(&self) -> Point2<f64> { 0.5 * Point2::new(self.0, self.0) }

  fn get_random_location(&self, rng : &mut RefMut<SmallRng>) -> Point2<f64> {
    let x = rng.gen_range(0., self.0);
    let y = rng.gen_range(0., self.0);

    Point2::new(x, y)
  }

  fn get_nearest_edge_point(&self, pos : &Point2<f64>) -> Point2<f64> {
    let hw = 0.5 * self.0;
    let x = if pos.x > hw { self.0 } else { 0. };
    let y = if pos.y > hw { self.0 } else { 0. };

    if (x - pos.x).abs() < (y - pos.y).abs() {
      Point2::new(x, pos.y)
    } else {
      Point2::new(pos.x, y)
    }
  }

  fn constrain_within(&self, pos : &Point2<f64>) -> Point2<f64> {
    let mut ret = pos.clone();
    ret.x = ret.x.max(0.).min(self.0);
    ret.y = ret.y.max(0.).min(self.0);
    ret
  }
}
