use core::f32::consts::PI;
use std::f32::consts::FRAC_PI_2 as P2;

#[repr(C)]

/// Uses f32::abs() as non-linear function
/// ```
/// let x = input.sin() * (amount + 1.0) * 10.0;
/// let y = 0.25 * x - 0.25;
/// 4.0 * ((y - y.round()).abs() - 0.25)
/// ```
pub struct Abs {}
/// Uses f32::sin() as non-linear function
/// ```
/// let x = input.sin() * (amount + 1.0) * 10.0;
/// let y = 0.25 * x - 0.25;
/// 4.0 * ((y - y.round()).sin())
/// ```
pub struct Sin {}
/// Uses f32::tanh() as non-linear function
/// ```
/// let x = input.sin() * (amount + 1.0) * 10.0;
/// let y = 0.25 * x - 0.25;
/// 4.0 * ((y - y.round()).tanh())
/// ```
pub struct Tanh {}


pub trait FoldTrait {
  fn fold(y: f32) -> f32;
}

impl FoldTrait for Abs {
  fn fold(y: f32) -> f32 {
    4.0 * ((y - y.round()).abs() - 0.25)
  }
}

impl FoldTrait for Sin {
  fn fold(y: f32) -> f32 {
    4.0 * ((y - y.round()).sin())
  }
}

impl FoldTrait for Tanh {
  fn fold(y: f32) -> f32 {
    4.0 * ((y - y.round()).tanh())
  }
}

#[repr(C)]
pub struct Fold {}

impl Fold {
  /// Simple Wavefolder introducing different types of nonlinearities. 
  pub fn process<FoldType>(input: f32, amount: f32) -> f32 
    where FoldType: FoldTrait
  {
    let x = input.sin() * (amount + 1.0) * 10.0;
    let y = 0.25 * x - 0.25;
    FoldType::fold(y)
  }
}
