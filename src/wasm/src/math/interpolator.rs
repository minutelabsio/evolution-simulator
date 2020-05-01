use super::lerp;

pub struct Interpolator {
  pts: Vec<(f64, f64)>
}

impl Interpolator {
  pub fn new(pts : &Vec<(f64, f64)>) -> Self {

    Self {
      pts: pts.clone()
    }
  }

  pub fn get(&self, x : f64) -> f64 {
    let len = self.pts.len();
    let min_x = self.pts[0].0;
    let max_x = self.pts[len - 1].0;

    if x < min_x {
      return self.pts[0].1;
    }

    if x >= max_x {
      return self.pts[len - 1].1;
    }

    let mut i = 0;

    for (idx, p) in self.pts.iter().enumerate() {
      i = idx;
      if x < p.0 {
        break;
      }
    }

    let x1 = self.pts[i - 1].0;
    let x2 = self.pts[i].0;
    if x1 == x2 { return self.pts[i].1 }

    let t = (x - x1) / (x2 - x1);
    return lerp(self.pts[i - 1].1, self.pts[i].1, t)
  }
}
