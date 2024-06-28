use interpolation::interpolation::InterpolationConst;
use std::{f32::consts::TAU, sync::{Arc, RwLock}};
use dsp::signal::clamp;

/// Vector Oscillator using an vector of float-arrays to traverse smoothly 
/// through different textures. Currently only using a hardcoded Linear 
/// interpolation between tables, but values within tables are able to use
/// all interpolation methods in the interpolation crate
pub struct VectorOscillator<const TABLESIZE: usize> {
  tables: Arc<RwLock<Vec<[f32; TABLESIZE]>>>,
  table_pos: f32,
  samplerate: f32,
  size: usize,
}

impl<const TABLESIZE:usize> VectorOscillator<TABLESIZE> {
  /// Create a 1D Vector Oscillator
  pub fn new(tables: Arc<RwLock<Vec<[f32; TABLESIZE]>>>, samplerate:f32) -> Self {
    let size = tables.try_read().unwrap().len();
    VectorOscillator { 
      tables,
      table_pos: 0.0,
      samplerate,
      size,
    }
  }

  pub fn set_samplerate(&mut self, samplerate:f32) {
    self.samplerate = samplerate;
  }

  /// Position is a value between 0.0 -> 1.0, scrolls through wavetables
  /// Frequency and phase are passed to each of the wavetable objects.
  pub fn play<TableInterpolation>(&mut self, frequency: f32, position: f32, phase: f32) -> f32 
  where 
      TableInterpolation: InterpolationConst
  {
    if frequency > (self.samplerate) { return 0.0; }
    let n_f32 = TABLESIZE as f32;
    // POSITION MUST NEVER REACH 1.0! (will only wrap around momentarily but sounds bad)
    let position = if position >= 1.0 { 0.99999999999999 } else {position};
    let position = position * (self.size as f32 - 1.0);
    let table_1 = position.floor() as usize % self.size;
    let table_2 = position.ceil() as usize % self.size;

    let out = {
      if let Ok(tables) = self.tables.try_read() {
        let x = position.fract();
        TableInterpolation::interpolate(self.table_pos, &tables[table_1], TABLESIZE) * (1.0 - x) +
        TableInterpolation::interpolate(self.table_pos, &tables[table_2], TABLESIZE) * x
      } else {
        0.0
      }
    };

    self.table_pos += n_f32 / (self.samplerate / frequency) + phase;

    while self.table_pos > n_f32 {
      self.table_pos -= n_f32;
    }
    
    while self.table_pos < -n_f32 {
      self.table_pos += n_f32;
    }

    out
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use interpolation::interpolation::Linear;
  use waveshape::traits::Waveshape;

  #[test]
  fn one_table() {
    let tables = Arc::new(RwLock::new([
      [0.0; 512].sine().to_owned(),
    ].to_vec()));
    let mut vc = VectorOscillator::new(tables, 48000.0);

    let mut shape = vec!();

    for i in 0..16 {
      shape.push(vc.play::<Linear>(1.0/(i as f32 + 1.0), 20.0, 1.0));
    }

    assert_eq!(16, shape.len())
  }

  #[test]
  fn division_by_zero() {
    assert!((0.0f32 / 0.0).is_infinite())
  }



}
