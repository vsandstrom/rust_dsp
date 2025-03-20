use super::{BiquadCoeffs, BiquadTrait};
use crate::filter::Filter;

#[derive(Clone, Copy)]
pub struct Biquad {
  x1: f32, x2: f32, y1: f32, y2: f32,
  bq: BiquadCoeffs,
}

impl Default for Biquad {
  fn default() -> Self { Self::new() }
}

impl Biquad {
  pub fn new() -> Self {
    Self {
      x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0, 
      bq: BiquadCoeffs{ a1: 0.0, a2: 0.0, b0: 0.0, b1: 0.0, b2: 0.0 },
    }
  }
}

impl Filter for Biquad {
  fn process(&mut self, sample: f32) -> f32 {
    let output = {
        self.bq.b0 * sample 
      + self.bq.b1 * self.x1 
      + self.bq.b2 * self.x2
      - self.bq.a1 * self.y1
      - self.bq.a2 * self.y2
    };

    self.x2 = self.x1;
    self.x1 = sample;
    self.y2 = self.y1;
    self.y1 = output;
    output
  }
}

impl BiquadTrait for Biquad {
  fn update(&mut self, bq: BiquadCoeffs) {
      self.bq = bq;
  }
}
