use super::*;

/// Wavetable that shares the table containing the wave representation.
///
/// Performance of the `shared` Wavetable implementation lies in between
/// an owned [`[f32; n]`] as table and an [`Arc<RwLock<Vec<f32>>>`] as table, 
/// and is preferred when trying to keep allocated data at a minimum.
/// !Beware of stack overflow when creating too many big arrays. 
#[derive(Clone, Copy)]
pub struct Wavetable {
  position: f32,
  sr_recip: f32,
}

impl Default for Wavetable {
   fn default() -> Self {
    Self {
      position: 0.0,
      sr_recip: 0.0,
    }
  }
}

impl Wavetable {
  pub fn new() -> Self {
    Self {
      position: 0.0,
      sr_recip: 0.0,
    }
  }

  /// Play function for wavetable where __SIZE__ is the table size and __TableInterpolation = &impl Interpolation__
  #[inline]
  pub fn play<T>(&mut self, table: &[f32], frequency: f32, phase: f32) -> f32
    where T: Interpolation
  {
    debug_assert!(self.sr_recip > f32::EPSILON, "samplerate has not been set");
    let len = table.len() as f32;
    self.position += len * (self.sr_recip * frequency + phase);
    while self.position > len { self.position -= len; }
    T::interpolate(self.position, table, table.len())
  }

  pub fn set_samplerate(&mut self, samplerate: f32) {
    self.sr_recip = 1.0 / samplerate;
  }
}
