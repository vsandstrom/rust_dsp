use super::Filter;

#[derive(Default)]
pub struct Onepole {
  prev: f32,
  coeff: f32
}

impl Onepole {
  pub fn new() -> Self {
    Self::default()
  }
  /// `0.0 < coeff < 1.0 == lowpass`
  /// `|self.coeff| < 1 for stability`
  pub fn set_coeff(&mut self, coeff: f32) {
    self.coeff = coeff;
  }
}

impl Filter for Onepole {
  /// `0.0 < coeff < 1.0 == lowpass`
  /// `|self.coeff| < 1 for stability`
  fn process(&mut self, sample: f32) -> f32 {
    self.prev = ((1.0 - self.coeff) * sample) + (self.coeff * self.prev);
    self.prev
  }
}

#[derive(Default)]
pub struct LagFilter {
  prev: f32,
  coeff: f32
}

impl LagFilter {
  pub fn new() -> Self {
    Self::default()
  }
  /// `self.coeff > 0 == lowpass`
  /// `self.coeff < 0 == highpass`
  /// `|self.coeff| < 1 for stability`
  pub fn set_coeff(&mut self, coeff: f32) {
    self.coeff = coeff;
  }
}

impl Filter for LagFilter {
  /// `self.coeff > 0 == lowpass`
  /// `self.coeff < 0 == highpass`
  /// `|self.coeff| < 1 for stability`
  fn process(&mut self, sample: f32) -> f32 {
    self.prev = ((1.0 - f32::abs(self.coeff)) * sample) + (self.coeff * self.prev);
    self.prev
  }
}




