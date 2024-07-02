use std::f32::consts::{PI, TAU};
use dsp::buffer::scale;

/// Create a complex waveform from amplitudes and phases of sine partials
/// (tip: normalize amplitudes to get waveform within -1.0 - 1.0)
pub fn complex_sine<'a, const N:usize>(table: &'a mut [f32], amps: &'a [f32; N], phases: &'a [f32; N]) -> &'a [f32] {
  let len = table.len();
  let mut n: f32 = 1.0;
  if amps.len() == phases.len() {
    for i in 0..amps.len() {
      let inc = TAU * n / len as f32;
      let mut angle = inc * len as f32 * phases[i];
      for j in 0..len {
        table[j] += angle.sin() * amps[i];
        angle += inc;
      }
      n += 1.0;
    }
    scale(table, -1.0f32, 1.0f32);
  }
  table
}
/// Sine: sin(2pi / table.len() * n)
pub fn sine(table: &mut [f32]) -> &[f32] {
  let mut angle: f32 = 0.0;
  let inc: f32 = TAU / table.len() as f32;
  for i in 0..table.len() {
    table[i] = angle.sin();
    angle += inc;
  }
  table
}

/// Squared sinewave, positive bellcurve. Useful as envelope
pub fn hanning(table: &mut [f32]) -> &[f32] {
  let mut angle: f32 = 0.0;
  let inc: f32 = PI / (table.len() as f32);
  for i in 0..table.len() {
    table[i] = angle.sin().powf(2.0);
    angle += inc;
  }
  table
}

/// Square
pub fn square(table: &mut [f32]) -> &[f32] {
  let mut val = -1.0;
  for i in 0..table.len() {
    table[i] = val;
    if i == table.len()/2-1 { val = 1.0; } 
  }
  table
}

/// Triangle 
pub fn triangle(table: &mut [f32]) -> &[f32] {
  let mut angle = 0.0;
  let mut inc: f32 = 2.0 / (table.len() as f32 / 2.0);
  for i in 0..table.len() {
    if angle >= 1.0 || angle <= -1.0 { inc = inc * -1.0; }
    table[i] = angle;
    angle += inc;
  }
  table
}

/// Sawtooth: -1.0 -> 1.0
pub fn sawtooth(table: &mut [f32]) -> &[f32] {
  let mut angle: f32 = 0.0;
  let inc: f32 = 2.0 / (table.len() as f32 - 1.0);
  for i in 0..table.len() {
    table[i] = angle - 1.0;
    angle += inc;
  }
  table
}

/// Reverse sawtooth: 1.0 -> -1.0
pub fn reverse_sawtooth(table: &mut [f32]) -> &[f32] {
  let mut angle: f32 = 0.0;
  let inc: f32 = 2.0 / (table.len() as f32 - 1.0);
  for i in 0..table.len() {
    table[i] = angle + 1.0;
    angle -= inc;
  }
  table
}

pub mod traits {
  use super::*;
  pub trait Waveshape<const N: usize> {
    type Output;

    fn sine(&mut self) -> &mut Self::Output;
    fn hanning(&mut self) -> &mut Self::Output;
    fn triangle(&mut self) -> &mut Self::Output;
    fn square(&mut self) -> &mut Self::Output;
    fn sawtooth(&mut self) -> &mut Self::Output;
    fn phasor(&mut self) -> &mut Self::Output;
    fn reverse_sawtooth(&mut self) -> &mut Self::Output;
    fn complex_sine<const M:usize>( 
      &mut self, amps: [f32; M], phases: [f32; M]
    ) -> &mut Self::Output;
  }

  impl<const N: usize> Waveshape<N> for [f32; N] {
    type Output = [f32; N];

    fn hanning(&mut self) -> &mut Self::Output{
      let mut angle: f32 = 0.0;
      let inc: f32 = PI / (self.len() as f32);
      for i in 0..N {
        self[i] = angle.sin().powf(2.0);
        angle += inc;
      }
      self
    }

    /// Sine: sin(2pi / table.len() * n)
    fn sine(&mut self) -> &mut Self::Output{
      let mut angle: f32 = 0.0;
      let inc: f32 = TAU / self.len() as f32;
      for i in 0..self.len() {
        self[i] = angle.sin();
        angle += inc;
      }
      self
    }

    ///Square
    fn square(&mut self) -> &mut Self::Output{
      let mut val = -1.0;
      for i in 0..self.len() {
        self[i] = val;
        if self[i] == self.len() as f32/2.0-1.0 { val = 1.0; } 
      }
      self
    }

    /// Triangle
    fn triangle(&mut self) -> &mut Self::Output {
      let mut angle = 0.0;
      let mut inc: f32 = 2.0 / (self.len() as f32 / 2.0);
      for i in 0..self.len() {
        if angle >= 1.0 || angle <= -1.0 { inc = inc * -1.0; }
        self[i] = angle;
        angle += inc;
      }
      self
    }

    /// Sawtooth: -1.0 -> 1.0
    fn sawtooth(&mut self) -> &mut Self::Output {
      let mut angle: f32 = 0.0;
      let inc: f32 = 2.0 / (self.len() as f32 - 1.0);
      for i in 0..self.len() {
        self[i] = angle - 1.0;
        angle += inc;
      }
      self
    }
    
    /// Sawtooth: -1.0 -> 1.0
    fn phasor(&mut self) -> &mut Self::Output {
      let mut angle: f32 = 0.0;
      let inc: f32 = 1.0 / (self.len() as f32 - 1.0);
      for i in 0..self.len() {
        self[i] = angle;
        angle += inc;
      }
      self
    }

    /// Reverse sawtooth: 1.0 -> -1.0
    fn reverse_sawtooth(&mut self) -> &mut Self::Output {
      let mut angle: f32 = 0.0;
      let inc: f32 = 2.0 / (self.len() as f32 - 1.0);
      for i in 0..self.len() {
        self[i] = angle + 1.0;
        angle -= inc;
      }
      self
    }

    /// Create a complex waveform from amplitudes and phases of sine partials
    fn complex_sine<const M:usize>(&mut self, amps: [f32; M], phases: [f32; M]) -> &mut Self::Output {
      let mut n: f32 = 1.0;
      if amps.len() == phases.len() {
        for i in 0..amps.len() {
          let inc = TAU * n / self.len() as f32;
          let mut angle = inc * self.len() as f32 * phases[i];
          for j in 0..self.len() {
            self[j] += angle.sin() * amps[i];
            angle += inc;
          }
          n += 1.0;
        }
        scale(self, -1.0f32, 1.0f32);
      }
      self
    }
  }


  impl<const N:usize> Waveshape<N> for Vec<f32>  {
    type Output = Vec<f32>;
    /// Squared sinewave, positive bellcurve. Useful as envelope
    fn hanning(&mut self) -> &mut Self::Output{
      let mut angle: f32 = 0.0;
      let inc: f32 = PI / (self.len() as f32);
      for i in 0..self.len() {
        self[i] = angle.sin().powf(2.0);
        angle += inc;
      }
      self
    }
    
    /// Phasor: 0.0 -> 1.0
    /// Useful for looping through buffers
    fn phasor(&mut self) -> &mut Self::Output {
      let mut angle: f32 = 0.0;
      let inc: f32 = 1.0 / (self.len() as f32 - 1.0);
      for i in 0..self.len() {
        self[i] = angle;
        angle += inc;
      }
      self
    }

    /// Sine: sin(2pi / table.len() * n)
    fn sine(&mut self) -> &mut Self::Output {
      let mut angle: f32 = 0.0;
      let inc: f32 = PI * 2.0 / self.len() as f32;
      for i in 0..self.len() {
        self[i] = angle.sin();
        angle += inc;
      }
      self
    }

    /// Square
    fn square(&mut self) -> &mut Self::Output {
      let mut val = -1.0;
      for i in 0..self.len() {
        self[i] = val;
        if self[i] == self.len() as f32/2.0-1.0 { val = 1.0; } 
      }
      self
    }

    /// Triangle
    fn triangle(&mut self) -> &mut Self::Output {
      let mut angle = 0.0;
      let mut inc: f32 = 2.0 / (self.len() as f32 / 2.0);
      for i in 0..self.len() {
        if angle >= 1.0 || angle <= -1.0 { inc = inc * -1.0; }
        self[i] = angle;
        angle += inc;
      }
      self
    }

    /// Sawtooth: 0.0 -> 1.0
    fn sawtooth(&mut self) -> &mut Self::Output {
      let mut angle: f32 = 0.0;
      let inc: f32 = 2.0 / (self.len() as f32 - 1.0);
      for i in 0..self.len() {
        self[i] = angle - 1.0;
        angle += inc;
      }
      self
    }
    

    /// Reverse sawtooth: 1.0 -> -1.0
    fn reverse_sawtooth(&mut self) -> &mut Self::Output {
      let mut angle: f32 = 0.0;
      let inc: f32 = 2.0 / (self.len() as f32 - 1.0);
      for i in 0..self.len() {
        self[i] = angle + 1.0;
        angle -= inc;
      }
      self
    }

    /// Create a complex waveform from amplitudes and phases of sine partials
    fn complex_sine<const M: usize> (&mut self, amps: [f32; M], phases: [f32; M]) -> &mut Self::Output {
      let mut n: f32 = 1.0;
      if amps.len() == phases.len() {
        for i in 0..amps.len() {
          let inc = PI * 2.0f32 * n / self.len() as f32;
          let mut angle = inc * self.len() as f32 * phases[i];
          for j in 0..self.len() {
            self[j] += angle.sin() * amps[i];
            angle += inc;
          }
          n += 1.0;
        }
        scale(self, -1.0f32, 1.0f32);
      }
      self
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::traits::Waveshape;
  #[test]
  fn test_phasor() {
    let x = [0.0; 8].phasor().to_owned();
    assert_eq!(x[0], 0.0);
  }
  
  #[test]
  fn test_phasor2() {
    let x = [0.0; 8].phasor().to_owned();
    assert_eq!(x[7], 1.0);
  }
}
