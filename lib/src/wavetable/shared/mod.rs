use super::*;

/// Wavetable that shares the table containing the wave representation.
///
/// Performance lies between owned and Arc<RwLock>, and is preferred when
/// trying to keep allocated data at a minimum. Beware of stack overflow when
/// creating too many big arrays. 
#[derive(Clone, Copy, Default, Debug)]
pub struct Wavetable {
  position: f32,
  samplerate: f32,
  sr_recip: f32,
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
  pub fn play<T: Interpolation>(&mut self, table: &[f32], frequency: f32, phase: f32) -> f32 {
    let len = table.len() as f32;
    // increment phase position in table
    self.position += len * self.sr_recip * frequency;
    if self.position > len { self.position -= len; }
    // add FM (phase modulation)
    let mut pos = self.position + (phase * len);
    while pos > len { pos -= len; }
    while pos < 0.0 { pos += len; }
    T::interpolate(pos, table, table.len())
  }

  pub fn set_samplerate(&mut self, samplerate: f32) {
    self.samplerate = samplerate;
    self.sr_recip = 1.0 / samplerate;
  }
}
