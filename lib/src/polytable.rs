use crate::{
  vector::VectorOscillator,
  wavetable::shared::WaveTable,
  interpolation::Interpolation
};

struct Token<T> {
  voice: T,
  freq: f32,
}

pub struct PolyTable<const VOICES: usize> {
  voices: [Token<WaveTable>; VOICES],
  next: usize,
}

impl<const VOICES: usize> Default for PolyTable<VOICES> {
  fn default() -> Self {
    let voices = std::array::from_fn(|_| {
      Token{
        voice: WaveTable::new(),
        freq: 0.0,
      }
    });
    Self {
      voices,
      next: 0,
    }
  }
}

impl<const VOICES: usize> PolyTable<VOICES> {
  pub fn new() -> Self {
    let voices = std::array::from_fn(|_| {
      Token{
        voice: WaveTable::new(),
        freq: 0.0,
      }
    });
    Self {
      voices,
      next: 0,
    }
  }

  pub fn trigger(&mut self, note: Option<f32>) {
    if let Some(freq) = note {
      unsafe {
        let v = self.voices.get_unchecked_mut(self.next);
        v.freq = freq;
      }
      self.next = (self.next + 1) % VOICES;
    }
  }

  #[inline]
  pub fn play<T: Interpolation, const N: usize>(
    &mut self,
    table: &[f32; N],
    phases: &[f32; VOICES],
    env_func: &mut impl FnMut(f32, usize) -> f32
  ) -> f32 {
    let mut sig = 0.0;
    for (i, v) in self.voices.iter_mut().enumerate() {
      sig += env_func(v.voice.play::<T>(table, v.freq, phases[i]), i);
    }
    sig
  }

  pub fn set_samplerate(&mut self, samplerate: f32) {
    for t in self.voices.iter_mut() {
      t.voice.set_samplerate(samplerate);
    }
  }
}

pub struct PolyVector<const VOICES: usize> {
  voices: [Token<VectorOscillator>; VOICES],
  next: usize,
}

impl<const VOICES: usize> PolyVector<VOICES> {
  pub fn new(samplerate: f32) -> Self {
    let voices = std::array::from_fn(|_| {
      Token{
        voice: VectorOscillator::new(samplerate),
        freq: 0.0,
      }
    });
    Self { voices, next: 0 }
  }
  
  pub fn trigger(&mut self, note: Option<f32>) {
    if let Some(freq) = note {
      unsafe {
        let v = self.voices.get_unchecked_mut(self.next);
        v.freq = freq;
      }
      self.next = (self.next + 1) % VOICES;
    }
  }

  pub fn play<const LENGTH: usize, OscInterpolation>(
    &mut self,
    tables: &[[f32; LENGTH]],
    positions: &[f32; VOICES],
    phases: &[f32; VOICES],
    env_func: &mut impl FnMut(f32, usize) -> f32
  ) -> f32
    where
        OscInterpolation: Interpolation,
  {
    let mut sig = 0.0;
    for (i, v) in self.voices.iter_mut().enumerate() {
      sig += env_func(v.voice.play::<LENGTH, OscInterpolation>( tables, v.freq, positions[i], phases[i]), i);
    }
    sig
  }

  pub fn set_samplerate(&mut self, samplerate: f32) {
    for v in &mut self.voices {
      v.voice.set_samplerate(samplerate);
    }
  }
}
