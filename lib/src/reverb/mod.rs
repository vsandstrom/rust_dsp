pub mod schroeder;
pub mod chowning;
pub mod dattoro;
pub mod vikverb;

// use crate::delay::{Delay, DelayTrait, FixedDelay};
use crate::filter::{Filter, Comb};
use crate::interpolation::Interpolation;



// pub struct DattVerb {
//   predelay: Delay,
//   bw_delay: FixedDelay,
//   diffusion: [Comb; 4],
//   tank_head: [Comb; 2],
//   exursion: [Delay; 2],
//   // dc_block,
//   tank_tail: [Comb; 2]
// }
//
// impl DattVerb {
//   pub fn new(samplerate: f32) -> Self{
//     let sr = samplerate as usize;
//     Self { 
//       predelay: Delay::new(sr), 
//       bw_delay: FixedDelay::new(),
//       diffusion: [
//         Comb::new::<142>(samplerate, 0.7, 0.7), 
//         Comb::new::<107>(samplerate, 0.7, 0.7), 
//         Comb::new::<379>(samplerate, 0.7, 0.7), 
//         Comb::new::<277>(samplerate, 0.7, 0.7), 
//       ],
//       tank_head: [
//         Comb::new::<672>(samplerate, 0.7, 0.7), 
//         Comb::new::<908>(samplerate, 0.7, 0.7), 
//       ],
//       exursion: [
//         Delay::new(sr),
//         Delay::new(sr)
//       ],
//       tank_tail: [
//         Comb::new::<1800>(samplerate, 0.7, 0.7), 
//         Comb::new::<2656>(samplerate, 0.7, 0.7), 
//       ]
//     }
//   }
//
//   #[inline]
//   fn diffuse(&mut self, sample: &mut f32) -> f32 {
//     for diff in self.diffusion.iter_mut() {
//       *sample = diff.process(*sample);
//
//     }
//     *sample
//   }
//
//   #[inline]
//   fn tank(&mut self, sample: f32) -> f32 {
//     todo!()
//   }
//
//   pub fn process<DelayInterpolation: Interpolation>(&mut self, sample: f32, predelay: f32, bw: f32) -> f32 {
//     let pd = self.predelay.play::<DelayInterpolation>(sample, predelay, 0.4);
//     let mut sample = self.bw_delay.play(pd, bw);
//     sample = self.diffuse(&mut sample);
//     self.tank(sample);
//     0.0
//
//   } 
// }
//

pub trait Verb {
  fn new(samplerate: f32) -> Self;
  fn process<T: Interpolation>(&mut self, sample: f32) -> f32;
  fn set_damp(&mut self, damp: f32) {}
}



#[cfg(test)]
mod tests {
}
