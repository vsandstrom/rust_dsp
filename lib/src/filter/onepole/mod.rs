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
  /// self.damp has the range of -1 .. 1
  /// self.damp > 0 == lowpass
  /// self.damp < 0 == highpass
  /// keep |self.damp| < 1 for stability
  pub fn set_coeff(&mut self, coeff: f32) {
    self.coeff = coeff;
  }
}

impl Filter for Onepole {
  /// self.damp has the range of -1 .. 1
  /// self.damp > 0 == lowpass
  /// self.damp < 0 == highpass
  /// keep |self.damp| < 1 for stability
  fn process(&mut self, sample: f32) -> f32 {
    self.prev = ((1.0 - f32::abs(self.coeff)) * sample) + (self.coeff * self.prev);
    self.prev
  }
}
