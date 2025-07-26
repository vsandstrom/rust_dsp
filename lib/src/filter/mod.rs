pub mod biquad;
pub mod onepole;
pub mod comb;
pub mod svf;

use alloc::{vec, vec::Vec};
use crate::interpolation::Interpolation;

pub struct Lpf;
pub struct Hpf;
pub struct Bpf;
pub struct Notch;
pub struct HighShelf;
pub struct LowShelf;
pub struct Peq;

pub trait Filter {
  fn process(&mut self, sample: f32) -> f32;
}

pub trait FilterKind {
  type Settings;
  type Coefficients;
  fn calc(settings: &Self::Settings) -> Self::Coefficients;
}

pub trait InterpolatingFilter {
  fn process<I: Interpolation>(&mut self, sample: f32, position: f32) -> f32;
}

pub trait SVFTrait {
  fn process(&mut self, sample: f32) -> f32;
  fn calc_lpf(&mut self, w: f32, q: f32);
  fn calc_hpf(&mut self, w: f32, q: f32);
  fn calc_bpf(&mut self, w: f32, q: f32);
  fn calc_peq(&mut self, w: f32, q: f32, gain: f32);
  fn calc_notch(&mut self, w: f32, q: f32);
  fn calc_high_shelf(&mut self, w: f32, q: f32, gain: f32) {}
  fn calc_low_shelf(&mut self, w: f32, q: f32, gain: f32) {}
}

#[macro_export]
macro_rules! impl_filter_kind {
  (
    trait $trait_name:ident,
    settings = $settings_ty:ty,
    output = $output_ty:ty,
    mappings = {
      $(
        $type:ty => $method:ident $( [ $($arg:ident),* ] )?
      ),* $(,)?
  }
  ) => {
    $(
      /// Generates implementations for different filter types, 
      /// assigns `Lpf` to use the `lpf`-method for a given trait. 
      /// ```
      /// impl $trait_name for $type {
      ///   type Settings = $settings_ty;
      ///   #[inline]
      ///   fn calc(settings: &Self::Settings) -> $output_ty {
      ///     < $output_ty >::$method(
      ///       settings.omega,
      ///       settings.q,
      ///       $( $(settings.$arg),*)?
      ///     )
      ///   }
      /// }
      /// ```
      ///
      impl $trait_name for $type {
        type Settings = $settings_ty;
        #[inline]
        fn calc(settings: &Self::Settings) -> $output_ty {
          < $output_ty >::$method(
            settings.omega,
            settings.q,
            $( $(settings.$arg),*)?
          )
        }
      }
    )*
  };
}

// pub struct Comb {
//   buffer: Vec<f32>,
//   damp: f32,
//   previous: f32,
//   feedforward: f32,
//   feedback: f32,
//   position: usize,
//   delay: usize,
//   previous_in: f32,
//   previous_out: f32,
// }
//
// impl Comb {
//   pub fn new<const N: usize>(feedforward: f32, feedback: f32) -> Self {
//     Self {
//       buffer: vec![0.0;N],
//       previous: 0.0,
//       damp: 0.0,
//       position: 0,
//       feedforward,
//       feedback,
//       delay: N,
//       previous_in: 0.0,
//       previous_out: 0.0
//     }
//   }
// }
//
// impl Filter for Comb {
//   /// Set optional LowPass damping, [0.0 - 1.0], 0.0 is off
//   fn set_damp(&mut self, damp: f32) {
//     self.damp = damp;
//   }
//
//   /// IIR: feedback > 0.0, feedforward == 0.0
//   /// FIR: feedback == 0.0, feedforward > 0.0
//   /// AllPass:  feedback == feedforward > 0.0
//   fn process(&mut self, sample: f32) -> f32 {
//     let delayed = self.buffer[self.position];
//     let dc_blocked = sample - self.previous_in + 0.995 * self.previous_out;
//
//     self.previous_in = sample;
//     self.previous_out = dc_blocked;
//
//     self.previous = delayed * (1.0 * self.damp) + self.previous * self.damp;
//     let fb = dc_blocked - self.feedback * self.previous;
//     self.buffer[self.position] = fb;
//     self.position = (self.position + 1) % self.delay;
//     self.feedforward * fb + delayed
//   }
// }
  
  //
  //         feedforward comb filter
  //
  //        ╓──> ( * b0 )───────╖
  //        ║   ╓─────────╖     V
  //  x(n) ─╨─> ║  z(-M)  ║─> ( + )──> y(n)
  //            ╙─────────╜    
  //
  
  //
  //          feedback comb filter
  //
  //               ╓─────────────────> y(n)
  //               ║   ╓─────────╖ 
  //  x(n) ─>( + )─╨─> ║  z(-M)  ║──╖
  //           Λ       ╙─────────╜  ║ 
  //           ╙────────( * aM ) <──╜
  //
  
  //
  //             allpass filter
  //
  //                ╓───> ( * b0 )─────────╖
  //                ║   ╓─────────╖        V
  //  x(n) ─> ( + )─╨─> ║  z(-M)  ║──╥─> ( + )──> y(n)
  //            Λ       ╙─────────╜  ║ 
  //            ╙────────( * -aM ) <─╜
  //
  //       where: b0 == aM

// #[derive(Default)]
// pub struct Onepole {
//   prev: f32,
//   damp: f32
// }
//
// impl Onepole {
//   pub fn new() -> Self {
//     Self {
//       prev: 0.0,
//       damp: 0.0
//     }
//   }
// }
//
// impl Filter for Onepole {
//   fn process(&mut self, sample: f32) -> f32 {
//     self.prev = (self.damp * sample) + ((1.0 - self.damp) * self.prev);
//     self.prev
//   }
//
//   fn set_damp(&mut self, damp: f32) {
//     self.damp = damp;
//   }
// }
//
//
// #[cfg(test)]
// mod tests {
//
// }
