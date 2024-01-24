pub mod interpolation {
  pub struct Linear { }
  pub struct Cubic { }
  pub struct Cosine { }
  pub struct Floor { }

  pub trait Interpolation {
    fn interpolate(position: f32, buffer: &Vec<f32>, buffer_size: usize) -> f32;
  }

  /// Linear interpolation - read position is interpolated between 2 points
  impl Interpolation for Linear {
    fn interpolate(position: f32, buffer: &Vec<f32>, buffer_size: usize) -> f32 {
      let prev = position.floor() as usize % buffer_size;
      let next = (position.ceil() as usize) % buffer_size;
      let x = position - (prev as f32);
      buffer[prev as usize] * (1.0-x) + buffer[next] * x 
    }
  }

  /// Cubic interpolation - read position is interpolated between 4 points
  impl Interpolation for Cubic {
    fn interpolate(position: f32, buffer: &Vec<f32>, buffer_size: usize) -> f32 {
      let prev = position.floor();
      let next = position.ceil();
      let pprev = prev as usize + buffer_size % buffer_size; // negative overflow guard
      let nnext = (next as usize + 1) % buffer_size;
      let diff = position - prev;
      let c0 = buffer[nnext] - buffer[next as usize] - buffer[pprev] + buffer[prev as usize];
      let c1 = buffer[pprev] - buffer[prev as usize] - c0;
      let c2 = buffer[next as usize] - buffer[pprev];
      (c0 * f32::powf(diff, 3.0)) + (c1 * (diff*diff)) + (c2 * diff) + buffer[prev as usize]
    }
  }

  impl Interpolation for Cosine {
    #[allow(unused)]
    fn interpolate(position: f32, buffer: &Vec<f32>, buffer_size: usize) -> f32 {
        todo!()
    } }

  /// No interpolation - read position is floored.
  impl Interpolation for Floor {
    fn interpolate(position: f32, buffer: &Vec<f32>, buffer_size: usize) -> f32 {
      let i: usize = position.floor() as usize;
      buffer[i]
    }
  }
}
 
#[cfg(test)]
mod tests {
  use crate::interpolation::{Linear, Cubic, Floor, Interpolation};

use super::*;

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
      assert_eq!(1.625, Cubic::interpolate(pos, &buffer, 4))
    }
    
    #[test]
    fn cubic_test2() {
      let buffer = vec![0.0, 4.0, 2.0, 1.0];
      let pos = 1.5;
      assert_eq!(3.125, Cubic::interpolate(pos, &buffer, 4))
    }
    
    #[test]
    fn cubic_test3() {
      let buffer = vec![0.0, 4.0, 4.2, 2.0, 1.0];
      let pos = 2.25;
      // LOL
      assert_eq!(3.7437500000000004, Cubic::interpolate(pos, &buffer, 4))
    }
}
