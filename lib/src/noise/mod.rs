pub mod white;
pub mod pink;
pub mod brown;
pub mod expensive;

pub const BIPOLAR: u32 = 0x40000000;
pub const UNIPOLAR: u32 = 0x3F800000;

pub struct Prng {
  s1: u32,
  s2: u32,
  s3: u32,
}

impl Prng {
  pub fn new(seed: u32) -> Self {
    let s1 = match 1234598713 ^ seed {
      x if x < 2 => 1243598713,
      x => x
    };
    let s2 = match 3093459404 ^ seed {
      x if x < 8 => 3093459404,
      x => x

    };
    let s3 = match 1821928721 ^ seed {
      x if x < 16 => 1821928721,
      x => x
    };

    Self {s1, s2, s3}
  }


  /// Generate a value between [-1.0..1.0)
  #[inline]
  pub fn frand_bipolar(&mut self) -> f32 {
    f32::from_bits(BIPOLAR | (self.trand() >> 9)) - 3.0
  }

  /// Generate a value between [0..1.0) 
  #[inline]
  pub fn frand_unipolar(&mut self) -> f32 {
    f32::from_bits(UNIPOLAR | (self.trand() >> 9)) - 1.0
  }

  /// Supercollider function [`trand`](
  /// ), producing a random u32
  #[inline]
  pub fn trand(&mut self) -> u32 {
    self.s1 = ((self.s1 & 0u32.wrapping_sub(2))  << 12) ^ (((self.s1 << 13) ^ self.s1) >> 19);
    self.s2 = ((self.s2 & 0u32.wrapping_sub(8))  << 4)  ^ (((self.s2 << 2)  ^ self.s2) >> 25);
    self.s3 = ((self.s3 & 0u32.wrapping_sub(16)) << 17) ^ (((self.s3 << 3)  ^ self.s3) >> 11);
    self.s1 ^ self.s2 ^ self.s3
  }
}

pub mod rng {
  /// Supercollider RNG - generates u32
  pub fn complex_u32(s1: &mut u32, s2: &mut u32, s3: &mut u32) -> u32 {
    *s1 = ((*s1 & 0u32.wrapping_sub(2))  << 12) ^ (((*s1 << 13) ^ *s1) >> 19);
    *s2 = ((*s2 & 0u32.wrapping_sub(8))  << 4)  ^ (((*s2 << 2)  ^ *s2) >> 25);
    *s3 = ((*s3 & 0u32.wrapping_sub(16)) << 17) ^ (((*s3 << 3)  ^ *s3) >> 11);
    *s1 ^ *s2 ^ *s3
  }

  pub fn simple_u32(state: &mut u32) -> u32 {
    *state = state.wrapping_mul(16807).wrapping_add(1);
    *state
  }
    
  #[inline(always)]
  pub fn moderate_u32(fpd: &mut u32) -> u32 {
    *fpd ^= *fpd << 13;
    *fpd ^= *fpd >> 17;
    *fpd ^= *fpd << 5;
    *fpd
  }

  pub fn utof_bipolar(int: &u32) -> f32 {
    f32::from_bits(*int >> 9 | super::BIPOLAR) - 3.0
  }

  pub fn utof_unipolar(int: &u32) -> f32 {
    f32::from_bits(*int >> 9 | super::UNIPOLAR) - 3.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn consistant_output() {
    const SEED: u32 = 1234;
    let mut rng = Prng::new(SEED);
    let mut a = vec![0; 2048];
    a.iter_mut().for_each(|x| *x = rng.trand());
    let mut rng = Prng::new(SEED);
    let mut b = vec![0; 2048];
    b.iter_mut().for_each(|x| *x = rng.trand());
    assert_eq!(a, b)
  }
}
