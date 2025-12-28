use super::*;

/// Wavetable oscillator that owns its table of the wave representation.
///
/// Fast and reliable but clones the table on init. This seems to be key to
/// the performance of it, as the table does not lie behind several pointer
/// references.
pub struct Wavetable {
  position: f32,
  table: Vec<f32>,
  samplerate: u32,
  sr_recip: f32,
}

impl Clone for Wavetable {
  fn clone(&self) -> Self {
    Self {
      position: self.position,
      table: self.table.clone(),
      samplerate: self.samplerate,
      sr_recip: self.sr_recip,
    }
  }
}

impl Wavetable {
  pub fn new<'a, const N: usize>(table: &'a [f32; N], samplerate: u32) -> Self {
    Self { 
      position: 0.0, 
      table: table.to_vec(),
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
    T::interpolate(pos, &self.table, self.table.len())
  }

  pub fn set_samplerate(&mut self, samplerate: u32) {
    self.samplerate = samplerate;
    self.sr_recip = 1.0 / samplerate as f32;
  }

}
