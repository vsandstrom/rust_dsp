use super::rand::{thread_rng, Rng};
use super::TRand;
pub struct WhiteNoise {
  rng: TRand
}
impl WhiteNoise {
  pub fn new(seed: u32) -> Self { 
    Self{
      rng: TRand::new(seed) 
    } 
  }

  pub fn process(&mut self, out: &mut [f32]) { 
    out.iter_mut().for_each(|x| {
      let val = self.rng.next() | 0x4000_0000;
      *x = f32::from_bits(val) - 3.0;
    });
  }

}

