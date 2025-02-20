use super::*;
use super::BiquadCoeffs;
use super::BiquadTrait;

pub trait FilterBankTrait<const N: usize> {
  fn process(&self, sample: f32) -> f32;
  fn set_coeffs(&mut self, coeffs: [BiquadCoeffs; N]);
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

impl<const N: usize> FilterBankTrait<N> for FilterBank<N> {
  fn process(&self, sample: f32) -> f32 {
    let mut out = 0.0;
    for mut b in self.bank {
      out += b.process(sample);
    }
    out
  }

  fn set_coeffs(&mut self, coeffs: [BiquadCoeffs; N]) {
    for (i, c) in coeffs.iter().enumerate().take(N) {
      self.bank[i].a1 = c.a1;
      self.bank[i].a2 = c.a2;
      self.bank[i].b0 = c.b0;
      self.bank[i].b1 = c.b1;
      self.bank[i].b2 = c.b2;
    }
  }
}


impl<const N:usize> Default for FilterBank<N> {
  fn default() -> Self {
      Self::new()
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
    for (bank, c) in self.bank.iter_mut().zip(coeffs.iter()) {
      bank.a1 = c.a1;
      bank.a2 = c.a2;
      bank.b0 = c.b0;
      bank.b1 = c.b1;
      bank.b2 = c.b2;
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
    for (bank, c) in self.bank.iter_mut().zip(coeffs.iter()) {
      bank.a1 = c.a1;
      bank.a2 = c.a2;
      bank.b0 = c.b0;
      bank.b1 = c.b1;
      bank.b2 = c.b2;
    }
  }
}

