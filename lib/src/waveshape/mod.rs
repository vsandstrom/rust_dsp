pub mod traits;
pub mod macros;
use core::f32::consts::{PI, TAU};
use crate::dsp::buffer::scale;
use alloc::{vec::Vec, borrow::ToOwned};

/// Create a complex waveform from amplitudes and phases of sine partials
/// (tip: normalize amplitudes to get waveform within -1.0 - 1.0)
pub fn complex_sine(table: &mut [f32], amps: &[f32], phases: &[f32]) {
  let len = table.len();
  let mut n: f32 = 1.0;
  if amps.len() == phases.len() {
    for i in 0..amps.len() {
      let inc = TAU * n / len as f32;
      let mut angle = inc * len as f32 * phases[i];
      for sample in table.iter_mut() {
        *sample += angle.sin() * amps[i];
        angle += inc;
      }
      n += 1.0;
    }
    scale(table, -1.0f32, 1.0f32);
  }
}
/// Sine: sin(2pi / table.len() * n)
pub fn sine(table: &mut [f32]) {
  let mut angle: f32 = 0.0;
  let inc: f32 = TAU / table.len() as f32;
  for sample in table.iter_mut() {
    *sample = angle.sin();
    angle += inc;
  }
}


/// Squared sinewave, positive bellcurve. Useful as envelope
pub fn hanning(table: &mut [f32]) {
  let mut angle: f32 = 0.0;
  let inc: f32 = PI / (table.len() as f32);
  for sample in table.iter_mut() {
    *sample = angle.sin().powf(2.0);
    angle += inc;
  }
}


/// Square
pub fn square(table: &mut [f32]) {
  let mut val = -1.0;
  let len = table.len();
  for (i, sample) in table.iter_mut().enumerate() {
    *sample = val;
    if i == len/2-1 { val = 1.0; } 
  }
}

/// Triangle 
pub fn triangle(table: &mut [f32]) {
  let mut angle = 0.0;
  let mut inc: f32 = 2.0 / (table.len() as f32 / 2.0);
  for sample in table.iter_mut() {
    if angle >= 1.0 || angle <= -1.0 { inc *= -1.0; }
    *sample = angle;
    angle += inc;
  }
}

/// Sawtooth: -1.0 -> 1.0
pub fn sawtooth(table: &mut [f32]) {
  let mut angle: f32 = 0.0;
  let inc: f32 = 2.0 / (table.len() as f32 - 1.0);
  for sample in table.iter_mut() {
    *sample = angle - 1.0;
    angle += inc;
  }
}

/// Reverse sawtooth: 1.0 -> -1.0
pub fn reverse_sawtooth(table: &mut [f32]) {
  let mut angle: f32 = 0.0;
  let inc: f32 = 2.0 / (table.len() as f32 - 1.0);
  for sample in table.iter_mut() {
    *sample = angle + 1.0;
    angle -= inc;
  }
}
    
pub fn phasor(table: &mut [f32]) {
  let mut angle: f32 = 0.0;
  let inc: f32 = 1.0 / (table.len() as f32 - 1.0);
  for sample in table.iter_mut() {
    *sample = angle;
    angle += inc;
  }
}



#[cfg(test)]
mod tests {
  use super::*;
  use crate::waveshape::traits::Waveshape;
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
