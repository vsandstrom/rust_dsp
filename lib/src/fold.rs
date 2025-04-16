use std::f32::consts::PI;

#[repr(C)]
pub enum FoldType {
  Abs,
  Sin,
  Tanh
}

#[repr(C)]
/// Uses f32::abs() as non-linear function
/// ```
/// let x = input.sin() * (amount + 1.0) * 10.0;
/// let y = 0.25 * x - 0.25;
/// 4.0 * ((y - y.round()).abs() - 0.25)
/// ```
pub struct Abs {}
#[repr(C)]
/// Uses f32::sin() as non-linear function
/// ```
/// let x = input.sin() * (amount + 1.0) * 10.0;
/// let y = 0.25 * x - 0.25;
/// 4.0 * ((y - y.round()).sin())
/// ```
pub struct Sin {}
#[repr(C)]
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
  #[inline]
  fn fold(y: f32) -> f32 {
    4.0 * ((y - y.round()).abs() - 0.25)
  }
}

impl FoldTrait for Sin {
  #[inline]
  fn fold(y: f32) -> f32 {
    4.0 * ((y - y.round()).sin())
  }
}

impl FoldTrait for Tanh {
  #[inline]
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

  pub fn process_dyn(input: f32, amount: f32, foldtype: FoldType) -> f32 {
    let x = input.sin() * (amount + 1.0) * 10.0;
    let y = 0.25 * x - 0.25;
    match foldtype {
      FoldType::Abs => Abs::fold(y),
      FoldType::Sin => Sin::fold(y),
      FoldType::Tanh => Tanh::fold(y),
    }
  }
}

pub fn sin_fold(sig: f32, a: f32) -> f32 {
  f32::sin(f32::sin(sig.sin() * 0.25 * a) * a) * 0.5 * a
}

pub fn tanh_fold(sig: f32, a: f32) -> f32 {
  f32::tanh(f32::tanh(sig.tanh() * 0.25 * a) * a) * 0.5 * a
}

pub fn mix_fold(sig: f32, a: f32) -> f32 {
  -f32::cos(f32::abs(f32::sin(sig*a) * 0.5 * a + 0.5 * PI) * a) * 0.5 * a
}
