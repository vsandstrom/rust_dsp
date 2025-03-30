use core::f32::consts::PI;
use std::f32::consts::FRAC_PI_2 as P2;


#[repr(C)]
pub struct Fold{}

impl Fold {
  pub fn process(input: f32, amount: f32) -> f32 {
    let x = input.sin() * (amount + 1.0) * 10.0;
    let y = 0.25 * x - 0.25;
    // 4.0 * ((y - y.round()).abs() - 0.25)
    // 4.0 * ((y - y.round()).tanh())
    // 4.0 * ((y - y.round()).sin())
  }
}
