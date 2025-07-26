use rust_dsp::{
  vector::VectorOscillator,
  wavetable::shared::Wavetable,
  envelope::Envelope,
  interpolation::Linear
};

use criterion::Criterion;
use rust_dsp::envelope::{BreakPoints, EnvType};

pub fn run_poly<const VOICES: usize, const N: usize>(pt: &mut PolyTable<VOICES>, table: &[f32; N], sr: f32) -> f32 {
  let mut out = 0.0; 
  let freq = 100.0;
  let env = Envelope::new(
    &EnvType::BreakPoint(
      BreakPoints{
        values: [0.0, 1.0, 0.0],
        durations: [0.1, 1.0],
        curves: None
  }), sr);
  
  for _ in 0..256 {
    out = pt.play::<Linear, Linear, N>(table, Some(freq), &[0.0f32; VOICES], &env)
  };
  out
}

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
  use rust_dsp::interpolation::Interpolation;
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
          sig += v.voice.play::<T>(table, v.freq, phases[i]) * env.read::<U>(v.env_pos);
          v.env_pos += 1.0;
        }
      }
      sig
    }

    pub fn set_samplerate(&mut self, sr: f32) {

      self.voices.iter_mut().for_each(|wt| {
        wt.voice.set_samplerate(sr);
      });
    }
  }
}

pub fn criterion_benchmark_poly_table(c: &mut Criterion) {
  use rust_dsp::waveshape::traits::Waveshape;
  const SIZE: usize = 1<<13;
  let table = [0.0; SIZE].sine();
  let mut pt = PolyTable::<24>::new();
  pt.set_samplerate(48000.0);
  let mut group = c.benchmark_group("poly tables");

  group.bench_function(
    "polytable - wavetable tokens",
    |b| b.iter(|| {run_poly::<24, SIZE>(&mut pt, &table, 48000.0)}) 
  );
}
