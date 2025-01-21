use crate::interpolation::Interpolation;
#[cfg(not(feature="std"))]
use alloc::{vec, vec::Vec};

/// Interpolating oscillator
///
/// Linearly interpolating through an array tables. 
/// Internaly the tables use the user-supplied method 
/// of type `trait Interpolation`
pub struct VectorOscillator {
  table_pos: f32,
  samplerate: f32,
  sr_recip: f32,
}

impl VectorOscillator {
  pub fn new(samplerate: f32) -> Self {
    Self {
      table_pos: 0.0,
      samplerate,
      sr_recip: 1.0 / samplerate,
    }
  }

  pub fn play<const LENGTH: usize, T: Interpolation>(&mut self, tables: &[[f32; LENGTH]], frequency: f32, position: f32, phase: f32) -> f32 {
    if frequency > self.samplerate * 0.5 {return 0.0}
    let len = LENGTH as f32;
    let width = tables.len();
    let position = if position >= 1.0 {0.99999999999999} else {position};
    let position = position * (width as f32 - 1.0);
    let t1 = position.floor() as usize % width;
    let t2 = (t1 + 1) % width;
    let sig = {
      let x = position.fract();
      T::interpolate(self.table_pos, &tables[t1], LENGTH) * (1.0 - x) +
      T::interpolate(self.table_pos, &tables[t2], LENGTH) * x
    };

    self.table_pos += (len * self.sr_recip * frequency) + (phase * len);
    while self.table_pos as usize > LENGTH { self.table_pos -= len; }
    while self.table_pos < 0.0 { self.table_pos += len; }
    sig
  }

  pub fn set_samplerate(&mut self, samplerate: f32) {
    self.samplerate = samplerate;
    self.sr_recip = 1.0 / samplerate;
  }

  pub fn play_linear<const LENGTH: usize>(&mut self, tables: &[[f32; LENGTH]], frequency: f32, position: f32, phase: f32) -> f32 {
    if frequency > self.samplerate * 0.5 {return 0.0}
    let len = LENGTH as f32;
    let width = tables.len();

    let position = if position >= 1.0 {0.99999999999999} else {position};
    let position = position * (width as f32 - 1.0);
    let table1 = position.floor() as usize % width;
    let table2 = (table1 + 1) % width;

    let y = position.fract();
    let x = self.table_pos.fract();
    let n = self.table_pos.floor() as usize;
    let m = n + 1;
    let a = tables[table1][n];
    let b = tables[table1][m];
    let c = tables[table2][n];
    let d = tables[table2][m];
    let diff1 = b - a;
    let diff2 = x*(diff1 - d + c);
    let sig = a + x * diff1 + y * (c - a * diff2);
    self.table_pos += (len * self.sr_recip * frequency) + (phase * len);
    while self.table_pos as usize > LENGTH { self.table_pos -= len; }
    while self.table_pos < 0.0 { self.table_pos += len; }
    sig
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::waveshape::traits::Waveshape;
  use crate::interpolation::Linear;

  #[test]
  fn one_table() {
    const SIZE: usize = 512;
    let tables = [[0.0; SIZE].sine()];
    let mut vc = VectorOscillator::new(48000.0);
    let mut shape = vec!();
    for i in 0..16 {
      shape.push(vc.play::<SIZE, Linear>(&tables, 1.0/(i as f32 + 1.0), 20.0, 1.0));
    }

    assert_eq!(16, shape.len())
  }
}
