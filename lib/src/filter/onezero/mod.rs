use super::Filter;

#[derive(Default)]
pub struct Onezero {
  prev: f32,
  coeff: f32
}

impl Onezero {
  pub fn set_coeff(&mut self, coeff: f32) {
    self.coeff = coeff
  }
}

impl Filter for Onezero {
  fn process(&mut self, sample: f32) -> f32 {
    let out = sample + self.prev;
    self.prev = sample;
    out
  }
}
