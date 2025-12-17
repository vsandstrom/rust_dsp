use super::{Filter, InterpolatingFilter};

pub struct Comb {
  buffer: Vec<f32>,
  feedforward: f32,
  feedback: f32,
  position: usize,
  delay: usize,
}

impl Comb {
  // IIR: feedback > 0.0, feedforward == 0.0
  // FIR: feedback == 0.0, feedforward > 0.0
  // AllPass:  feedback == feedforward > 0.0
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
  pub fn new<const N: usize>(feedforward: f32, feedback: f32) -> Self {
    Self {
      buffer: vec![0.0;N],
      position: 0,
      feedforward,
      feedback,
      delay: N,
    }
  }
}

impl Filter for Comb {
  /// IIR: feedback > 0.0, feedforward == 0.0
  /// FIR: feedback == 0.0, feedforward > 0.0
  /// AllPass:  feedback == feedforward > 0.0
  fn process(&mut self, sample: f32) -> f32 {
    let buf = self.buffer[self.position];

    let fb = sample - self.feedback * buf;
    self.buffer[self.position] = fb;

    let out = self.feedforward * fb + buf;
    self.position += 1;
    if self.position >= self.delay {
      self.position -= self.delay;
    }
    out
  }
}

impl InterpolatingFilter for Comb {
  /// IIR: feedback > 0.0, feedforward == 0.0
  /// FIR: feedback == 0.0, feedforward > 0.0
  /// AllPass:  feedback == feedforward > 0.0
  /// 
  /// Offset is amount of samples from the end of the delay line 
  /// that it can subtract from the read position.
  fn process<I: crate::interpolation::Interpolation>(&mut self, sample: f32, offset: f32) -> f32 {
    let offset = offset.clamp(0.0, (self.delay-1) as f32);
    let buf = I::interpolate(self.position as f32 - offset, &self.buffer, self.delay);
    let fb = sample - self.feedback * buf;
    self.buffer[self.position] = fb;

    let out = self.feedforward * fb + buf;
    self.position += 1;
    if self.position >= self.delay {
      self.position -= self.delay;
    }
    out
  }
}

pub trait InverseComb {
  fn process_inverse(&mut self, sample: f32) -> f32;
}

impl InverseComb for Comb {
  fn process_inverse(&mut self, sample: f32) -> f32 {
    let buf = self.buffer[self.position];
    let fb = sample + self.feedback * buf;
    self.buffer[self.position] = fb;

    let out = self.feedforward * fb - buf;
    self.position += 1;
    if self.position >= self.delay {
      self.position -= self.delay;
    }
    out
  }
}

pub struct LPComb {
  buffer: Vec<f32>,
  damp: f32,
  previous: f32,
  feedforward: f32,
  feedback: f32,
  position: usize,
  delay: usize,
  previous_in: f32,
  previous_out: f32,
}

impl LPComb {
  pub fn new<const N: usize>(feedforward: f32, feedback: f32) -> Self {
    Self {
      buffer: vec![0.0;N],
      previous: 0.0,
      damp: 0.0,
      position: 0,
      feedforward,
      feedback,
      delay: N,
      previous_in: 0.0,
      previous_out: 0.0
    }
  }
  
  /// Set optional LowPass damping, [0.0 - 1.0], 0.0 is off
  pub fn set_damp(&mut self, damp: f32) {
    self.damp = damp;
  }
}

impl Filter for LPComb {

  // IIR: feedback > 0.0, feedforward == 0.0
  // FIR: feedback == 0.0, feedforward > 0.0
  // AllPass:  feedback == feedforward > 0.0
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
  fn process(&mut self, sample: f32) -> f32 {
    let delayed = self.buffer[self.position];
    let dc_blocked = sample - self.previous_in + 0.995 * self.previous_out;

    self.previous_in = sample;
    self.previous_out = dc_blocked;

    self.previous = delayed + self.damp * ( self.previous - delayed );

    let fb = dc_blocked - self.feedback * self.previous;
    self.buffer[self.position] = fb;
    self.position = (self.position + 1) % self.delay;
    self.feedforward * fb + delayed
  }
}

impl InterpolatingFilter for LPComb {
  fn process<I: crate::interpolation::Interpolation>(&mut self, sample: f32, offset: f32) -> f32 {
    let offset = offset.clamp(0.0, (self.delay-1) as f32);
    let delayed = I::interpolate(self.position as f32 - offset, &self.buffer, self.delay);
    let dc_blocked = sample - self.previous_in + 0.995 * self.previous_out;

    self.previous_in = sample;
    self.previous_out = dc_blocked;

    self.previous = delayed + self.damp * ( self.previous - delayed );

    let fb = dc_blocked - self.feedback * self.previous;
    self.buffer[self.position] = fb;
    self.position = (self.position + 1) % self.delay;
    self.feedforward * fb + delayed
  }
}

