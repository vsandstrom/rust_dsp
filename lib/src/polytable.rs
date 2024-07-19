use std::sync::{Arc, RwLock};
use crate::interpolation::Interpolation;
use crate::envelope::Envelope;

pub mod table {
  use crate::wavetable::shared::WaveTable;
  use super::{ Interpolation, Arc, RwLock, Envelope };
  /// Polysynth using only wavetables 
  pub struct PolyTable<const VOICES: usize> {
    voices: [WaveTable; VOICES],
    table: Arc<RwLock<Vec<f32>>>,
    frequencies: [f32; VOICES],
    env_positions: [f32; VOICES],
    next_voice: usize,
    envelope: Envelope,
  }

  impl<const VOICES: usize> PolyTable<VOICES> {
    pub fn new<const TABLESIZE: usize>(samplerate: f32) -> Self {
      let table = Arc::new(RwLock::new([0.0; TABLESIZE].to_vec()));
      let voices = std::array::from_fn(|_| { WaveTable::new(table.clone(), samplerate) });
      let envelope = Envelope::default();
      Self {
        voices,
        table,
        frequencies: [0.0; VOICES],
        next_voice: 0,
        envelope,
        env_positions: [0.0; VOICES]
      }
    }

    #[inline]
    pub fn play<T: Interpolation, U: Interpolation>(&mut self, note: Option<f32>, phases: &[f32;VOICES]) -> f32 {
      let mut sig = 0.0;
      if let Some(freq) = note {
        self.frequencies[self.next_voice] = freq;
        self.env_positions[self.next_voice] = 0.0;
        self.next_voice = (self.next_voice+1) % VOICES;
      }

      for i in 0..VOICES {
        if (self.env_positions[i] as usize) < self.envelope.len() {
          sig += self.voices[i].play::<T>(self.frequencies[i], phases[i]) * 
            self.envelope.read::<U>(self.env_positions[i]);
          self.env_positions[i] += 1.0;
        } 
      }
      sig
    }

    pub fn update_table(&mut self, sample: f32, index: usize) -> Result<(), &'static str> {
      if let Ok(mut inner_table) = self.table.try_write() {
        if index >= inner_table.len() {
          return Err("index out of bounds");
        }

        inner_table[index] = sample;
      }

      Ok(())

    }

    pub fn change_table(&mut self, table: Vec<f32>) -> Result<(), &'static str> {
      if let Ok(mut inner_table) = self.table.try_write() {
        if inner_table.len() != table.len() {
          return Err("wavetable sizes don't match")
        }
        *inner_table = table;
      } else {
        return Err("could not write-lock shared table")
      }

      Ok(())
    }
  }
}


pub mod vector {
  use crate::envelope::{EnvType, Envelope};
  use crate::vector::VectorOscillator;
  use std::sync::{Arc, RwLock};
  use crate::interpolation::Interpolation;

  pub struct PolyVector<const VOICES: usize, const TABLESIZE: usize> {
    voices: [VectorOscillator<TABLESIZE>; VOICES],
    frequencies: [f32; VOICES],
    next_voice: usize,
    envelope: Envelope,
    env_positions: [f32; VOICES],
    samplerate: f32
  }

  impl<const VOICES: usize, const TABLESIZE: usize> PolyVector<VOICES, TABLESIZE> {
    pub fn new(tables: Arc<RwLock<Vec<[f32; TABLESIZE]>>>, samplerate: f32) -> Self {
      let voices = std::array::from_fn(|_| {
        VectorOscillator::new(tables.clone(), samplerate)
      });
      Self {
        voices,
        envelope: Envelope::default(),
        next_voice: 0,
        frequencies: [0.0; VOICES],
        env_positions: [0.0; VOICES],
        samplerate
      }
    }

    #[inline]
    pub fn play<T, U>(&mut self, note: Option<f32>, positions: &[f32; VOICES], phases: &[f32;VOICES]) -> f32 
      where 
          T: Interpolation,
          U: Interpolation
    {
      let mut sig = 0.0;
      if let Some(freq) = note {
        self.frequencies[self.next_voice] = freq;
        self.env_positions[self.next_voice] = 0.0;
        self.next_voice = (self.next_voice+1) % VOICES;
      }
      for i in 0..VOICES {
        if (self.env_positions[i] as usize) < self.envelope.len() {
          sig += self.voices[i].play::<T>(self.frequencies[i], positions[i], phases[i]) * 
            self.envelope.read::<U>(self.env_positions[i]);
          self.env_positions[i] += 1.0;
        } 
      }
      sig
    }

    #[inline]
    pub fn set_samplerate(&mut self, samplerate: f32) {
      self.samplerate = samplerate;
      for v in self.voices.iter_mut() {
        v.set_samplerate(samplerate)
      }
    }

    // pub fn update_table(&mut self, sample: f32, voice_index: usize, table_index: usize) -> Result<(), &'static str>{
    //   todo!()
    // }

    #[inline]
    pub fn update_envelope<const N:usize, const M:usize>(&mut self, shape: &EnvType<N, M>) {
      self.envelope.new_shape(shape, self.samplerate)
    }
  }
}

pub mod simple {
    use std::marker::PhantomData;

    use crate::{envelope::Envelope, interpolation::Interpolation, vector::simple, wavetable::simple::WaveTable};

  pub struct PolyTable<const VOICES: usize> {
    voices: [WaveTable; VOICES],
    freqs: [f32; VOICES],
    env_pos: [f32; VOICES],
    next: usize,
  }

  impl<const VOICES: usize> PolyTable<VOICES> {
    pub fn new() -> Self {
      let voices = std::array::from_fn(|_| { WaveTable::new() });
      Self {
        voices,
        freqs: [0.0; VOICES],
        env_pos: [0.0; VOICES],
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
        self.freqs[self.next] = freq;
        self.env_pos[self.next] = 0.0;
        self.next = (self.next + 1) % VOICES;
      }

      for i in 0..VOICES {
        if (self.env_pos[i] as usize) < env.len() {
          sig += self.voices[i].play::<N, T>(table, self.freqs[i], phases[i]) * env.read::<U>(self.env_pos[i]);
          self.env_pos[i] += 1.0;
        }
      }
      sig
    }
  }

  pub struct PolyVector<const VOICES: usize> {
    voices: [simple::VectorOscillator; VOICES],
    freqs: [f32; VOICES],
    next: usize,
    env_pos: [f32; VOICES],
    // samplerate: f32,
    // sr_recip: f32
  }

  impl<const VOICES: usize> PolyVector<VOICES> {
    pub fn new(samplerate: f32) -> Self {
      let voices = std::array::from_fn(|_| {simple::VectorOscillator::new(samplerate)});
      Self {
        voices,
        freqs: [0.0; VOICES],
        env_pos: [0.0; VOICES],
        next: 0
      }
    }

    pub fn play<OscInterpolation, EnvInterpolation, const WIDTH: usize, const LENGTH: usize>(
      &mut self,
      note: Option<f32>,
      tables: &[[f32; LENGTH]; WIDTH],
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
        self.freqs[self.next] = freq;
        self.env_pos[self.next] = 0.0;
        self.next = (self.next + 1) % VOICES;
      }

      for i in 0..VOICES {
        if (self.env_pos[i] as usize) < env.len() {
          sig += self.voices[i].play::<OscInterpolation, WIDTH, LENGTH>(
            tables,
            self.freqs[i],
            positions[i],
            phases[i]
          ) * env.read::<EnvInterpolation>(
            self.env_pos[i]
          );
          self.env_pos[i] += 1.0;
        }
      }
      sig
    }
  }
}


