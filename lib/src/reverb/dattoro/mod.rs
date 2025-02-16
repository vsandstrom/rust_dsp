use core::ops::Mul;

use crate::delay::{Delay, DelayTrait, FixedDelay};
use crate::dsp::buffer::traits::SignalVector;
use crate::filter::{Filter, Comb};
use super::Interpolation;
use crate::interpolation::Linear;

pub struct DattVerb {
  predelay_line: PreDelay,
  prev: f32,
  
  diffuser: [Comb; 4],

  pub predelay: f32,
  pub bandwidth: f32,
  pub decay: f32,
  input_diffusion_1: f32,
  input_diffusion_2: f32,
  decay_diffusion_1: f32,
  decay_diffusion_2: f32,
  damping: f32,
}

impl Default for DattVerb {
  fn default() -> Self {
    Self {
      // Predelay
      predelay_line: PreDelay::new(48000),
      predelay: 0.1,
      // Bandwidth filter
      bandwidth: 0.9995,
      prev: 0.0,
      // Input Diffuser
      input_diffusion_1: 0.75,
      input_diffusion_2: 0.625,
      diffuser: [
        Comb::new::<142>(
          0.0, 
          input_diffusion_1, 
          input_diffusion_1),
        Comb::new::<107>(
          0.0,
          input_diffusion_1,
          input_diffusion_1),
        Comb::new::<379>(
          0.0,
          input_diffusion_2,
          input_diffusion_2),
        Comb::new::<277>(
          0.0,
          input_diffusion_2,
          input_diffusion_2)
      ],

      // Tank
      decay: 0.5,
      damping: 0.0005,
      decay_diffusion_1: 0.7,
      decay_diffusion_2: 0.5,

    }
  }
}

impl DattVerb {
  pub fn new() -> Self {
    let input_diffusion_1 = 0.75;
    let input_diffusion_2 = 0.625;
    let decay_diffusion_1 = 0.7;
    let decay_diffusion_2 = 0.5;

    Self {
      // Predelay
      predelay_line: PreDelay::new(48000),
      predelay: 0.1,
      // Bandwidth filter
      bandwidth: 0.9995,
      prev: 0.0,
      // Input Diffuser
      input_diffusion_1: 0.75,
      input_diffusion_2: 0.625,
      diffuser: [
        Comb::new::<142>(
          0.0, 
          input_diffusion_1, 
          input_diffusion_1),
        Comb::new::<107>(
          0.0,
          input_diffusion_1,
          input_diffusion_1),
        Comb::new::<379>(
          0.0,
          input_diffusion_2,
          input_diffusion_2),
        Comb::new::<277>(
          0.0,
          input_diffusion_2,
          input_diffusion_2)
      ],

      // Tank
      decay: 0.5,
      damping: 0.0005,
      decay_diffusion_1,
      decay_diffusion_2,

    }
  }




  pub fn process(&mut self, samples: &[f32; 2]) -> f32 {
    // Predelay
    let mut sig = samples.sum().mul(0.5);
    sig = self.predelay_line.play::<Linear>(sig, self.predelay);
    // Bandwidth
    let temp = sig;
    sig = sig * self.bandwidth + (self.prev * self.bandwidth * -1.0);
    self.prev = temp;

    // Input diffusers:
    for c in self.diffuser.iter_mut() {
      sig = c.process(sig);
    }

    // TANK
    sig
  }
}


  // /// Set delay time in samples
  // fn set_time(&mut self, delay: f32) {
  //   self.delay = delay;
  // }

struct PreDelay { 
  buffer: Vec<f32>,
  position: usize,
}

impl PreDelay {
  pub fn play<T: Interpolation>(&mut self, input: f32, delay: f32) -> f32 {
    let len = self.buffer.len() as f32;
    let mut time = self.position as f32 + delay;
    while time >= len { time -= len };
    while time < 0.0  { time += len };
    let out = T::interpolate(time, &self.buffer, self.buffer.len());
    self.position %= self.buffer.len();
    self.buffer[self.position] = input;
    self.position += 1;
    out
  }
}

impl DelayTrait for PreDelay {
  /// Delay in samples
  fn new(max_samples: usize) -> Self {
    PreDelay{
      buffer: vec![0.0; max_samples],
      position: 0,
    }
  }
}

#[derive(Default)]
pub struct Onepole {
  prev: f32,
  damp: f32
}

impl Onepole {
  pub fn new() -> Self {
    Self {
      prev: 0.0,
      damp: 0.0
    }
  }
  fn process(&mut self, sample: f32) -> f32 {
    self.prev = (self.damp * sample) + ((1.0 - self.damp) * self.prev);
    self.prev
  }

  fn set_damp(&mut self, damp: f32) {
    self.damp = damp;
  }
}
