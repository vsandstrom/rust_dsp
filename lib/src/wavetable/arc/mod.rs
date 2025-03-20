use super::*;
use std::sync::{Arc, RwLock};

pub struct Wavetable{
  position: f32,
  table: Arc<RwLock<Vec<f32>>>,
  size: usize,
  samplerate: f32,
  sr_recip: f32,
}

impl Wavetable {
  pub fn new(table: Arc<RwLock<Vec<f32>>>, samplerate: f32) -> Self {
    let size = table.try_read().unwrap().len();
    Self {
      position: 0.0,
      table,
      size,
      samplerate,
      sr_recip: 1.0 / samplerate,
    }
  }

  #[inline]
  pub fn play<T: Interpolation>(&mut self, frequency: f32, phase: f32) -> f32 {
    debug_assert!(self.samplerate > f32::EPSILON, "samplerate has not been set");
    if frequency > self.samplerate * 0.5 { return 0.0; }
    let len= self.size as f32;
    self.position += (len * self.sr_recip * frequency) + len * phase;
    while self.position > len {
      self.position -= len;
    }
    if let Ok(table) = &self.table.try_read() {
      T::interpolate(self.position, table.as_ref(), self.size)
    } else {
      0.0
    }
  }
  
  pub fn set_samplerate(&mut self, samplerate: f32) {
    self.samplerate = samplerate;
    self.sr_recip = 1.0 / samplerate;
  }
}
