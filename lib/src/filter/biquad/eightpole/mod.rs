use super::{BiquadCoeffs, BiquadTrait};
use crate::filter::Filter;

#[derive(Clone, Copy)]
pub struct Biquad8 {
  x1_1: f32, x1_2: f32, y1_1: f32, y1_2: f32, 
  x2_1: f32, x2_2: f32, y2_1: f32, y2_2: f32,
  x3_1: f32, x3_2: f32, y3_1: f32, y3_2: f32,
  x4_1: f32, x4_2: f32, y4_1: f32, y4_2: f32,
  bq: BiquadCoeffs,
}

impl Biquad8 {
  pub fn new(settings: BiquadCoeffs) -> Self { 
    Self { 
      x1_1: 0.0, x1_2: 0.0, y1_1: 0.0, y1_2: 0.0,
      x2_1: 0.0, x2_2: 0.0, y2_1: 0.0, y2_2: 0.0,
      x3_1: 0.0, x3_2: 0.0, y3_1: 0.0, y3_2: 0.0,
      x4_1: 0.0, x4_2: 0.0, y4_1: 0.0, y4_2: 0.0, 
      bq: settings,
    }
  }
}


impl Filter for Biquad8 {
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
    
    output = 
        self.bq.b0 * output 
      + self.bq.b1 * self.x3_1 
      + self.bq.b2 * self.x3_2
      - self.bq.a1 * self.y3_1
      - self.bq.a2 * self.y3_2;

    self.x3_2 = self.x3_1;
    self.x3_1 = self.y2_1;
    self.y3_2 = self.y3_1;
    self.y3_1 = output;
      
    output = 
        self.bq.b0 * output 
      + self.bq.b1 * self.x4_1 
      + self.bq.b2 * self.x4_2
      - self.bq.a1 * self.y4_1
      - self.bq.a2 * self.y4_2;

    self.x4_2 = self.x4_1;
    self.x4_1 = self.y3_1;
    self.y4_2 = self.y4_1;
    self.y4_1 = output;
    output
  }
}

impl BiquadTrait for Biquad8 {
  fn update(&mut self, settings: &BiquadCoeffs) {
      self.bq = *settings;
  }
}

