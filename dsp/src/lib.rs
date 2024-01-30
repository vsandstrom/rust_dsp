
pub mod signal {
  pub fn clamp(signal: f32, bottom: f32, top: f32 ) -> f32 {
      f32::max(bottom, f32::min(signal, top))
  }

  /// Map a signal of range m -> n into new range, x -> y
  pub fn map(signal: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
      (out_max - out_min) * (signal - in_min) / (in_max - in_min) + out_min
  }

  pub fn dcblock(signal: f32, xm1: f32, ym1: f32 ) -> f32 {
      signal - xm1 + 0.995 * ym1
  }
  
  /// Convenience for normalizing a signal to be only positive.
  pub fn unipolar(sample: f32) -> f32{
      map(sample, -1.0, 1.0, 0.0, 1.0)
  }

}

pub mod buffer {
  use crate::signal::map;
  /// Same as map, but for entire buffers. Suitable for normalizing Wavetable buffers.
  pub fn range(values: &mut Vec<f32>, in_min: f32, in_max: f32, out_min: f32, out_max: f32) {
    for i in 0..values.len() {
      map(values[i], in_min, in_max, out_min, out_max);
    }
  }

  pub fn sum(values: &Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for i in 0..values.len() {
      sum += values[i];
    }
    sum
  }
    
  pub fn normalize(values: &mut Vec<f32>) {
    let x = 1.0 / sum(values);
    for i in 0..values.len() {
      values[i] *= x;
    }
  }

  pub fn scale(values: &mut Vec<f32>, outmin: f32, outmax: f32) {
    let mut min = 0.0f32;
    let mut max = 0.0f32;
    for i in 0..values.len() {
      if values[i] < min { min = values[i] };
      if values[i] > max { max = values[i] };
    }
    range(values, min, max, outmin, outmax)
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
    use crate::signal::unipolar;
    use crate::signal::map;
    use crate::math::mtof;
    use crate::math::ftom;

    #[test]
    fn test_unipolar() {
        let sample:f32 = 0.0;
        assert_eq!(0.5f32, unipolar(sample));
    }

    #[test]
    fn test_map() {
        let signal: f32 = 0.0;
        assert_eq!(0.5f32, map(signal, -1.0, 1.0, 0.0, 1.0))
    }
    
    #[test]
    fn test_map2() {
        let signal: f32 = 0.0;
        assert_eq!(0.25f32, map(signal, -1.0, 1.0, -0.5, 1.0))
    }

    #[test]
    fn test_midi_to_frequency() {
        let midi = 12;
        assert_eq!(880f32, mtof(midi, 440f32))
    }
    
    #[test]
    /// frequencies are a bit skewed, towards equal temperment
    fn test_midi_to_frequency2() {
        let midi = 19;
        assert_eq!(1318.5103f32, mtof(midi, 440f32))
    }

    #[test]
    fn test_frequency_to_midi() {
        let freq = 880f32;
        assert_eq!(12, ftom(freq, 440f32))
    }
    
    #[test]
    /// frequencies are a bit skewed, towards equal temperment
    fn test_frequency_to_midi2() {
        let freq = 1318.5103f32;
        assert_eq!(19, ftom(freq, 440f32))
    }
}
