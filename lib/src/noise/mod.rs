pub mod white;
pub mod pink;
pub mod brown;
mod expensive;

use rand;

struct TRand {
  s1: u32,
  s2: u32,
  s3: u32,
}

impl TRand {
  fn new(seed: u32) -> Self {
    Self {    
      s1: seed.wrapping_mul(0x12345678).wrapping_add(1),
      s2: seed.wrapping_mul(0x87654321).wrapping_add(1),
      s3: seed ^ 0xdeadbeef
    }
  }

  /// SuperCollider RNG
  #[inline(always)]
  fn next(&mut self) -> u32 {
      // This matches SC's Tausworthe-based RNG structure
      self.s1 = ((self.s1 & 0xFFFF_FFFE) << 12) ^ (((self.s1 << 13) ^ self.s1) >> 19);
      self.s2 = ((self.s2 & 0xFFFF_FFF8) << 4)  ^ (((self.s2 << 2)  ^ self.s2) >> 25);
      self.s3 = ((self.s3 & 0xFFFF_FFF0) << 17) ^ (((self.s3 << 3)  ^ self.s3) >> 11);
      self.s1 ^ self.s2 ^ self.s3
  }
}
