/// Simple implementation of linear interpolation
pub fn lerp<T>(
  first : T,
  second : T,
  t : f64,
) -> <<T as std::ops::Mul<f64>>::Output as std::ops::Add>::Output
where
  T : std::ops::Mul<f64>,
  <T as std::ops::Mul<f64>>::Output : std::ops::Add<<T as std::ops::Mul<f64>>::Output>,
{
  first * (1. - t) + second * t
}
