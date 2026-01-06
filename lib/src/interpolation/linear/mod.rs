pub struct Linear { }

/// Linear interpolation - read position is interpolated between 2 points
impl super::Interpolation for Linear {
  #[inline(always)]
  fn interpolate(position: f32, buffer: &[f32], buffer_size: usize) -> f32 {
    let pos = position as usize % buffer_size;
    let pos2 = (pos + 1) % buffer_size;
    let x = position.fract();
    let a = buffer[pos];
    let b = buffer[pos2];
    a + x * (b - a)
    // buffer[pos % buffer_size] * (1.0-x) + buffer[(pos+1) % buffer_size] * x 
  }
}
