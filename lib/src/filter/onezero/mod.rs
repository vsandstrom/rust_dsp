use super::Filter;
use core::f32::consts::TAU;

#[derive(Default)]
pub struct Onezero {
  prev: f32,
  coeff: f32,
  samplerate: u32
}

impl Onezero {
  pub fn new(samplerate: u32) -> Self {
    Self {
      samplerate,
      ..Default::default()
    }
  }

  pub fn set_coeff(&mut self, coeff: f32) {
    self.coeff = coeff
  }

  pub fn set_cutoff(&mut self, freq: f32) {
    self.set_coeff((-TAU * freq / self.samplerate as f32).exp());
  }
}

impl Filter for Onezero {
  fn process(&mut self, sample: f32) -> f32 {
    let out = sample + self.prev;
    self.prev = sample;
    out
  }
}
