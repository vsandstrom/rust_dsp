use super::rand::{SeedableRng, rngs::SmallRng, RngCore};

pub struct Noise {
  rng: SmallRng
}

impl Default for Noise {
  fn default() -> Self {
    Self{
      rng: SmallRng::from_os_rng()
    } 
  }
}

impl Noise {
  pub fn new() -> Self { 
    Self::default()
  }

  pub fn process_block(&mut self, out: &mut [f32]) { 
    out.iter_mut().for_each(|x| {
      let val = 0x3F80_0000 | self.rng.next_u32() & 0x007F_FFFF;
      *x = (f32::from_bits(val) - 1.5) * 2.0;
    });
  }

  pub fn process(&mut self) -> f32 {
    let val = 0x3F80_0000 | self.rng.next_u32() & 0x007F_FFFF;
    (f32::from_bits(val) - 1.5) * 2.0
  }
}

