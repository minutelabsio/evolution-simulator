use rand::distributions::{Normal, Distribution};
use std::cell::{RefMut};
use rand::{rngs::SmallRng};

fn mutate(rng : &mut RefMut<SmallRng>, center : f64, variance: f64) -> f64 {
  let normal = Normal::new(center, variance);
  normal.sample(&mut **rng)
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Mutatable<T>(pub T, pub f64);
impl<T> Mutatable<T>
where T: std::convert::From<f64>, f64: std::convert::From<T> {
  #[allow(dead_code)]
  pub fn get_mutated(self, rng : &mut RefMut<SmallRng>) -> Self {
    let v = mutate(rng, self.0.into(), self.1.into());
    Self(v.into(), self.1)
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PositiveMutatable<T>(pub T, pub f64);
impl<T> PositiveMutatable<T>
where T: std::convert::From<f64>, f64: std::convert::From<T> {
  pub fn get_mutated(self, rng : &mut RefMut<SmallRng>) -> Self {
    let v = mutate(rng, self.0.into(), self.1.into()).max(0.);
    Self(v.into(), self.1)
  }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PositiveNonZeroMutatable<T>(pub T, pub f64);
impl<T> PositiveNonZeroMutatable<T>
where T: std::convert::From<f64>, f64: std::convert::From<T> {
  pub fn get_mutated(self, rng : &mut RefMut<SmallRng>) -> Self {
    let v = mutate(rng, self.0.into(), self.1.into()).max(std::f64::MIN_POSITIVE);
    Self(v.into(), self.1)
  }
}
