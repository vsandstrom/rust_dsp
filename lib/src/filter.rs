use crate::dsp::signal::dcblock;
use crate::buffer::Buffer;
use crate::interpolation::interpolation::Interpolation;

pub struct Comb<const N: usize> {
  buffer: Buffer<N>,
  damp: f32,
  previous: f32,
  feedforward: f32,
  feedback: f32,
  position: usize,
  delay: usize,
  previous_in: f32,
  previous_out: f32,
}


pub trait Filter {
  fn set_damp(&mut self, damp: f32);
  fn process<T: Interpolation>(&mut self, sample: f32) -> f32;
}

impl<const N: usize> Comb<N> {
  pub fn new(samplerate: f32, feedforward: f32, feedback: f32) -> Self {
    let buffer = Buffer::<N>::new(samplerate);
    Comb{
      buffer,
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
}

impl<const N:usize> Filter for Comb<N> {
  /// Set optional LowPass damping, [0.0 - 1.0], 0.0 is off
  fn set_damp(&mut self, damp: f32) {
    self.damp = damp;
  }

  /// IIR: feedback > 0.0, feedforward == 0.0
  /// FIR: feedback == 0.0, feedforward > 0.0
  /// AP:  feedback == feedforward > 0.0
  fn process<T: Interpolation>(&mut self, sample: f32) -> f32 {
    let delayed = self.buffer.read::<T>(self.position as f32);
    let dc_blocked = sample - self.previous_in + 0.995 * self.previous_out;

    self.previous_in = sample;
    self.previous_out = dc_blocked;

    self.previous = delayed * (1.0 * self.damp) + self.previous * self.damp;
    let fb = dc_blocked - self.feedback * self.previous;
    self.buffer.write(fb, self.position);
    self.position = (self.position + 1) % self.delay;
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

#[allow(unused)]
pub struct VComb<const N:usize> {
  buffer: Buffer<N>,
  previous: f32,
  write_pos: usize,
  read_pos: f32,
  damp: f32
}

#[allow(unused)]
impl<const N:usize> VComb<N> {
  fn new(delay: usize, samplerate: f32, feedforward: f32, feedback: f32) -> Self {
    todo!();
  }
}


pub mod biquad {

  pub struct BiquadCoeffs {a1: f32, a2: f32, b0: f32, b1: f32, b2: f32}
  
  pub trait BiquadTrait {
    fn process(&mut self, sample: f32) -> f32;
    fn set_coeffs(&mut self, coeffs: BiquadCoeffs);
  }
  
  trait FilterBankTrait<const N: usize> {
    fn process(&self, sample: f32) -> f32;
    fn set_coeffs(&mut self, coeffs: [BiquadCoeffs; N]);
  }

  #[derive(Clone, Copy)]
  pub struct Biquad {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    prev_in: f32,
    prev_out: f32
  }
  
  #[derive(Clone, Copy)]
  pub struct Biquad4 {
    x1_1: f32,
    x1_2: f32,
    y1_1: f32,
    y1_2: f32,
    x2_1: f32,
    x2_2: f32,
    y2_1: f32,
    y2_2: f32,
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    prev_in: f32,
    prev_out: f32
  }
  
  #[derive(Clone, Copy)]
  pub struct Biquad8 {
    x1_1: f32,
    x1_2: f32,
    y1_1: f32,
    y1_2: f32,
    x2_1: f32,
    x2_2: f32,
    y2_1: f32,
    y2_2: f32,
    x3_1: f32,
    x3_2: f32,
    y3_1: f32,
    y3_2: f32,
    x4_1: f32,
    x4_2: f32,
    y4_1: f32,
    y4_2: f32,
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    prev_in: f32,
    prev_out: f32
  }
  
  impl Biquad {
    pub fn new() -> Self {
      Self {
        x1: 0.0,
        x2: 0.0,
        y1: 0.0,
        y2: 0.0,
        a1: 0.0,
        a2: 0.0,
        b0: 0.0,
        b1: 0.0,
        b2: 0.0,
        prev_in: 0.0,
        prev_out: 0.0
      }
    }
  }
  
  impl BiquadTrait for Biquad {
    // Direct form I
    fn process(&mut self, sample: f32) -> f32 {
      let output = 
        self.b0 * sample 
        + self.b1 * self.x1 
        + self.b2 * self.x2
        - self.a1 * self.y1
        - self.a2 * self.y2;

      self.x2 = self.x1;
      self.x1 = sample;
      self.y2 = self.y1;
      self.y1 = output;
      output
    }

    fn set_coeffs(&mut self, coeffs: BiquadCoeffs) {
      self.a1 = coeffs.a1;
      self.a2 = coeffs.a2;
      self.b0 = coeffs.b0;
      self.b1 = coeffs.b1;
      self.b2 = coeffs.b2;
    }
  }

  impl BiquadTrait for Biquad4 {
    fn process(&mut self, sample: f32) -> f32 {
      let mut output = 
        self.b0 * sample 
        + self.b1 * self.x1_1 
        + self.b2 * self.x1_2
        - self.a1 * self.y1_1
        - self.a2 * self.y1_2;

      self.x1_2 = self.x1_1;
      self.x1_1 = sample;
      self.y1_2 = self.y1_1;
      self.y1_1 = output;
      
      output = self.b0 * output 
        + self.b1 * self.x2_1 
        + self.b2 * self.x2_2
        - self.a1 * self.y2_1
        - self.a2 * self.y2_2;

      self.x2_2 = self.x2_1;
      self.x2_1 = self.y1_1;
      self.y2_2 = self.y2_1;
      self.y2_1 = output;
      output
        
    }

    fn set_coeffs(&mut self, coeffs: BiquadCoeffs) {
      self.a1 = coeffs.a1;
      self.a2 = coeffs.a2;
      self.b0 = coeffs.b0;
      self.b1 = coeffs.b1;
      self.b2 = coeffs.b2;
    }
  }

  impl BiquadTrait for Biquad8 {
    fn process(&mut self, sample: f32) -> f32 {
      let mut output = 
        self.b0 * sample 
        + self.b1 * self.x1_1 
        + self.b2 * self.x1_2
        - self.a1 * self.y1_1
        - self.a2 * self.y1_2;

      self.x1_2 = self.x1_1;
      self.x1_1 = sample;
      self.y1_2 = self.y1_1;
      self.y1_1 = output;
      
      output = self.b0 * output 
        + self.b1 * self.x2_1 
        + self.b2 * self.x2_2
        - self.a1 * self.y2_1
        - self.a2 * self.y2_2;

      self.x2_2 = self.x2_1;
      self.x2_1 = self.y1_1;
      self.y2_2 = self.y2_1;
      self.y2_1 = output;
      output
        
    }

    fn set_coeffs(&mut self, coeffs: BiquadCoeffs) {
      self.a1 = coeffs.a1;
      self.a2 = coeffs.a2;
      self.b0 = coeffs.b0;
      self.b1 = coeffs.b1;
      self.b2 = coeffs.b2;
    }
  }
  
  #[derive(Clone, Copy)]
  pub struct BiquadN<const N: usize> {
    bq: [Biquad; N],
    a1: f32,
    a2: f32,
    b0: f32,
    b1: f32,
    b2: f32,
    prev_in: f32,
    prev_out: f32
  }
  
  impl<const N: usize> BiquadN<N> {
    pub fn new() -> Self {
      Self {
        bq: [Biquad::new(); N],
        a1: 0.0,
        a2: 0.0,
        b0: 0.0,
        b1: 0.0,
        b2: 0.0,
        prev_in: 0.0,
        prev_out: 0.0
      }
    }
  }

  impl<const N:usize> BiquadTrait for BiquadN<N> {
    fn process(&mut self, sample: f32) -> f32 {
      let mut input = sample;
      for mut b in self.bq {
        input = b.process(input);
      }
      input 
    }
    
    fn set_coeffs(&mut self, coeffs: BiquadCoeffs) {
      self.a1 = coeffs.a1;
      self.a2 = coeffs.a2;
      self.b0 = coeffs.b0;
      self.b1 = coeffs.b1;
      self.b2 = coeffs.b2;
    }
  }


  #[derive(Clone, Copy)]
  pub struct FilterBank<const N: usize> {
    bank: [Biquad; N],
  }

  pub struct FilterBank4<const N:usize> {
    bank: [BiquadN<2>; N]
  }
  
  pub struct FilterBank8<const N:usize> {
    bank: [BiquadN<4>; N]
  }

  impl<const N:usize> FilterBank<N> {
    pub fn new() -> Self {
      Self {
        bank: [Biquad::new(); N]
      }
    }
    pub fn size(&self) -> usize {
      N
    }
  }

  impl<const N: usize> FilterBankTrait<N> for FilterBank4<N> {
    fn process(&self, sample: f32) -> f32 {
      let mut out = 0.0;
      for mut b in self.bank {
        out += b.process(sample);
      }
      out
    }

    fn set_coeffs(&mut self, coeffs: [BiquadCoeffs; N]) {
      for i in 0..N {
        self.bank[i].a1 = coeffs[i].a1;
        self.bank[i].a2 = coeffs[i].a2;
        self.bank[i].b0 = coeffs[i].b0;
        self.bank[i].b1 = coeffs[i].b1;
        self.bank[i].b2 = coeffs[i].b2;
      }
    }
  }
  
  impl<const N: usize> FilterBankTrait<N> for FilterBank8<N> {
    fn process(&self, sample: f32) -> f32 {
      let mut out = 0.0;
      for mut b in self.bank {
        out += b.process(sample);
      }
      out
    }

    fn set_coeffs(&mut self, coeffs: [BiquadCoeffs; N]) {
      for i in 0..N {
        self.bank[i].a1 = coeffs[i].a1;
        self.bank[i].a2 = coeffs[i].a2;
        self.bank[i].b0 = coeffs[i].b0;
        self.bank[i].b1 = coeffs[i].b1;
        self.bank[i].b2 = coeffs[i].b2;
      }
    }
  }
  
  impl<const N: usize> FilterBankTrait<N> for FilterBank<N> {
    fn process(&self, sample: f32) -> f32 {
      let mut out = 0.0;
      for mut b in self.bank {
        out += b.process(sample);
      }
      out
    }

    fn set_coeffs(&mut self, coeffs: [BiquadCoeffs; N]) {
      for i in 0..N {
        self.bank[i].a1 = coeffs[i].a1;
        self.bank[i].a2 = coeffs[i].a2;
        self.bank[i].b0 = coeffs[i].b0;
        self.bank[i].b1 = coeffs[i].b1;
        self.bank[i].b2 = coeffs[i].b2;
      }
    }
  }

  #[inline]
  pub fn calc_lpf(w: f32, q: f32) -> BiquadCoeffs {
      let alpha = w.sin() / (2.0 * q);
      let a0 = 1.0 + alpha;
      let a1 = (-1.0 * w.cos()) / a0 ;
      let a2 = (1.0 - alpha) / a0;

      let b1 = (1.0 - w.cos()) / a0;
      let b0 = b1 / 2.0 / a0;
      let b2 = b0;
      BiquadCoeffs{a1, a2, b0, b1, b2}
  }
    
  #[inline]
  pub fn calc_bpf(w: f32, q: f32) -> BiquadCoeffs {
    let alpha = w.sin() / (2.0 * q);
    
    let a0 = 1.0 + alpha;
    let a1 = (-1.0 * w.cos()) / a0;
    let a2 = (1.0 - alpha) / a0;

    let b0 = alpha / a0;
    let b1 = 0.0;
    let b2 = -alpha / a0;
    BiquadCoeffs{a1, a2, b0, b1, b2}
  }

  #[inline]
  pub fn calc_hpf(w: f32, q: f32) -> BiquadCoeffs {
    let alpha = w.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    let a1 = -1.0 * w.cos() / a0;
    let a2 = 1.0 - alpha / a0;

    let b0 = (1.0 + w.cos()) / 2.0 / a0;
    let b1 = -(b0 * 2.0);
    let b2 = b0;
    BiquadCoeffs{a1, a2, b0, b1, b2}
  }

  #[inline]
  pub fn calc_notch(w: f32, q: f32) -> BiquadCoeffs {
    let alpha = w.sin() / (2.0 * q);
    let a0 = 1.0 + alpha;
    let a1 = -2.0 * w.cos() / a0;
    let a2 = (1.0 - alpha) / a0;

    let b0 = 1.0 / a0;
    let b1 = a1;
    let b2 = b0;
    BiquadCoeffs{a1, a2, b0, b1, b2}
  }

  #[inline]
  pub fn calc_peq(w: f32, q: f32) -> BiquadCoeffs {
    todo!()
  }

}


#[cfg(test)]
mod tests {

}
