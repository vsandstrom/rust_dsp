use crate::{
  vector::VectorOscillator,
  wavetable::WaveTable,
  envelope::Envelope,
  interpolation::Interpolation
};

struct Token<T> {
  voice: T,
  freq: f32,
  env_pos: f32,
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
        env_pos: 0.0
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
        env_pos: 0.0
      }
    });
    Self {
      voices,
      next: 0,
    }
  }

  #[inline]
  pub fn play<T: Interpolation, U: Interpolation, const N: usize>(
    &mut self,
    table: &[f32; N],
    note: Option<f32>,
    phases: &[f32; VOICES],
    env: &Envelope
  ) -> f32 {
    let mut sig = 0.0;
    if let Some(freq) = note {
      unsafe {
        let v = self.voices.get_unchecked_mut(self.next);
        v.freq = freq;
        v.env_pos = 0.0;
      }
      self.next = (self.next + 1) % VOICES;
    }

    for (i, v) in self.voices.iter_mut().enumerate() {
      if (v.env_pos as usize) < env.len() {
        sig += v.voice.play::<N, T>(table, v.freq, phases[i]) * env.read::<U>(v.env_pos);
        v.env_pos += 1.0;
      }
    }
    sig
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
        env_pos: 0.0,
      }
      });
    Self {
      voices,
      next: 0
    }
  }

  pub fn play<const LENGTH: usize, OscInterpolation, EnvInterpolation>(
    &mut self,
    note: Option<f32>,
    tables: &[[f32; LENGTH]],
    env: &Envelope,
    positions: &[f32; VOICES],
    phases: &[f32; VOICES]
  ) -> f32
    where
        OscInterpolation: Interpolation,
        EnvInterpolation: Interpolation
  {
    let mut sig = 0.0;
    if let Some(freq) = note {
      unsafe {
        let v = self.voices.get_unchecked_mut(self.next);
        v.freq = freq;
        v.env_pos = 0.0;
      }
      self.next = (self.next + 1) % VOICES;
    }

    for (i, v) in self.voices.iter_mut().enumerate() {
      if (v.env_pos as usize) < env.len() {
        sig += v.voice.play::<LENGTH, OscInterpolation>(
          tables,
          v.freq,
          positions[i],
          phases[i]
        ) * env.read::<EnvInterpolation>(
          v.env_pos
        );
        v.env_pos += 1.0;
      }
    }
    sig
  }
}
