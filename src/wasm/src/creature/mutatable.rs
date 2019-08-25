use rand::distributions::{Normal, Distribution};
use std::cell::{RefMut};
use rand::{rngs::SmallRng};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Mutatable<T>(pub T, pub f64);
impl<T> Mutatable<T>
where T: std::convert::From<f64>, f64: std::convert::From<T> {
  pub fn get_mutated(self, rng : &mut RefMut<SmallRng>) -> Self {
    let normal = Normal::new(self.0.into(), self.1);
    let v = normal.sample(&mut **rng);

    Self(v.into(), self.1)
  }
}
