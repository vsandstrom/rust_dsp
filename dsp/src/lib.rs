
pub mod signal {

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


  pub mod traits {
    use crate::signal::map;
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
  use crate::signal::{map};

  /// Same as map, but for entire buffers. Suitable for normalizing Wavetable buffers.
  pub fn range(values: &mut Vec<f32>, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> &Vec<f32> {
    for i in 0..values.len() {
      map(&mut values[i], in_min, in_max, out_min, out_max);
    }
    values
  }

  pub fn sum(values: &Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for i in 0..values.len() {
      sum += values[i];
    }
    sum
  }
    
  /// Normalizes contents of vec, sum of contents == 1.0
  pub fn normalize(values: &mut Vec<f32>) {
    let x = 1.0 / sum(values);
    for i in 0..values.len() {
      values[i] *= x;
    }
  }

  // Scales the contents of a Vec to be between outmin -> outmax
  pub fn scale(values: &mut Vec<f32>, outmin: f32, outmax: f32) -> &Vec<f32> {
    let mut min = 0.0f32;
    let mut max = 0.0f32;
    for i in 0..values.len() {
      if values[i] < min { min = values[i] };
      if values[i] > max { max = values[i] };
    }
    range(values, min, max, outmin, outmax)
  }


  pub mod traits {
    use crate::{buffer::{range, sum, map}, signal::traits::SignalFloat};
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
        for i in 0..self.len() {
          let temp = self[i].map(in_min, in_max, out_min, out_max);
          self[i] = temp;
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
        for i in 0..self.len() {
          self[i] *= y;
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
  /// Find next pow of two for quick wrap
  pub fn next_pow2(size: usize) -> usize {
    let mut pow: usize = 1;
    while pow < size {pow = pow << 1;}
    pow
  }

  /// Translate midi-number to frequency
  pub fn mtof(midi: u32, tuning: f32) -> f32 {
      tuning * f32::powf(2.0, midi as f32/12.0)
  }

  /// Translate frequency to midi-number
  pub fn ftom(freq: f32, tuning: f32) -> i32 {
      (12f32 * f32::log10(freq / tuning) / f32::log10(2f32)) as i32
  }

  // Translate decibel to linear volume
  #[allow(non_snake_case)]
  pub fn db_to_volume(dB: f32) -> f32 {
      f32::powf(10.0, 0.05*dB)
  }

  // Translate  linear volume to decibel
  pub fn volume_to_db(volume: f32) -> f32 {
      20.0 * f32::log10(volume)
  }
}

#[cfg(test)]
mod tests {
    use crate::buffer::{normalize, sum};
    use crate::signal::{ 
      unipolar, map, clamp,
      traits::SignalFloat 
    };
    use crate::buffer::{
      range, scale,
      traits::SignalVector
    };
    use crate::math::mtof;
    use crate::math::ftom;

    #[test]
    fn clamp_test() {
        let sample:f32 = -1.0;
        assert_eq!(0.0f32, clamp(sample, 0.0, 1.0));
    }

    #[test]
    fn clamp_trait_test() {
        let sample:f32 = -1.0;
        assert_eq!(0.0f32, sample.clamp(0.0, 1.0));
    }
    
    #[test]
    fn clamp2_test() {
        let sample:f32 = 2.0;
        assert_eq!(1.0f32, clamp(sample, 0.0, 1.0));
    }

    #[test]
    fn clamp_trait2_test() {
        let sample:f32 = 2.0;
        assert_eq!(1.0f32, sample.clamp(0.0, 1.0));
    }

    #[test]
    fn unipolar_test() {
        let sample:f32 = 0.0;
        assert_eq!(0.5f32, unipolar(sample));
    }

    #[test]
    fn unipolar_trait_test() {
        let sample:f32 = 0.0;
        assert_eq!(0.5f32, sample.unipolar());
    }

    #[test]
    fn map_test() {
        let mut signal: f32 = 0.0;
        assert_eq!(0.5f32, map(&mut signal, -1.0, 1.0, 0.0, 1.0))
    }
    
    #[test]
    fn map_trait_test() {
        let signal: f32 = 0.0;
        assert_eq!(0.25f32, signal.map(-1.0, 1.0, -0.5, 1.0))
    }

    #[test]
    fn range_test() {
      let mut vec = vec![0.0, 1.0, 0.0];
      vec = range(&mut vec, 0.0, 1.0, 0.0, 0.5).to_vec();
      println!("{:?}", vec);
      assert_eq!(0.5, vec[1]);
    }

    #[test]
    fn range_trait_test() {
      let vec = vec![0.0, 1.0, 0.0].range(0.0, 1.0, 0.0, 0.5);
      assert_eq!(0.5, vec[1]);
    }
    
    #[test]
    fn normalize_test() {
      let mut vec = vec![0.0, 1.0, 8.0];
      normalize(&mut vec);
      println!("{:?}", vec);
      assert_eq!(1.0/9.0 * 1.0 , vec[1]);
      assert_eq!(1.0/9.0 * 8.0, vec[2]);
    }
    
    #[test]
    fn normalize_test2() {
      let mut vec = vec![0.0, 3.0, 18.0];
      normalize(&mut vec);
      println!("{:?}", vec);
      assert_eq!(1.0/21.0 * 3.0, vec[1]);
      assert_eq!(1.0/21.0 * 18.0, vec[2]);
    }

    #[test]
    fn normalize_trait_test() {
      let vec = vec![0.0, 1.0, 8.0].normalize();
      assert_eq!(1.0/9.0 * 8.0, vec[2]);
    }

    #[test]
    fn normalize_trait_test2() {
      let vec = vec![-2.0, 4.0, 20.0].normalize();
      println!("{:?}", vec);
      assert_eq!(1.0/22.0 * 20.0, vec[2]);
    }

    #[test]
    fn normalize_sum_test() {
      let mut vec = vec![0.0, 3.0, 18.0];
      normalize(&mut vec);
      let sum = sum(&vec);
      assert_eq!(1.0, sum)
    }
    
    #[test]
    fn normalize_sum_trait_test() {
      let vec = vec![0.0, 3.0, 18.0].normalize();
      let sum = &vec.sum();
      assert_eq!(1.0, *sum)
    }
    
    #[test]
    fn scale_test() {
      let mut vec = vec![0.0, 3.0, 18.0];
      scale(&mut vec, 0.0, 1.0);
      assert_eq!(1.0, vec[2])
    }
    
    #[test]
    fn scale_trait_test() {
      let vec = vec![0.0, 3.0, 18.0].normalize().scale(0.0, 1.0);
      assert_eq!(1.0, vec[2])
    }

    #[test]
    fn scale_test2() {
      let mut vec = vec![0.0, 3.0, 18.0];
      scale(&mut vec, 0.5, 1.0);
      assert_eq!(0.5, vec[0])
    }
    
    #[test]
    fn scale_trait_test2() {
      let vec = vec![0.0, 3.0, 18.0].normalize().scale(0.5, 1.0);
      assert_eq!(0.5, vec[0])
    }

    #[test]
    fn midi_to_frequency_test() {
        let midi = 12;
        assert_eq!(880f32, mtof(midi, 440f32))
    }
    
    #[test]
    /// frequencies are a bit skewed, towards equal temperment
    fn midi_to_frequency2_test() {
        let midi = 19;
        assert_eq!(1318.5103f32, mtof(midi, 440f32))
    }

    #[test]
    fn frequency_to_midi_test() {
        let freq = 880f32;
        assert_eq!(12, ftom(freq, 440f32))
    }
    
    #[test]
    /// frequencies are a bit skewed, towards equal temperment
    fn frequency_to_midi2_test() { let freq = 1318.5103f32;
        assert_eq!(19, ftom(freq, 440f32))
    }
}
