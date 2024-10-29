use crate::interpolation::Interpolation;
use std::debug_assert;

pub mod owned {
  use super::*;

  /// Wavetable oscillator that owns its table of the wave representation.
  ///
  /// Fast and reliable but clones the table on init. This seems to be key to
  /// the performance of it, as the table does not lie behind several pointer
  /// references.
  pub struct WaveTable<const N:usize> {
    position: f32,
    table: Vec<f32>,
    size: usize,
    frequency: f32,
    samplerate: f32,
    sr_recip: f32,
  }

  impl<const N:usize> Clone for WaveTable<N> {
    fn clone(&self) -> Self {
      Self {
        position: self.position,
        table: self.table.clone(),
        size: self.size,
        frequency: self.frequency,
        samplerate: self.samplerate,
        sr_recip: self.sr_recip,
      }
    }
  }

    
  impl<const N: usize> WaveTable<N> {
    pub fn new(table: &[f32; N], samplerate: f32) -> Self {
      Self { 
        position: 0.0, 
        table: table.to_vec(),
        size: table.len(),
        frequency: 0.0,
        samplerate,
        sr_recip: 1.0 / samplerate,
      } 
    }

    #[inline]
    pub fn play<T: Interpolation>(&mut self, frequency: f32, phase: f32) -> f32 {
      debug_assert!(self.samplerate > f32::EPSILON, "samplerate has not been set");
      if frequency > self.samplerate * 0.5 { return 0.0; }
      let len = self.size as f32;
      self.position += (len * self.sr_recip * frequency) + (phase * len);
      while self.position > len {
        self.position -= len;
      }
      T::interpolate(self.position, &self.table, self.table.len())
    }
  
    pub fn set_samplerate(&mut self, samplerate: f32) {
      self.samplerate = samplerate;
      self.sr_recip = 1.0 / samplerate;
    }

  }
}

pub mod arc {
  use super::*;
  use std::sync::{Arc, RwLock};

  pub struct WaveTable{
    position: f32,
    table: Arc<RwLock<Vec<f32>>>,
    size: usize,
    samplerate: f32,
    sr_recip: f32,
  }

  impl WaveTable {
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
}

pub mod shared {
  use super::*;

  /// Wavetable that shares the table containing the wave representation.
  ///
  /// Performance lies between owned and Arc<RwLock>, and is preferred when
  /// trying to keep allocated data at a minimum. Beware of stack overflow when
  /// creating too many big arrays. 
  pub struct WaveTable {
    position: f32,
    samplerate: f32,
    sr_recip: f32,
  }

  impl From<f32> for WaveTable {
    /// Create a wavetable instance using the samplerate value
    fn from(samplerate: f32) -> Self {
      Self{
        position: 0.0,
        samplerate,
        sr_recip: 1.0/samplerate
      }
    }
  }

  impl Default for WaveTable {
     fn default() -> Self {
      Self {
        position: 0.0,
        samplerate: 0.0,
        sr_recip: 0.0,
      }
    }
  }

  impl WaveTable {
    pub fn new() -> Self {
      Self {
        position: 0.0,
        samplerate: 0.0,
        sr_recip: 0.0,
      }
    }
  

    /// Play function for wavetable where __SIZE__ is the table size and __TableInterpolation = &impl Interpolation__
    #[inline]
    pub fn play<const SIZE: usize, TableInterpolation>(&mut self, table: &[f32; SIZE], frequency: f32, phase: f32) -> f32
      where
          TableInterpolation: Interpolation
    {
      debug_assert!(self.samplerate > f32::EPSILON, "samplerate has not been set");
      if frequency > self.samplerate * 0.5 { return 0.0; }
      let len = SIZE as f32;
      self.position += (len * self.sr_recip * frequency) + (phase * len);
      while self.position > len { self.position -= len; }
      TableInterpolation::interpolate(self.position, table, SIZE)
    }
      
    pub fn set_samplerate(&mut self, samplerate: f32) {
      self.samplerate = samplerate;
      self.sr_recip = 1.0 / samplerate;
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    interpolation::{Floor, Linear},
    waveshape::traits::Waveshape,
  };

  use super::shared::WaveTable;

  const SAMPLERATE: f32 = 48000.0;

  #[test] 
  fn triangletest_simple() {
    const SIZE: usize = 16;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    // let mut wt = simple::WaveTable::new();
    let mut wt = WaveTable::new();
    wt.set_samplerate(SAMPLERATE);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<SIZE, Floor>(&table, SAMPLERATE/ SIZE as f32, 0.0);
      shape.push(out);
    }
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25,  0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25,  0.0
    ], shape)
  }
  
  #[test] 
  fn interptest_simple() {
    const SIZE: usize = 16;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = WaveTable::new();
    wt.set_samplerate(SAMPLERATE);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<SIZE, Linear>(&table, SAMPLERATE / SIZE as f32, 1.0);
      shape.push(out);
    }
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25, 0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25, 0.0
    ], shape)
  }

  #[test]
  fn freq_test_simple() {
    const SIZE: usize = 8;
    let mut table = [0.0; SIZE];
    let table = Box::new(table.triangle());
    let mut wt = WaveTable::new();
    wt.set_samplerate(SAMPLERATE);
    let mut shape = vec!();
    for _ in 0..20 { 
      let out = wt.play::<SIZE, Linear>(&table, SAMPLERATE / SIZE as f32, 1.0);
      shape.push(out) 
    } 
    println!("{:?}", shape);
  }

  #[test]
  fn linear_test_simple() {
    const SIZE: usize = 4;
    let dilude = 2;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = WaveTable::new();
    wt.set_samplerate(SAMPLERATE);
    let mut shape = vec!();
    for _ in 0..(SIZE * dilude) {
      shape.push(wt.play::<SIZE, Linear>(&table, SAMPLERATE / SIZE as f32 * 0.5, 0.0));
    }
    println!("{:?}", shape);
    assert_eq!(vec![
       0.5,  1.0,  0.5, 0.0,
      -0.5, -1.0, -0.5, 0.0
    ], shape);
  }
}
