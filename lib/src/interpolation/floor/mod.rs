
pub struct Floor { }

/// No interpolation - read position is floored.
impl super::Interpolation for Floor {
  #[inline(always)]
  fn interpolate(position: f32, buffer: &[f32], buffer_size: usize) -> f32 {
    let i: usize = position as usize % buffer_size;
    buffer[i]
  }
}
