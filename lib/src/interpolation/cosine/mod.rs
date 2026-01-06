use core::f32::consts::PI;

pub struct Cosine { }

/// Cosine interpolation - read position is interpolated between 4 points
impl super::Interpolation for Cosine {
  #[inline(always)]
  fn interpolate(position: f32, buffer: &[f32], buffer_size: usize) -> f32 {
    let diff = position.fract();
    let n = position as usize;
    let m = if n + 1 >= buffer_size { n - (buffer_size - 1) } else { n + 1 };
    let a = buffer[n];
    let b = buffer[m];
    let x = (1.0 - f32::cos(diff*PI)) / 2.0;
    a + x * (b - a)
  } 
}
