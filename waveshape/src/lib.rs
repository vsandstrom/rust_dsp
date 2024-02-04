use std::f32::consts::PI;
use dsp::buffer::{normalize, scale};


/// Create a complex waveform from amplitudes and phases of sine partials
/// (tip: normalize amplitudes to get waveform within -1.0 - 1.0)
pub fn complex_sine<'a>(table: &'a mut Vec<f32>, amps: &'a Vec<f32>, phases: &'a Vec<f32>) -> &'a Vec<f32> {
  let len = table.len();
  let mut n: f32 = 1.0;
  if amps.len() == phases.len() {
    for i in 0..amps.len() {
      let inc = PI * 2.0f32 * n / len as f32;
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
pub fn sine(table: &mut Vec<f32>) -> &Vec<f32> {
  let mut angle: f32 = 0.0;
  let inc: f32 = PI * 2.0 / table.len() as f32;
  for i in 0..table.len() {
    table[i] = angle.sin();
    angle += inc;
  }
  table
}


/// Squared sinewave, positive bellcurve. Useful as envelope
pub fn hanning(table: &mut Vec<f32>) -> &Vec<f32> {
  let mut angle: f32 = 0.0;
  let inc: f32 = PI / (table.len() as f32);
  for i in 0..table.len() {
    table[i] = angle.sin().powf(2.0);
    angle += inc;
  }
  table
}

/// Square
pub fn square(table: &mut Vec<f32>) -> &Vec<f32> {
  let mut val = -1.0;
  for i in 0..table.len() {
    table[i] = val;
    if i == table.len()/2-1 { val = 1.0; } 
  }
  table
}

/// Triangle 
pub fn triangle(table: &mut Vec<f32>) -> &Vec<f32> {
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
pub fn sawtooth(table: &mut Vec<f32>) -> &Vec<f32> {
  let mut angle: f32 = 0.0;
  let inc: f32 = 2.0 / (table.len() as f32 - 1.0);
  for i in 0..table.len() {
    table[i] = angle - 1.0;
    angle += inc;
  }
  table
}

/// Reverse sawtooth: 1.0 -> -1.0
pub fn reverse_sawtooth(table: &mut Vec<f32>) -> &Vec<f32> {
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
  pub trait Waveshape {
    fn sine(self) -> Self;
    fn hanning(self) -> Self;
    fn triangle(self) -> Self;
    fn square(self) -> Self;
    fn sawtooth(self) -> Self;
    fn reverse_sawtooth(self) -> Self;
    fn complex_sine(self, amps: &Vec<f32>, phases: &Vec<f32>) -> Self;
  }

  impl Waveshape for Vec<f32>  {
    /// Squared sinewave, positive bellcurve. Useful as envelope
    fn hanning(mut self) -> Self {
      let mut angle: f32 = 0.0;
      let inc: f32 = PI / (self.len() as f32);
      for i in 0..self.len() {
        self[i] = angle.sin().powf(2.0);
        angle += inc;
      }
      self
    }

    /// Sine: sin(2pi / table.len() * n)
    fn sine(mut self) -> Self {
      let mut angle: f32 = 0.0;
      let inc: f32 = PI * 2.0 / self.len() as f32;
      for i in 0..self.len() {
        self[i] = angle.sin();
        angle += inc;
      }
      self
    }

    /// Square
    fn square(mut self) -> Self {
      let mut val = -1.0;
      for i in 0..self.len() {
        self[i] = val;
        if self[i] == self.len() as f32/2.0-1.0 { val = 1.0; } 
      }
      self
    }

    /// Triangle
    fn triangle(mut self) -> Self {
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
    fn sawtooth(mut self) -> Self {
      let mut angle: f32 = 0.0;
      let inc: f32 = 2.0 / (self.len() as f32 - 1.0);
      for i in 0..self.len() {
        self[i] = angle - 1.0;
        angle += inc;
      }
      self
    }

    /// Reverse sawtooth: 1.0 -> -1.0
    fn reverse_sawtooth(mut self) -> Self {
      let mut angle: f32 = 0.0;
      let inc: f32 = 2.0 / (self.len() as f32 - 1.0);
      for i in 0..self.len() {
        self[i] = angle + 1.0;
        angle -= inc;
      }
      self
    }

    /// Create a complex waveform from amplitudes and phases of sine partials
    fn complex_sine(mut self, amps: &Vec<f32>, phases: &Vec<f32>) -> Self {
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
        scale(&mut self, -1.0f32, 1.0f32);
      }
      self
    }
  }
}

#[cfg(test)]
mod tests {

}
