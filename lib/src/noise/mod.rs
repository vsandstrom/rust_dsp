pub mod white;
pub mod pink;
pub mod brown;
mod expensive;

use rand;

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
    self.s1 = ((self.s1 & 0u32.wrapping_sub(2)) << 12) ^ (((self.s1 << 13) ^ self.s1) >> 19);
    self.s2 = ((self.s2 & 0u32.wrapping_sub(8)) << 4) ^ (((self.s2 << 2) ^ self.s2) >> 25);
    self.s3 = ((self.s3 & 0u32.wrapping_sub(16)) << 17) ^ (((self.s3 << 3) ^ self.s3) >> 11);
    self.s1 ^ self.s2 ^ self.s3
  }
}
