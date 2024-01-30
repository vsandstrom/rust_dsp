pub mod interpolation {
    use std::{f32::consts::PI, io::BufRead};

  pub struct Linear { }
  pub struct Cubic { }
  pub struct Cosine { }
  pub struct Hermetic { }
  pub struct Floor { }

  pub trait Interpolation {
    fn interpolate(position: f32, buffer: &Vec<f32>, buffer_size: usize) -> f32;
  }

  /// Linear interpolation - read position is interpolated between 2 points
  impl Interpolation for Linear {
    fn interpolate(position: f32, buffer: &Vec<f32>, buffer_size: usize) -> f32 {
      let prev = position.floor();
      let next = position.ceil();
      let x = position - prev;
      buffer[prev as usize % buffer_size] * (1.0-x) + buffer[next as usize % buffer_size] * x 
    }
  }

      // let buffer = vec![0.0, 4.0, 4.2, 2.0, 1.0];
  /// Cubic interpolation - read position is interpolated between 4 points
  impl Interpolation for Cubic {
    fn interpolate(position: f32, buffer: &Vec<f32>, buffer_size: usize) -> f32 {
      let diff = position - position.floor();
      let a2 = position as usize % buffer_size;
      let a1 = match a2 == 0 {true => buffer_size-1, false => a2-1}; 
      let b1 = match (a2+1) >= buffer_size {true => (a2+1) % buffer_size, false => a2+1};
      let b2 = match (a2+2) >= buffer_size {true => (a2+2) % buffer_size, false => a2+2};
      let c1 = buffer[b2] - buffer[b1] - buffer[a1] + buffer[a2];
      let c2 = buffer[a1] - buffer[a2] - c1;
      let c3 = buffer[b1] - buffer[a1];
      (c1 * f32::powf(diff, 3.0)) + (c2 * f32::powf(diff, 2.0)) + (c3 * diff) + buffer[a2]
    }
  }

  impl Interpolation for Cosine {
    fn interpolate(position: f32, buffer: &Vec<f32>, buffer_size: usize) -> f32 {
      let diff = position - position.floor();
      let a1 = position as usize;
      let b1 = match a1 + 1 >= buffer_size {true => (a1+1) % buffer_size, false => a1+1};
      let bw = (1.0 - f32::cos(diff*PI)) / 2.0;
      let aw = 1.0 - bw;
      buffer[a1] * aw + buffer[b1] * bw
    } 
  }

  impl Interpolation for Hermetic {
    fn interpolate(position: f32, buffer: &Vec<f32>, buffer_size: usize) -> f32 {
      let diff = position - position.floor();
      let a2 = position as usize % buffer_size;
      let a1 = match a2 == 0 {true => buffer_size-1, false => a2-1}; 
      let b1 = match (a2+1) >= buffer_size {true => (a2+1) % buffer_size, false => a2+1};
      let b2 = match (a2+2) >= buffer_size {true => (a2+2) % buffer_size, false => a2+2};
      let sub = buffer[a2] - buffer[b1];
      let c1 = buffer[b1] - buffer[a1];
      let c3 = buffer[b2] - buffer[a2] + 3.0 * sub;
      let c2 = -2.0 * sub + c1 + c3;
      0.5 * ((c3*diff+c2) * diff + c1) * diff + buffer[a2]
      
    }
  }

  /// No interpolation - read position is floored.
  impl Interpolation for Floor {
    fn interpolate(position: f32, buffer: &Vec<f32>, buffer_size: usize) -> f32 {
      let i: usize = position as usize % buffer_size;
      buffer[i]
    }
  }
}
 
#[cfg(test)]
mod tests {
  extern crate waveshape;
  use crate::interpolation::{Linear, Cubic, Floor, Interpolation};
  use waveshape::sine;

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
      assert_eq!(0.5, Linear::interpolate(pos, &buffer, 2), "wrapping around buffer works")
    }

    #[test]
    fn cubic_wrap_test() {
      let buffer = vec![0.0, 4.0, 4.2, 2.0, 1.0];
      let pos = 7.25;
      assert_eq!(3.725, Cubic::interpolate(pos, &buffer, 5), "wrapping around buffer works")
    }

    #[test] fn cubic_vs_linear() {
      let mut buf = Vec::with_capacity(16);
      sine(&mut buf, 16);
      let pos = 4.5;
      let lin = Linear::interpolate(pos, &buf, 16);
      let cub = Cubic::interpolate(pos, &buf, 16);
      assert_ne!(lin, cub, "Linear: {} should not be equal Cubic: {}", lin, cub)
    }
}
