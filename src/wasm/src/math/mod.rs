use nalgebra::Vector2;
use nalgebra::Point2;

mod stats;
pub use stats::*;

// r1, r2. Points defining line segment
// https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line#Vector_formulation
pub fn distance_to_line(r1 : &Point2<f64>, r2 : &Point2<f64>, p : &Point2<f64>) -> Option<f64> {
  let a = r1;
  let v = r2 - r1;
  let n = v.normalize();
  // vector from the point to first point on line
  let pa = a - p;
  let pb = r2 - p;

  let pa_dot_n = pa.dot(&n);
  let pb_dot_n = pb.dot(&n);

  // this means we're outside the line segment
  if pa_dot_n * pb_dot_n > 0. {
    return None
  }

  // projection of pa onto the line
  let z = pa_dot_n * n;

  let d = pa - z;
  Some(d.norm())
}

// gives the projection vector of point p wrt r1
pub fn project_to_line(r1 : &Point2<f64>, r2 : &Point2<f64>, p : &Point2<f64>) -> Option<Vector2<f64>> {
  let a = r1;
  let v = r2 - r1;
  let n = v.normalize();
  // vector from the point to first point on line
  let pa = a - p;
  let pb = r2 - p;

  let pa_dot_n = pa.dot(&n);
  let pb_dot_n = pb.dot(&n);

  // this means we're outside the line segment
  if pa_dot_n * pb_dot_n > 0. {
    return None
  }

  // projection of pa onto the line
  Some(-pa_dot_n * n)
}
