use super::{BiquadCoeffs, BiquadTrait, BiquadKind};
use crate::filter::Filter;
use core::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Biquad {
  x1: f32, x2: f32, y1: f32, y2: f32,
  bq: BiquadCoeffs,
}

impl Biquad {
  pub fn new(settings: BiquadCoeffs) -> Self {
    Self {
      x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0, 
      bq: settings,
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
  fn update(&mut self, settings: &BiquadCoeffs) {
      self.bq = *settings;
  }
}
