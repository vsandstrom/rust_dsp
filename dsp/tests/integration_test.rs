#[cfg(test)]
mod tests {
    use dsp::buffer::{normalize, sum};
    use dsp::signal::{ 
      unipolar, map, clamp,
      traits::SignalFloat 
    };
    use dsp::buffer::{
      range, scale,
      traits::SignalVector
    };
    use dsp::math::mtof;
    use dsp::math::ftom;
    use dsp::math::{is_pow2, next_pow2};

    #[test]
    fn clamp_test() {
        let sample:f32 = -1.0;
        assert_eq!(0.0f32, clamp(sample, 0.0, 1.0));
    }
    
    #[test]
    fn clamp_test2() {
        let sample:f32 = 2.0;
        assert_eq!(1.0f32, clamp(sample, 0.0, 1.0));
    }

    #[test]
    fn clamp_trait_test() {
        let sample:f32 = -1.0;
        assert_eq!(0.0f32, sample.clamp(0.0, 1.0));
    }

    #[test]
    fn clamp_trait_test2() {
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
    fn scale_test2() {
      let mut vec = vec![0.0, 3.0, 18.0];
      scale(&mut vec, 0.5, 1.0);
      assert_eq!(0.5, vec[0])
    }
    
    #[test]
    fn scale_trait_test() {
      let vec = vec![0.0, 3.0, 18.0].normalize().scale(0.0, 1.0);
      assert_eq!(1.0, vec[2])
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

    #[test]
    fn is_pow2_test() {
      let n = 16;
      assert_eq!(is_pow2(n), true);
    }

    #[test]
    fn is_pow2_test1() {
      let n = 17;
      assert_ne!(is_pow2(n), true);
    }
    
    #[test]
    fn is_pow2_test2() {
      let n = 0;
      assert_ne!(is_pow2(n), true);
    }

    #[test]
    fn pow2_test() {
      let n = 111;
      assert_eq!(is_pow2(next_pow2(n)), true);

    }
}
