use super::*;

/// Wavetable that shares the table containing the wave representation.
///
/// Performance lies between owned and Arc<RwLock>, and is preferred when
/// trying to keep allocated data at a minimum. Beware of stack overflow when
/// creating too many big arrays. 
pub struct Wavetable {
  position: f32,
  samplerate: f32,
  sr_recip: f32,
}

impl From<f32> for Wavetable {
  /// Create a wavetable instance using the samplerate value
  fn from(samplerate: f32) -> Self {
    Self{
      position: 0.0,
      samplerate,
      sr_recip: 1.0/samplerate
    }
  }
}

impl Default for Wavetable {
   fn default() -> Self {
    Self {
      position: 0.0,
      samplerate: 0.0,
      sr_recip: 0.0,
    }
  }
}

impl Wavetable {
  pub fn new() -> Self {
    Self {
      position: 0.0,
      samplerate: 0.0,
      sr_recip: 0.0,
    }
  }

  /// Play function for wavetable where __SIZE__ is the table size and __TableInterpolation = &impl Interpolation__
  #[inline]
  pub fn play<TableInterpolation>(&mut self, table: &[f32], frequency: f32, phase: f32) -> f32
    where
        TableInterpolation: Interpolation
  {
    debug_assert!(self.samplerate > f32::EPSILON, "samplerate has not been set");
    if frequency > self.samplerate * 0.5 { return 0.0; }
    let len = table.len() as f32;
    self.position += (len * self.sr_recip * frequency) + (phase * len);
    while self.position > len { self.position -= len; }
    TableInterpolation::interpolate(self.position, table, table.len())
  }

  pub fn set_samplerate(&mut self, samplerate: f32) {
    self.samplerate = samplerate;
    self.sr_recip = 1.0 / samplerate;
  }
}
