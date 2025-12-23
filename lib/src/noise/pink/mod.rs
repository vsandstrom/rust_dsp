
pub mod pk3 {
  use crate::noise::white;
  pub struct Noise {
    noise: white::Noise,
    coeffs: [(f32,f32); 6],
    filter: [f32; 7]
  }

  impl Default for Noise {
    fn default() -> Self {
      Self{
        noise: white::Noise::new(),
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
  }

  impl Noise {
    pub fn new() -> Self {
      Self::default()
    }

    pub fn process(&mut self,) -> f32 {
      let white = self.noise.process();
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
  use crate::noise::white;
  pub struct Noise {
    noise: white::Noise,
    coeffs: [(f32,f32); 3],
    filter: [f32; 3],
  }

  impl Default for Noise {
    fn default() -> Self {
      Self{
        noise: white::Noise::new(),
        coeffs: [
          (0.99765, 0.099046 ),
          (0.963  , 0.2965164),
          (0.57   , 1.0526913),
        ],
        filter: [0.0; 3],
      }
    }
  }

  impl Noise {
    pub fn new() -> Self { Self::default() }
    pub fn process(&mut self,) -> f32 {
      let white = self.noise.process();
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
mod voss_mccartney {
  use super::*;
  use rand::{SeedableRng, rngs::SmallRng, RngCore};
use rand::Rng;
  pub struct PinkNoise {
    total: u32,
    dice: [u32; 16],
    rng: SmallRng
  }

  impl Default for PinkNoise {
    fn default() -> Self {
      Self { 
        total: 0,
        dice: [0; 16],
        // Can panic on really rare occations
        rng: SmallRng::from_os_rng()
      }
    }
  }

  impl PinkNoise {
    pub fn new() -> Self {
      Self::default()
    }

    pub fn play(&mut self, out: &mut [f32]) {
      let mut counter = 0u32;
      out.iter_mut().for_each(|x| {
        let k = (counter.trailing_zeros() & 15) as usize;
        counter = counter.wrapping_add(1);
        let newrand = self.rng.next_u32() & 0x7FFF;
        let prevrand = self.dice[k];
        self.dice[k] = newrand;
        self.total = self.total.wrapping_add(newrand.wrapping_sub(prevrand));
        let bits = 0x3F80_0000 | (self.total & 0x007F_FFFF); 
        *x = f32::from_bits(bits) * 2.0 - 1.0;
      });
    }
  }
}


