
pub struct Hermite { }

/// Hermite interpolation - read position is interpolated between 4 points
impl super::Interpolation for Hermite {
  #[inline(always)]
  fn interpolate(position: f32, buffer: &[f32], buffer_size: usize) -> f32 {
    let diff = position.fract();
    let a2 = position as usize % buffer_size;
    let a1 = {if a2 == 0 { buffer_size-1 } else { a2 - 1 }};
    let b1 = {if a2 + 1 >= buffer_size { a2 + 1 - buffer_size } else { a2 + 1 }};
    let b2 = {if b1 + 1 >= buffer_size { b1 + 1 - buffer_size } else { b1 + 1 }};
    let sub = buffer[a2] - buffer[b1];
    let c1 = buffer[b1] - buffer[a1];
    let c3 = buffer[b2] - buffer[a2] + 3.0 * sub;
    let c2 = -(2.0 * sub + c1 + c3);
    0.5 * ((c3*diff+c2) * diff + c1) * diff + buffer[a2]
  }
}
