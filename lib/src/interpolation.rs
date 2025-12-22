use core::{f32::consts::PI, usize};

pub struct Linear { }
pub struct Cubic { }
pub struct Cosine { }
pub struct Hermite { }
pub struct Floor { }

pub trait Interpolation {
  fn interpolate(position: f32, buffer: &[f32], buffer_size: usize) -> f32;
}

/// Linear interpolation - read position is interpolated between 2 points
impl Interpolation for Linear {
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

/// Cubic interpolation - read position is interpolated between 4 points
impl Interpolation for Cubic {
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

/// Cosine interpolation - read position is interpolated between 4 points
impl Interpolation for Cosine {
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


/// Hermite interpolation - read position is interpolated between 4 points
impl Interpolation for Hermite {
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

/// No interpolation - read position is floored.
impl Interpolation for Floor {
  #[inline(always)]
  fn interpolate(position: f32, buffer: &[f32], buffer_size: usize) -> f32 {
    let i: usize = position as usize % buffer_size;
    buffer[i]
  }
}

 
#[cfg(test)]
mod tests {
  use alloc::vec;
  use super::*;
  use crate::waveshape::sine;

    #[test]
    fn none_test() {
      let buffer = vec![0.0, 1.0];
      let pos = 0.5;
      assert_eq!(0.0, Floor::interpolate(pos, &buffer, 2))
    }

    #[test]
    fn linear_test() {
      let buffer = vec![0.0, 1.0];
      let pos = 0.5;
      assert_eq!(0.5, Linear::interpolate(pos, &buffer, 2))
    }

    #[test]
    fn cubic_test() {
      let buffer = vec![0.0, 1.0, 2.0, 1.0];
      let pos = 1.5;
      assert_eq!(1.75, Cubic::interpolate(pos, &buffer, 4))
    }
    
    #[test]
    fn cubic_test2() {
      let buffer = vec![0.0, 4.0, 2.0, 1.0];
      let pos = 1.5;
      assert_eq!(3.625, Cubic::interpolate(pos, &buffer, 4))
    }
    
    #[test]
    fn cubic_test3() {
      let buffer = vec![0.0, 4.0, 4.2, 2.0, 1.0];
      let pos = 2.25;
      // LOL
      assert_eq!(3.725, Cubic::interpolate(pos, &buffer, 5))
    }
    
    #[test]
    fn linear_wrap_test() {
      let buffer = vec![0.0, 1.0];
      let pos = 2.5;
      assert_eq!(0.5, Linear::interpolate(pos, &buffer, 2), "wrapping around linear")
    }

    #[test]
    fn cubic_wrap_test() {
      let buffer = vec![0.0, 4.0, 4.2, 2.0, 1.0];
      let pos = 7.25;
      assert_eq!(3.725, Cubic::interpolate(pos, &buffer, 5), "wrapping around cubic")
    }

    #[test] fn cubic_vs_linear() {
      let mut buf = [0.0; 512];
      sine(&mut buf);
      let pos = 4.5;
      let lin = Linear::interpolate(pos, &buf, 512);
      let cub = Cubic::interpolate(pos, &buf, 512);
      assert_ne!(lin, cub, "Linear: {} should not be equal Cubic: {}", lin, cub)
    }
}
