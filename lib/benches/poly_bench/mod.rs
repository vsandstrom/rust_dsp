use rust_dsp::{
  vector::VectorOscillator,
  wavetable::shared::Wavetable,
  envelope::Envelope,
  interpolation::Interpolation
};

use criterion::Criterion;

// pub fn run_poly();
// pub fn run_poly_arc();

struct Token<T> {
  voice: T,
  freq: f32,
  env_pos: f32,
}

pub struct PolyTable<const VOICES: usize> {
  voices: [Token<Wavetable>; VOICES],
  next: usize,
}

mod old {
  use super::*;
  impl<const VOICES: usize> Default for PolyTable<VOICES> {
    fn default() -> Self {
      let voices = std::array::from_fn(|_| {
        Token{
          voice: Wavetable::new(),
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
          voice: Wavetable::new(),
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
}
