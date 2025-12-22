use super::TRand;


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
pub mod voss_mccartney {
  use super::*;
use rand::{thread_rng, Rng};
  pub struct PinkNoise {
    total: u32,
    dice: [u32; 16],
    rng: TRand
  }

  impl PinkNoise {
    pub fn new(seed: u32) -> Self {
      Self { 
        total: 0,
        dice: [0; 16],
        rng: TRand::new(seed),
      }
    }

    pub fn play(&mut self, out: &mut [f32]) {
      out.iter_mut().for_each(|x| {
        let counter = self.rng.next(); // which row, closer to 0x0 / 0x1 is more common, 0x1000
                                         // is more rare
        let k = (counter.trailing_zeros() & 15) as usize;
        let newrand = counter >> 13;
        let prevrand = self.dice[k];
        self.total = self.total.wrapping_add(newrand.wrapping_sub(prevrand));
        let white = self.rng.next();
        let bits = self.total.wrapping_add(white) | 0x4000_0000;
        *x = f32::from_bits(bits) - 3.0;
      });
    }
  }
}


pub mod rbj {
  use super::*;
}
