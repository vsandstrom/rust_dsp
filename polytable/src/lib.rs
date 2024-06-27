use std::sync::{Arc, RwLock};
use interpolation::interpolation::{Interpolation, InterpolationConst};
use envelope::Envelope;

// pub mod table {
//   use wavetable::shared::WaveTable;
//   use crate::{ Interpolation, InterpolationConst, Arc, RwLock, Envelope };
//   /// Polysynth using only wavetables 
//   pub struct PolyTable<const VOICES: usize> {
//     voices: [WaveTable; VOICES],
//     table: Arc<RwLock<Vec<f32>>>,
//     frequencies: [f32; VOICES],
//     env_positions: [f32; VOICES],
//     next_voice: usize,
//     envelope: Envelope,
//   }
//
//   impl<const VOICES: usize> PolyTable<VOICES> {
//     pub fn new<const TABLESIZE: usize>(samplerate: f32) -> Self {
//       let table = Arc::new(RwLock::new([0.0; TABLESIZE].to_vec()));
//       let voices = std::array::from_fn(|_| { WaveTable::new(table.clone(), samplerate) });
//       let envelope = Envelope::default();
//       Self {
//         voices,
//         table,
//         frequencies: [0.0; VOICES],
//         next_voice: 0,
//         envelope,
//         env_positions: [0.0; VOICES]
//       }
//     }
//
//     pub fn play<T: InterpolationConst, U: Interpolation>(&mut self, note: Option<f32>, phase: &[f32;VOICES]) -> f32 {
//       let mut sig = 0.0;
//       for (i, v) in self.voices.iter_mut().enumerate() {
//         if let Some(freq) = note { 
//           if i == self.next_voice {
//             v.frequency = freq;
//             sig += v.play::<T>(freq, phase[i]) * self.envelope.read::<U>(0.0) ; 
//             self.env_positions[i] = 1.0;
//             self.next_voice = (self.next_voice + 1) % VOICES;
//           }
//         } else {
//           if (self.env_positions[i] as usize) + 1 < self.envelope.len() {
//             sig += v.play::<T>(self.frequencies[i], phase[i]) * self.envelope.read::<U>(self.env_positions[i]);
//             self.env_positions[i] += 1.0;
//           }
//         }
//       }
//       sig
//     }
//
//     pub fn update_table(&mut self, sample: f32, index: usize) -> Result<(), &'static str> {
//       if let Ok(mut inner_table) = self.table.try_write() {
//         if index >= inner_table.len() {
//           return Err("index out of bounds");
//         }
//
//         inner_table[index] = sample;
//       }
//
//       Ok(())
//
//     }
//
//     pub fn change_table(&mut self, table: Vec<f32>) -> Result<(), &'static str> {
//       if let Ok(mut inner_table) = self.table.try_write() {
//         if inner_table.len() != table.len() {
//           return Err("wavetable sizes don't match")
//         }
//         *inner_table = table;
//       } else {
//         return Err("could not write-lock shared table")
//       }
//
//       Ok(())
//     }
//   }
// }
//
pub mod vector {
  use envelope::{BreakPoints, Envelope};
  use vector::VectorOscillator;
  use std::sync::{Arc, RwLock};
  use crate::{InterpolationConst, Interpolation};

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

    // fn idle_play<TableInterpolation, EnvelopeInterpolation>(&mut self, positions: &[f32; VOICES], phases: &[f32;VOICES]) -> f32
    //   where 
    //       TableInterpolation: InterpolationConst,
    //       EnvelopeInterpolation: Interpolation
    // {
    //   let mut sig = 0.0;
    //   for (i, v) in self.voices.iter_mut().enumerate() {
    //     if (self.env_positions[i] as usize) < self.envelope.len() {
    //       sig += v.play::<TableInterpolation>(self.frequencies[i], positions[i], phases[i]) * 
    //         self.envelope.read::<EnvelopeInterpolation>(self.env_positions[i]);
    //       self.env_positions[i] += 1.0;
    //     } 
    //   }
    //   sig
    //
    // }
      
    pub fn play<TableInterpolation, EnvelopeInterpolation>(&mut self, note: Option<f32>, positions: &[f32; VOICES], phases: &[f32;VOICES]) -> f32 
      where 
          TableInterpolation: InterpolationConst,
          EnvelopeInterpolation: Interpolation
    {
      let mut sig = 0.0;
      let mut triggered = false;
      for i in 0..VOICES {
        if let Some(freq) = note { 
          if i == self.next_voice && !triggered  {
            println!("voice {i}");
            self.frequencies[i] = freq;
            self.env_positions[i] = 0.0;
            sig += self.voices[i].play::<TableInterpolation>(freq, positions[i], phases[i]) * self.envelope.read::<EnvelopeInterpolation>(0.0) ; 
            self.next_voice = (self.next_voice + 1) % VOICES;
            triggered = true;
          } else {
            if (self.env_positions[i] as usize) < self.envelope.len() {
              sig += self.voices[i].play::<TableInterpolation>(self.frequencies[i], positions[i], phases[i]) * 
                self.envelope.read::<EnvelopeInterpolation>(self.env_positions[i]);
              self.env_positions[i] += 1.0;
            } 
          }
        } else {
          if (self.env_positions[i] as usize) < self.envelope.len() {
            sig += self.voices[i].play::<TableInterpolation>(self.frequencies[i], positions[i], phases[i]) * 
              self.envelope.read::<EnvelopeInterpolation>(self.env_positions[i]);
            self.env_positions[i] += 1.0;
          } 
        } 
      }
      sig
    }

    pub fn update_envelope<const N:usize, const M:usize>(&mut self, breakpoints: &BreakPoints<N, M>) {
      self.envelope.new_shape(breakpoints, self.samplerate)
      
    }
  }
}


