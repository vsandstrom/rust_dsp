#![deprecated = "Implementation with Arc<RwLock<Vec<f32>>> table is not usable in real world"]

use super::*;
use std::sync::{Arc, RwLock};

#[derive(Clone, Default, Debug)]
pub struct Wavetable{
  position: f32,
  table: Arc<RwLock<Vec<f32>>>,
  size: usize,
  samplerate: u32,
  sr_recip: f32,
}

impl Wavetable {
  pub fn new(table: Arc<RwLock<Vec<f32>>>, samplerate: u32) -> Self {
    let size = table.try_read().unwrap().len();
    Self {
      position: 0.0,
      table,
      size,
      samplerate,
      sr_recip: 1.0 / samplerate as f32,
    }
  }

  #[inline]
  pub fn play<T: Interpolation>(&mut self, frequency: f32, phase: f32) -> f32 {
    let len= self.size as f32;
    self.position += len * self.sr_recip * frequency;
    if self.position > len { self.position -= len; }
    let mut pos = self.position + (phase * len);
    while pos > len { pos -= len; }
    while pos < 0.0 { pos += len; }
    if let Ok(table) = &self.table.try_read() {
      T::interpolate(pos, table.as_ref(), self.size)
    } else {
      0.0
    }
  }
  
  pub fn set_samplerate(&mut self, samplerate: u32) {
    self.samplerate = samplerate;
    self.sr_recip = 1.0 / samplerate as f32;
  }
}
