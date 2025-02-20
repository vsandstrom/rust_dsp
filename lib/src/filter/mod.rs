pub mod comb;
pub mod biquad;
pub mod onepole;

use alloc::{vec, vec::Vec};

pub trait Filter {
  fn process(&mut self, sample: f32) -> f32;
}




