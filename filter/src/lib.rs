extern crate buffer;
extern crate interpolation;
use std::marker::PhantomData;

use buffer::Buffer;
use interpolation::interpolation::Interpolation;
pub struct Comb<T> {
  buffer: Buffer<T>,
  previous: f32,
  damp: f32,
  feedforward: f32,
  feedback: f32,
  position: usize,
  delay: usize,
  interpolation: PhantomData<T>
}


impl<T: Interpolation> Comb<T> {
  pub fn new(delay: usize, samplerate: f32, feedforward: f32, feedback: f32) -> Self {
    let buffer = Buffer::<T>::new(delay, samplerate);
    Comb{
      buffer,
      previous: 0.0,
      damp: 0.0,
      position: 0,
      feedforward,
      feedback,
      delay,
      interpolation: PhantomData
    }
  }

  /// Set optional LowPass damping, [0.0 - 1.0], 0.0 is off
  pub fn set_damp(&mut self, damp: f32) {
    self.damp = damp;
  }

  /// IIR: feedback > 0.0, feedforward == 0.0
  /// FIR: feedback == 0.0, feedforward > 0.0
  /// AP:  feedback == feedforward > 0.0
  pub fn process(&mut self, sample: f32) -> f32 {
    let delayed = self.buffer.read(self.position as f32);
    self.previous = delayed * (1.0 * self.damp) + self.previous * self.damp;
    let fb = sample - self.feedback * self.previous;
    self.buffer.write(fb, self.position);
    self.position += 1 % self.delay;
    self.feedforward * fb + delayed
  }
}
  
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

#[cfg(test)]
mod tests {
    use super::*;

}
