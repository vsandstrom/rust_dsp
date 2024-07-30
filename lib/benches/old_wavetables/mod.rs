use std::sync::{Arc, RwLock};
use rust_dsp::{
  interpolation::Interpolation,
  dsp::signal::clamp
};


  /// Shared refers to the ownership of the underlying wavetable structure.
  /// In the `shared` module, the table can be shared between the instances
  /// of the WaveTable struct over threads. The changes to the underlying 
  /// wavetable propagates through the shared references.

pub struct ArcWaveTable{
    position: f32,
    table: Arc<RwLock<Vec<f32>>>,
    size: usize,
    frequency: f32,
    samplerate: f32
  }

  impl ArcWaveTable {
    pub fn new(table: Arc<RwLock<Vec<f32>>>, samplerate: f32) -> ArcWaveTable {
      let size = table.try_read().unwrap().len();
      ArcWaveTable{
        position: 0.0,
        table,
        size,
        frequency: 0.0,
        samplerate
      }
    }

    #[inline]
    pub fn play<T: Interpolation>(&mut self, frequency: f32, phase: f32) -> f32 {
      if frequency > (self.samplerate / 2.0) { return 0.0; }
      self.frequency = frequency;
      let norm_ph = clamp((phase+1.0)*0.5, 0.0, 1.0);
      self.position += self.size as f32 / (self.samplerate / (frequency * norm_ph));
      while self.position > self.size as f32 {
        self.position -= self.size as f32;
      }
      if let Ok(table) = &self.table.try_read() {
        T::interpolate(self.position, table.as_ref(), self.size)
      } else {
        0.0
      }
    }
  }


  /// Single refers to the ownership of the underlying wavetable structure.
  /// In the `single` module, the table is always owned by the instance of 
  /// the WaveTable struct. This is useful when designing a VectorSynth with 
  /// the ability to scroll between different wavetables seamlessly.


  pub struct OwnWaveTable<const N:usize> {
    position: f32,
    table: Vec<f32>,
    size: usize,
    frequency: f32,
    samplerate: f32,
  }

  impl<const N:usize> Clone for OwnWaveTable<N> {
    fn clone(&self) -> Self {
      Self {
        position: self.position,
        table: self.table.clone(),
        size: self.size,
        frequency: self.frequency,
        samplerate: self.samplerate,
      }
    }
  }
    
  impl<const N: usize> OwnWaveTable<N> {
    pub fn new(table: &[f32; N], samplerate: f32) -> Self {
      Self { 
        position: 0.0, 
        table: table.to_vec(),
        size: table.len(),
        frequency: 0.0,
        samplerate,
      } 
    }

    #[inline]
    pub fn play<T: Interpolation>(&mut self, frequency: f32, phase: f32) -> f32 {
      if frequency > (self.samplerate / 2.0) { return 0.0; }
      self.frequency = frequency;
      let len = self.size;

      self.position += len as f32 / (self.samplerate /  frequency) + (phase * len as f32);
      while self.position > self.size as f32 {
        self.position -= self.size as f32;
      }
      T::interpolate(self.position, &self.table, self.table.len())
    }

  }
