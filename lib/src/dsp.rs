#[cfg(feature="no_std")]
extern crate alloc;
// #[cfg(feature="no_std")]

use alloc::vec::Vec;

pub mod signal {
  use core::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_4};

  pub fn clamp(signal: f32, bottom: f32, top: f32 ) -> f32 {
      f32::max(bottom, f32::min(signal, top))
  }

  /// Map a signal of range m -> n into new range, x -> y
  pub fn map(signal: &mut f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    *signal = (out_max - out_min) * (*signal - in_min) / (in_max - in_min) + out_min;
    *signal
  }

  pub fn dcblock(signal: f32, xm1: f32, ym1: f32 ) -> f32 {
      signal - xm1 + 0.995 * ym1
  }
  
  /// Convenience for normalizing a signal to be only positive.
  pub fn unipolar(mut sample: f32) -> f32 {
    map(&mut sample, -1.0, 1.0, 0.0, 1.0)
  }

  /// calculates panning weights for stereo equal power panning.
  pub fn pan_exp2(pan: f32) -> (f32, f32) {
    let p = pan * FRAC_PI_4;
    let c = f32::cos(p);
    let s = f32::sin(p);
    (
      FRAC_1_SQRT_2 * (c + s),
      FRAC_1_SQRT_2 * (c - s)
    )
  }

  pub fn pan_lin2(pan:f32) -> (f32, f32) {
    match pan * 0.5 {
      x if x < 0.0 => {(0.5 + x, 0.5 - x)},
      x            => {(0.5 - x, 0.5 + x)}
    }
  }

  pub mod traits {
    use crate::dsp::signal::map;
    /// DSP specific trait for manipulating samples. For chaining method calls on <f32>
    pub trait SignalFloat {
      fn unipolar(self) -> Self;
      fn dcblock(self, xm1: f32, ym1: f32 ) -> Self;
      fn map(self, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> Self;
      // fn clamp(self, bottom: f32, top: f32 ) -> Self; 
    }

    impl SignalFloat for f32 {
      // clamp exists for f32 already
      // fn clamp(self, bottom: f32, top: f32 ) -> f32 {
      //     f32::max(bottom, f32::min(self, top))
      // }

      /// Map a signal of range m -> n into new range, x -> y
      fn map(mut self, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
          self = (out_max - out_min) * (self - in_min) / (in_max - in_min) + out_min;
          self
      }

      fn dcblock(self, xm1: f32, ym1: f32 ) -> f32 {
          self - xm1 + 0.995 * ym1
      }
      
      /// Convenience for normalizing a signal to be only positive.
      fn unipolar(mut self) -> f32 {
          map(&mut self, -1.0, 1.0, 0.0, 1.0);
          self
      }
    }
  }
}

pub mod buffer {
  use crate::dsp::signal::map;

  /// Same as map, but for entire buffers. Suitable for normalizing Wavetable buffers.
  pub fn range(values: &mut [f32], in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> &[f32] {
    for x in values.iter_mut() {
      map(x, in_min, in_max, out_min, out_max);
    }
    values
  }

  pub fn sum(values: &[f32]) -> f32 {
    let mut sum = 0.0;
    for v in values.iter() {
      sum += v;
    }
    sum
  }
    
  /// Normalizes contents of vec, sum of contents == 1.0
  pub fn normalize(values: &mut [f32]) {
    let x = 1.0 / sum(values);
    for v in values.iter_mut() {
      *v *= x;
    }
  }

  // Scales the contents of a Vec to be between outmin -> outmax
  pub fn scale(values: &mut [f32], outmin: f32, outmax: f32) -> &[f32] {
    let mut min = 0.0f32;
    let mut max = 0.0f32;
    for v in values.iter_mut() {
      if *v < min { min = *v };
      if *v > max { max = *v };
    }
    range(values, min, max, outmin, outmax)
  }


  pub mod traits {
    
    use crate::dsp::Vec;
    use crate::dsp::{buffer::{range, sum}, signal::traits::SignalFloat};
    /// DSP specific trait for manipulating arrays/vectors. 
    /// For chaining method calls Vec<f32>
    pub trait SignalVector {
      fn scale(self, outmin: f32, outmax: f32) -> Self;
      fn normalize(self) -> Self;
      fn sum(&self) -> f32;
      fn range(self, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> Self; 
    }

    impl SignalVector for Vec<f32> {
      fn range(mut self, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> Self {
        for n in self.iter_mut() {
          let temp = n.map(in_min, in_max, out_min, out_max);
          *n = temp;
        }
        self
      }

      fn sum(&self) -> f32 {
        let mut sum = 0.0;
        for x in self {
          sum += x;
        }
        sum
      }
        
      /// Sum of values in vec == 1
      fn normalize(mut self) -> Self {
        let y = 1.0 / sum(&self);
        for x in self.iter_mut() {
          *x *= y;
        }
        self
      }

      // Scales the contents of a Vec to be between outmin -> outmax
      fn scale(mut self, outmin: f32, outmax: f32) -> Self{
        let mut min = 0.0f32;
        let mut max = 0.0f32;
        for x in &self {
          if x < &min { min = *x };
          if x > &max { max = *x };
        }
        range(&mut self, min, max, outmin, outmax);
        self
      }
    }
    
    impl<const N:usize> SignalVector for [f32; N] {
      fn range(mut self, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> Self {
        for x in self.iter_mut() {
          let temp = x.map(in_min, in_max, out_min, out_max);
          *x = temp;
        }
        self
      }

      fn sum(&self) -> f32 {
        let mut sum = 0.0;
        for x in self {
          sum += x;
        }
        sum
      }
        
      /// Sum of values in vec == 1
      fn normalize(mut self) -> Self {
        let y = 1.0 / sum(&self);
        for x in self.iter_mut() {
          *x *= y;
        }
        self
      }

      // Scales the contents of a Vec to be between outmin -> outmax
      fn scale(mut self, outmin: f32, outmax: f32) -> Self{
        let mut min = 0.0f32;
        let mut max = 0.0f32;
        for x in &self {
          if x < &min { min = *x };
          if x > &max { max = *x };
        }
        range(&mut self, min, max, outmin, outmax);
        self
      }
    }
  }
}

pub mod math {
    use core::f32::consts::PI;

  /// Find next pow of two for quick wrap
  #[inline]
  pub const fn next_pow2(size: usize) -> usize {
    let mut pow: usize = 1;
    while pow < size {pow <<= 1;}
    pow
  }

  #[inline]
  pub const fn is_pow2(size: usize) -> bool {
    size != 0 && size & (size-1) == 0 
  }

  /// Translate midi-number to frequency
  #[inline]
  pub fn midi_to_freq(midi: u8, tuning: f32) -> f32 {
    let exp: f32 = (midi - 69) as f32 / 12.0;
    tuning * f32::powf(2.0, exp)
  }

  /// Translate frequency to midi-number
  #[inline]
  pub fn freq_to_midi(freq: f32, tuning: f32) -> u8 {
    ((12.0 * f32::log10(freq / tuning) / f32::log10(2f32)) + 69.0).round() as u8
  }

  /// Translate midi-number to playback rate
  pub fn midi_to_rate(midi: u8) -> f32 {
    f32::powf(2.0, (midi as f32 - 36.0) / 12.0)
  }

  pub fn hz_to_radian(hz: f32, samplerate: f32) -> f32 {
    2.0 * PI * hz * (1.0 / samplerate)
  }



  // Translate decibel to linear volume
  #[allow(non_snake_case)]
  #[inline]
  pub fn db_to_volume(dB: f32) -> f32 {
    f32::powf(10.0, 0.05*dB)
  }

  // Translate  linear volume to decibel
  #[inline]
  pub fn volume_to_db(volume: f32) -> f32 {
      20.0 * f32::log10(volume)
  }

  #[inline]
  pub fn samples_to_wavelength(samples: usize, samplerate: f32) -> f32 {
    (343.0 / samplerate) * samples as f32
  }

  #[inline]
  pub fn wavelength_to_samples(wavelength: f32, samplerate: f32) -> usize {
    (samplerate / (343.0 / wavelength)) as usize
  }
}

#[cfg(test)]
mod test {
  use crate::dsp::signal::pan_exp2;

  #[test]
  fn pan_center() {
    assert_eq!(core::f32::consts::FRAC_1_SQRT_2, pan_exp2(0.0).0);
    assert_eq!(core::f32::consts::FRAC_1_SQRT_2, pan_exp2(0.0).1);
  }

  #[test]
  fn pan_left() {
    assert!((pan_exp2(1.0).0 - 1.0).abs() < f32::EPSILON);
    assert_eq!(0.0, pan_exp2(1.0).1);
  }

  #[test]
  fn pan_right() {
    assert_eq!(0.0, pan_exp2(-1.0).0);
    assert!((pan_exp2(-1.0).1 - 1.0).abs() < f32::EPSILON);
  }
}

