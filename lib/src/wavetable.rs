use crate::interpolation::Interpolation;
use std::debug_assert;

pub struct WaveTable {
  position: f32,
  samplerate: f32,
  sr_recip: f32,
}

impl WaveTable {
  pub fn new() -> Self {
    Self {
      position: 0.0,
      samplerate: 0.0,
      sr_recip: 0.0,
    }
  }

  #[inline]
  pub fn play<const N: usize, TableInterpolation>(&mut self, table: &[f32; N], frequency: f32, phase: f32) -> f32
    where
        TableInterpolation: Interpolation
  {
    debug_assert!(self.samplerate > f32::EPSILON, "samplerate has not been set");
    if frequency > self.samplerate * 0.5 { return 0.0; }
    let len = N as f32;
    self.position += len * self.sr_recip * frequency + (phase * len);
    while self.position > len { self.position -= len; }
    while self.position < 0.0 { self.position += len; }
    TableInterpolation::interpolate(self.position, table, N)
  }
    
  pub fn set_samplerate(&mut self, samplerate: f32) {
    self.samplerate = samplerate;
    self.sr_recip = 1.0 / samplerate;
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use crate::{
    interpolation::{Floor, Linear},
    waveshape::traits::Waveshape
  };

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
    let table = table.triangle();
    let mut wt = WaveTable::new();
    wt.set_samplerate(SAMPLERATE);
    let mut shape = vec!();
    for _ in 0..20 { 
      let out = wt.play::<SIZE, Floor>(&table, 1.0, 1.0);
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
      shape.push(wt.play::<SIZE, Linear>(&table, SAMPLERATE / (SIZE * dilude) as f32, 1.0));
    }
    println!("{:?}", shape);
    assert_eq!(vec![
       0.5,  1.0,  0.5, 0.0,
      -0.5, -1.0, -0.5, 0.0
    ], shape);
  }
}
