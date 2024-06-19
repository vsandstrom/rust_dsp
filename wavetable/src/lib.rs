extern crate interpolation;
extern crate waveshape;
extern crate dsp;

use interpolation::interpolation::InterpolationConst;
use dsp::signal::clamp;

pub mod single {
  /// Single refers to the ownership of the underlying wavetable structure.
  /// In the `single` module, the table is always owned by the instance of 
  /// the WaveTable struct.


  use super::*;

  pub struct WaveTable<const N:usize> {
    position: f32,
    table: [f32; N],
    size: usize,
    pub frequency: f32,
    samplerate: f32,
  }

  impl<const N:usize> Clone for WaveTable<N> {
    fn clone(&self) -> Self {
      Self {
        position: self.position,
        table: self.table.clone(),
        size: self.size,
        frequency: self.frequency,
        samplerate: self.samplerate,
      }
    }
  }
    
  impl<const N: usize> WaveTable<N> {
    pub fn new(table: &[f32; N], samplerate: f32) -> WaveTable<N> {
      WaveTable { 
        position: 0.0, 
        table: *table,
        size: table.len(),
        frequency: 0.0,
        samplerate,
      } 
    }

    pub fn update_table(&mut self, value: f32, index: usize) -> std::result::Result<(), &'static str> {
      match self.table.get_mut(index) {
        Some(x) => {*x = value; Ok(())},
        None => Err("table out of bounds")
      }
    }

    pub fn play<T: InterpolationConst>(&mut self, frequency: f32, phase: f32) -> f32 {
      if frequency > (self.samplerate / 2.0) { return 0.0; }
      self.frequency = frequency;
      let norm_ph = clamp((phase+1.0)*0.5, 0.0, 1.0);
      let len = self.size;
      self.position += len as f32 / (self.samplerate /  (frequency * norm_ph));
      while self.position > self.size as f32 {
        self.position -= self.size as f32;
      }
      T::interpolate(self.position, &self.table, self.table.len())
    }

    #[allow(unused)]
    fn read(&mut self) -> f32 {
      let out = self.table[self.position as usize];
      self.position = ((self.position as usize + 1) % (self.table.len())) as f32;
      out
    }
  }
}

pub mod shared {
  /// Shared refers to the ownership of the underlying wavetable structure.
  /// In the `shared` module, the table can be shared between the instances
  /// of the WaveTable struct over threads. The changes to the underlying 
  /// wavetable propagates through the shared references.

  use super::{interpolation::interpolation::InterpolationConst, clamp};
  use std::sync::{Arc, RwLock};

  pub struct WaveTable{
    position: f32,
    table: Arc<RwLock<Vec<f32>>>,
    size: usize,
    pub frequency: f32,
    samplerate: f32
  }

  impl WaveTable {
    pub fn new(table: Arc<RwLock<Vec<f32>>>, samplerate: f32) -> WaveTable {
      let size = table.try_read().unwrap().len();
      WaveTable{
        position: 0.0,
        table,
        size,
        frequency: 0.0,
        samplerate
      }
    }

    pub fn play<T: InterpolationConst>(&mut self, frequency: f32, phase: f32) -> f32 {
      if frequency > (self.samplerate / 2.0) { return 0.0; }
      self.frequency = frequency;
      let norm_ph = clamp((phase+1.0)*0.5, 0.0, 1.0);
      self.position += self.size as f32 / (self.samplerate / (frequency * norm_ph));
      while self.position > self.size as f32 {
        self.position -= self.size as f32;
      }
      if let Ok(table) = &self.table.try_read() {
        T::interpolate(self.position, table.as_ref(), self.size)
      } else {
        0.0
      }
    }

    #[allow(unused)]
    fn read(&mut self) -> f32 {
      let mut out = 0.0;
      if let Ok(table) = self.table.try_read() {
        out = table[self.position as usize];
      }
      self.position = ((self.position as usize + 1) % (self.size)) as f32;
      out
    }
  }

}

