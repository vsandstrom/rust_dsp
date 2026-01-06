
pub struct Cubic { }

/// Cubic interpolation - read position is interpolated between 4 points
impl super::Interpolation for Cubic {
  #[inline(always)]
  fn interpolate(position: f32, buffer: &[f32], buffer_size: usize) -> f32 {
    let a2 = (position.floor() as usize) % buffer_size;
    let diff = position.fract();
    let a1 = {if a2 == 0 { buffer_size - 1 } else { a2 - 1 }};
    let b1 = {if a2 + 1 >= buffer_size { a2 - (buffer_size - 1) } else { a2 + 1 }};
    let b2 = {if b1 + 1 >= buffer_size { b1 - (buffer_size - 1) } else { b1 + 1 }};

    let c0 = buffer[b2] - buffer[b1] - buffer[a1] + buffer[a2];
    let c1 = buffer[a1] - buffer[a2] - c0;
    let c2 = buffer[b1] - buffer[a1];
    (c0 * f32::powf(diff, 3.0)) + (c1 * f32::powf(diff, 2.0)) + (c2 * diff) + buffer[a2]
  }
}
