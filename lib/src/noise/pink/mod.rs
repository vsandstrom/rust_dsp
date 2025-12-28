// default noise
pub use ridgerat::Noise;

pub mod pk3 {
  use crate::noise::Prng;
  /// 6.2ns per call - on Intel i7 3.1GHz Quad Core
  pub struct Noise {
    rng: Prng,
    coeffs: [(f32,f32); 6],
    filter: [f32; 7]
  }

  impl Noise {
    pub fn new(seed: u32) -> Self {
      Self {
        rng: Prng::new(seed),
        coeffs: [
          (0.99886, 0.0555179),
          (0.99332, 0.0750759),
          (0.96900, 0.153852 ),
          (0.8665 , 0.3104856),
          (0.55   , 0.5329522),
          (-0.7616, 0.016898 ), 
        ],
        filter: [0.0; 7]
      }
    }

    pub fn play(&mut self) -> f32 {
      let white = self.rng.frand_bipolar();
      self.filter[0] = self.coeffs[0].0 * self.filter[0] + white * self.coeffs[0].1;
      self.filter[1] = self.coeffs[1].0 * self.filter[1] + white * self.coeffs[1].1;
      self.filter[2] = self.coeffs[2].0 * self.filter[2] + white * self.coeffs[2].1;
      self.filter[3] = self.coeffs[3].0 * self.filter[3] + white * self.coeffs[3].1;
      self.filter[4] = self.coeffs[4].0 * self.filter[4] + white * self.coeffs[4].1;
      self.filter[5] = self.coeffs[5].0 * self.filter[5] - white * self.coeffs[5].1;
      let pink = self.filter[..6].iter().sum::<f32>() + white * 0.5362;
      self.filter[6] = white * 0.115926;
      pink
    }
  }
}

pub mod pke {
  /// 4.9ns per call - on Intel i7 3.1GHz Quad Core
  use crate::noise::{Prng};
  pub struct Noise {
    rng: Prng,
    coeffs: [(f32,f32); 3],
    filter: [f32; 3],
  }


  impl Noise {
    pub fn new(seed: u32) -> Self { 
      Self{
        rng: Prng::new(seed),
        coeffs: [
          (0.99765, 0.099046 ),
          (0.963  , 0.2965164),
          (0.57   , 1.0526913),
        ],
        filter: [0.0; 3],
      }
    }

    pub fn play(&mut self) -> f32 {
      let white = self.rng.frand_bipolar();
      self.coeffs.iter().zip(self.filter.iter_mut()).for_each(|((b, a), f)| {
        *f = *b * *f + white * *a;
      });
      self.filter.iter().sum::<f32>() + white * 0.1848
    }
    
    pub fn play_control(&mut self) -> f32 {
      let white = self.rng.frand_unipolar();
      self.coeffs.iter().zip(self.filter.iter_mut()).for_each(|((b, a), f)| {
        *f = *b * *f + white * *a;
      });
      self.filter.iter().sum::<f32>() + white * 0.1848
    }
  }
}

/// The Voss-McCartney Pink Noise generator sums a number of "rows" 
/// of independent white noise sources. Each row is updated at half
/// the rate of the row above. The top row is updated every sample.
/// ```ignore
/// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx - row 0
/// x x x x x x x x x x x x x x x x - row 1
///  x   x   x   x   x   x   x   x  - row 2
///    x       x       x       x    - row 3
///        x               x        - row 4
///                x                - row 5
/// ```
/// https://www.firstpr.com.au/dsp/pink-noise/#Filtering
pub mod voss_mccartney2 {
  /// 8.2ns per call - on Intel i7 3.1GHz Quad Core
  use rand::{SeedableRng, rngs::SmallRng, RngCore};
  pub struct Noise {
    noise: [u32; 16],
    rng: SmallRng,
    total: u32,
    counter: u16
  }

  impl Default for Noise {
    fn default() -> Self {
      Self { 
        noise: [0; 16],
        // Can panic on really rare occations
        rng: SmallRng::from_os_rng(),
        total: 0,
        counter: 1
      }
    }
  }

  impl Noise {
    const MASK: u16 = 0x1FFF;

    pub fn new() -> Self {
      Self::default()
    }
    pub fn play(&mut self) -> f32 {
      let i = self.counter.trailing_zeros() as usize;
      self.counter = (Self::MASK & self.counter) + 1;
      let next = self.rng.next_u32() >> 13;
      let prev = self.noise[i];
      self.noise[i] = next;
      self.total += next - prev;
      let val = f32::from_bits((self.total + (self.rng.next_u32() >> 13)) | crate::noise::BIPOLAR);
      val - 3.0
    }
  }
}


/// The Voss-McCartney Pink Noise generator sums a number of "rows" 
/// of independent white noise sources. Each row is updated at half
/// the rate of the row above. The top row is updated every sample.
/// ```ignore
/// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx - row -1
/// x x x x x x x x x x x x x x x x - row  0
///  x   x   x   x   x   x   x   x  - row  1
///    x       x       x       x    - row  2
///        x               x        - row  3
///                x                - row  4
/// ```
/// https://www.firstpr.com.au/dsp/pink-noise/#Filtering
pub mod voss_mccartney {
  /// 4.6ns per call - on Intel i7 3.1GHz Quad Core
  use crate::noise::Prng;
  pub struct Noise {
    noise: [u32; 16],
    rng: Prng,
    total: u32,
    counter: u16
  }

  impl Noise {
    const MASK: u16 = 0x1FFF;
    pub fn new(seed: u32) -> Self {
      Self { 
        noise: [0; 16],
        rng: Prng::new(seed),
        total: 0,
        counter: 1
      }
    }
    pub fn play(&mut self) -> f32 {
      let i = self.counter.trailing_zeros() as usize;
      self.counter = (Self::MASK & self.counter) + 1;
      let next = self.rng.trand() >> 13;
      let prev = self.noise[i];
      self.noise[i] = next;
      self.total += next - prev;
      let val = f32::from_bits((self.total + (self.rng.trand() >> 13)) | crate::noise::BIPOLAR);
      val - 3.0
    }
  }
}


/// Voss-McCartney design with extremely simple RNG suggested by `Matthijs` and `Skythedragon` on
/// the `Rust Audio discord`
pub mod discord {
  /// 7.2ns per call - on Intel i7 3.1GHz Quad Core
  pub struct Noise {
    noise: [u32; 16],
    total: u32,
    rng: u32,
    counter: u16,
  }

  impl Noise {
    const MASK: u16 = 0x1FFF;
    pub fn new(seed: u32) -> Self {
      Self { 
        noise: [0; 16],
        rng: seed,
        total: 0,
        counter: 1
      }
    }
    pub fn play(&mut self) -> f32 {
      let i = self.counter.trailing_zeros() as usize;
      self.counter = (Self::MASK & self.counter) + 1;
      self.rng = self.rng.wrapping_mul(16807).wrapping_add(1);
      let next = self.rng >> 13;
      let prev = self.noise[i];
      self.noise[i] = next;
      self.total += next - prev;
      self.rng = self.rng.wrapping_mul(16807).wrapping_add(1);
      let val = f32::from_bits((self.total + (self.rng >> 13)) | crate::noise::BIPOLAR);
      val - 3.0
    }
  }

}

/// Implementation of the Larry Tremell (RidgeRat) Pink noise design
/// borrowed from the FireWheel project by BillyDM
///
/// refs:
/// http://www.ridgerat-tech.us/pink/newpink.htm
/// https://github.com/BillyDM/Firewheel/blob/f72f532b9714eca27db3960ea6bc0ba91215b80b/crates/firewheel-nodes/src/noise_generator/pink.rs#L145
mod ridgerat {
  /// 7.2ns per call - on Intel i7 3.1GHz Quad Core
  pub struct Noise {
    seed: i32,
    contrib: [i32; 5],
    accum: i32,
  }

  impl Noise {
    const COEFF_A: [i32; 5] = [14055, 12759, 10733, 12273, 15716];
    const COEFF_SUM: [i16; 5] = [22347, 27917, 29523, 29942, 30007];
    const DIV: f32 = 1.0 / 2_147_483_648.0;
    pub fn new(seed: i32) -> Self { Self { seed, accum: 0, contrib: [0;5] } }
    #[inline(always)]
    pub fn play(&mut self) -> f32 {
      let randu: i16 = (Self::rng(&mut self.seed) & 0x7fff) as i16;
      let bytes = Self::rng(&mut self.seed).to_ne_bytes();
      let randv = i16::from_ne_bytes([bytes[0], bytes[1]]) as i32;

      if randu < Self::COEFF_SUM[0] {
          Self::update_contrib::<0>(&mut self.accum, &mut self.contrib, randv);
      } else if randu < Self::COEFF_SUM[1] {
          Self::update_contrib::<1>(&mut self.accum, &mut self.contrib, randv);
      } else if randu < Self::COEFF_SUM[2] {
          Self::update_contrib::<2>(&mut self.accum, &mut self.contrib, randv);
      } else if randu < Self::COEFF_SUM[3] {
          Self::update_contrib::<3>(&mut self.accum, &mut self.contrib, randv);
      } else if randu < Self::COEFF_SUM[4] {
          Self::update_contrib::<4>(&mut self.accum, &mut self.contrib, randv);
      }
      self.accum as f32 * Self::DIV 
    }

    #[inline(always)]
    fn rng(fpd: &mut i32) -> i32 {
      *fpd ^= *fpd << 13;
      *fpd ^= *fpd >> 17;
      *fpd ^= *fpd << 5;
      *fpd
    }

    #[inline(always)]
    fn update_contrib<const I: usize>(accum: &mut i32, contrib: &mut [i32; 5], randv: i32) {
      *accum = accum.wrapping_sub(contrib[I]);
      contrib[I] = randv * Self::COEFF_A[I];
      *accum = accum.wrapping_add(contrib[I]);
    }
  }
}


