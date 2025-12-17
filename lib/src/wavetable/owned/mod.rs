use super::*;

/// Wavetable oscillator that owns its table of the wave representation.
///
/// Fast and reliable but clones the table on init. This seems to be key to
/// the performance of it, as the table does not lie behind several pointer
/// references.
pub struct Wavetable<'a, const N:usize> {
  position: f32,
  table: &'a [f32; N],
  samplerate: u32,
  sr_recip: f32,
}

impl<'a, const N:usize> Clone for Wavetable<'a, N> {
  fn clone(&self) -> Self {
    Self {
      position: self.position,
      table: self.table,
      samplerate: self.samplerate,
      sr_recip: self.sr_recip,
    }
  }
}

impl<'a, const N: usize> Wavetable<'a, N> {
  pub fn new(table: &'a [f32; N], samplerate: u32) -> Self {
    Self { 
      position: 0.0, 
      table,
      samplerate,
      sr_recip: 1.0 / samplerate as f32,
    } 
  }

  #[inline]
  pub fn play<T: Interpolation>(&mut self, frequency: f32, phase: f32) -> f32 {
    let len = self.table.len() as f32;
    self.position += len * self.sr_recip * frequency;
    if self.position > len { self.position -= len; }
    let mut pos = self.position + (phase * len);
    while pos > len { pos -= len; }
    while pos < 0.0 { pos += len; }
    T::interpolate(pos, self.table, self.table.len())
  }

  pub fn set_samplerate(&mut self, samplerate: u32) {
    self.samplerate = samplerate;
    self.sr_recip = 1.0 / samplerate as f32;
  }

}
