use super::{BiquadCoeffs, BiquadTrait};
use crate::filter::Filter;

#[derive(Clone, Copy)]
pub struct Biquad4 {
  x1_1: f32, x1_2: f32, y1_1: f32, y1_2: f32,
  x2_1: f32, x2_2: f32, y2_1: f32, y2_2: f32,
  bq: BiquadCoeffs,
}

impl Biquad4 {
  pub fn new(settings: BiquadCoeffs) -> Self {
    Self { 
      x1_1: 0.0, x1_2: 0.0, y1_1: 0.0, y1_2: 0.0, x2_1: 0.0, x2_2: 0.0, y2_1: 0.0, y2_2: 0.0,
      bq: settings,
    }
  }
}

impl Filter for Biquad4 {
  fn process(&mut self, sample: f32) -> f32 {
    let mut output = 
        self.bq.b0 * sample 
      + self.bq.b1 * self.x1_1 
      + self.bq.b2 * self.x1_2
      - self.bq.a1 * self.y1_1
      - self.bq.a2 * self.y1_2;

    self.x1_2 = self.x1_1;
    self.x1_1 = sample;
    self.y1_2 = self.y1_1;
    self.y1_1 = output;
    
    output = 
        self.bq.b0 * output 
      + self.bq.b1 * self.x2_1 
      + self.bq.b2 * self.x2_2
      - self.bq.a1 * self.y2_1
      - self.bq.a2 * self.y2_2;

    self.x2_2 = self.x2_1;
    self.x2_1 = self.y1_1;
    self.y2_2 = self.y2_1;
    self.y2_1 = output;
    output
      
  }
}

impl BiquadTrait for Biquad4 {
  fn update(&mut self, settings: &BiquadCoeffs) {
      self.bq = *settings;
  }
}

