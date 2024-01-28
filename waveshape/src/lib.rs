use std::f32::consts::PI;
use core::marker::PhantomData;
use dsp::buffer::{normalize, scale};

pub struct Sine;
pub struct Triangle;
pub struct Square;
pub struct Sawtooth;
pub struct Hanning;
pub struct RevSawtooth;
pub struct User;

pub trait Waveshape {
  fn create(table: &mut Vec<f32>, length: usize);
}

impl Waveshape for Sine {
  fn create(table: &mut Vec<f32>, length: usize) {
    let mut angle: f32 = 0.0;
    let inc: f32 = PI * 2.0 / length as f32;
    for _ in 0..length {
      table.push(angle.sin());
      angle += inc;
    }
    table.push(0.0);
  }
}

impl Waveshape for Triangle {
  fn create(table: &mut Vec<f32>, length: usize) {
    let mut angle = 0.0;
    let mut inc: f32 = 2.0 / (length as f32 / 2.0);
    for _ in 0..length {
      if angle >= 1.0 || angle <= -1.0 { inc = inc * -1.0; }
      table.push(angle);
      angle += inc;
    }
    table.push(0.0);
  }
}

impl Waveshape for Square {
  fn create(table: &mut Vec<f32>, length: usize) {
    let mut val = -1.0;
    for i in 0..length {
      table.push(val);
      if i == length/2-1 { val = 1.0; } 
    }
    table.push(0.0);
  }
}

impl Waveshape for Sawtooth {
  fn create(table: &mut Vec<f32>, length: usize) {
    let mut angle: f32 = 0.0;
    let inc: f32 = 2.0 / (length as f32 - 1.0);
    for _ in 0..length {
      table.push(angle - 1.0);
      angle += inc;
    }
    table.push(0.0);
  }
}
impl Waveshape for RevSawtooth {
  fn create(table: &mut Vec<f32>, length: usize) {
    let mut angle: f32 = 0.0;
    let inc: f32 = 2.0 / (length as f32 - 1.0);
    for _ in 0..length {
      table.push(angle + 1.0);
      angle -= inc;
    }
    table.push(0.0);
  }
}

impl Waveshape for Hanning {
  fn create(table: &mut Vec<f32>, length: usize) {
    let mut angle: f32 = 0.0;
    let inc: f32 = PI / (length as f32);
    for _ in 0..length {
      table.push(angle.sin().powf(2.0));
      angle += inc;
    }
    table.push(0.0);
  }
}

impl Waveshape for User {
  fn create(_table: &mut Vec<f32>, _length: usize) {
    panic!("dummy method for use when Wavetable is using the ::from() method")
  }
}

pub fn complex_sine(
  table: &mut Vec<f32>,
  length: usize,
  amps: &mut Vec<f32>,
  phases: &Vec<f32>) 
{
  normalize(amps);
  let mut n: f32 = 1.0;
  while let Some((amp, phs)) = amps.iter().zip(phases.into_iter()).next() {
    let inc = PI * 2.0f32 * n / length as f32;
    let mut angle = inc * length as f32 * phs;
    for _ in 0..length {
      table.push(angle.sin() * amp);
      angle += inc;
    }
    n += 1.0;
  }
  table.push(0.0);
  scale(table, -1.0f32, 1.0f32);
}

#[cfg(test)]
mod tests {
    use super::*;

}
