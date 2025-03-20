use super::*;

/// Wavetable oscillator that owns its table of the wave representation.
///
/// Fast and reliable but clones the table on init. This seems to be key to
/// the performance of it, as the table does not lie behind several pointer
/// references.
pub struct Wavetable<const N:usize> {
  position: f32,
  table: Vec<f32>,
  size: usize,
  frequency: f32,
  samplerate: f32,
  sr_recip: f32,
}

impl<const N:usize> Clone for Wavetable<N> {
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

impl<const N: usize> Wavetable<N> {
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
