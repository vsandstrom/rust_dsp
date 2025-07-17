use super::{BiquadCoeffs, BiquadTrait, BiquadKind};
use crate::filter::Filter;
use core::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Biquad4<T: BiquadKind> {
  x1_1: f32, x1_2: f32, y1_1: f32, y1_2: f32,
  x2_1: f32, x2_2: f32, y2_1: f32, y2_2: f32,
  bq: BiquadCoeffs,
  _marker: PhantomData<T>
}

impl<T: BiquadKind> Biquad4<T> {
  pub fn new(settings: T::Settings) -> Self {
    Self { 
      x1_1: 0.0, x1_2: 0.0, y1_1: 0.0, y1_2: 0.0, x2_1: 0.0, x2_2: 0.0, y2_1: 0.0, y2_2: 0.0,
      bq: T::calc(&settings),
        // BiquadCoeffs{a1: 0.0, a2: 0.0, b0: 0.0, b1: 0.0, b2: 0.0},
      _marker: PhantomData
    }
  }
}

impl<T: BiquadKind> Filter for Biquad4<T> {
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

impl<T: BiquadKind> BiquadTrait<T> for Biquad4<T> {
  fn update(&mut self, settings: &T::Settings) {
      self.bq = T::calc(settings);
  }
}

