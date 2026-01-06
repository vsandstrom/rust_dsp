pub mod linear;
pub mod cubic;
pub mod cosine;
pub mod hermite;
pub mod floor;

pub use {
  linear::Linear,
  cubic::Cubic,
  cosine::Cosine,
  hermite::Hermite,
  floor::Floor
};


pub trait Interpolation {
  fn interpolate(position: f32, buffer: &[f32], buffer_size: usize) -> f32;
}

#[cfg(test)]
mod tests {
  use alloc::vec;
  use super::*;
  use crate::waveshape::sine;
  use super::linear::Linear;
  use super::cubic::Cubic;
  use super::floor::Floor;

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
