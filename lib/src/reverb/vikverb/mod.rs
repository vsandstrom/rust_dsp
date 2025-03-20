use crate::delay::{Delay, DelayTrait};
use crate::dsp::math::samples_to_wavelength;
use crate::dsp::signal;
use crate::dsp::signal::traits::SignalFloat;
use crate::interpolation::Linear;
use crate::filter::{onepole::Onepole, Filter, comb::{Comb, LPComb}};
use super::Verb;
use crate::noise::Noise;

// TANK PHASE DOES NOT SOUND GOOD YET

struct Diffuse {delay: Delay, noise: Noise, time: f32, coeff: f32}
struct Tank {
  allpass1: Comb, coeff1: f32,
  delay: Delay, noise: Noise, time: f32, coeff3: f32,
  allpass2: Comb, coeff2: f32
}

pub struct VikVerb {
  diffuse: [Diffuse; 4],
  tank_l: Tank,
  tank_r: Tank,
  onepole: Onepole,
  diff_feedback: f32,
  tank_feedback: f32,
  prev: f32,
  prev_l: f32,
  prev_r: f32,
  mod_freq: f32,
  mod_amount: f32,
}

impl VikVerb {
  pub fn new(samplerate: f32) -> Self {
    let mut onepole = Onepole::new();
    let ff = 0.63;
    let fb = 0.63;
    let tank_l = 
      Tank{allpass1: Comb::new::<2819>(ff, fb), coeff1: 1.0,
      delay: Delay::new(4096), noise: Noise::new(samplerate), time: 3639.8, coeff3: 0.7,
      allpass2: Comb::new::<3889>(-ff, -fb), coeff2: 1.0};
    let tank_r = 
      Tank{allpass1: Comb::new::<3797>(ff, fb), coeff1: 1.0,
      delay: Delay::new(4096), noise: Noise::new(samplerate), time: 3339.5, coeff3: 0.7,
      allpass2: Comb::new::<2617>(-ff, -fb), coeff2: 1.0};

    // tank_l.allpass1.set_damp(0.1);
    // tank_r.allpass1.set_damp(0.1);
    // tank_l.allpass2.set_damp(0.1);
    // tank_r.allpass2.set_damp(0.1);

    onepole.set_coeff(0.45);
    let size = 0.6;
    Self {
      diffuse: [
        Diffuse{
          delay: Delay::new(64), 
          noise: Noise::new(samplerate),
          time: 27.337 * size,
          coeff: 0.507
        },
        Diffuse{
          delay: Delay::new(64), 
          noise: Noise::new(samplerate),
          time: 34.287 * size,
          coeff: 0.51
        },
        Diffuse{
          delay: Delay::new(64), 
          noise: Noise::new(samplerate),
          time: 43.22993 * size,
          coeff: 0.685
        },
        Diffuse{
          delay: Delay::new(128),
          noise: Noise::new(samplerate),
          time: 58.3471 * size,
          coeff: 0.625
        },
      ],
      tank_l,
      tank_r,
      onepole,
      diff_feedback: 0.35,
      tank_feedback: 0.2,
      prev: 0.0,
      prev_l: 0.0,
      prev_r: 0.0,
      mod_freq: 12.0,
      mod_amount: 0.01,
    }
  }
}

impl Verb for VikVerb {
  fn process<T: super::Interpolation>(&mut self, sample: f32) -> f32 {
    let mut sig = sample + (self.prev * self.diff_feedback);
    let t = self.diffuse[0].time - self.diffuse[0].noise.play(0.05).map(-1.0, 1.0, 0.0, self.diffuse[0].time * self.mod_amount);
    sig = self.diffuse[0].delay.play::<Linear>(
      sig,
      t,
      0.16
    ) * self.diffuse[0].coeff;
    let t = self.diffuse[1].time - self.diffuse[1].noise.play(0.085).map(-1.0, 1.0, 0.0, self.diffuse[1].time * self.mod_amount);
    sig = self.diffuse[1].delay.play::<Linear>(
      sig,
      t,
      0.18
    ) * self.diffuse[1].coeff;
    let t = self.diffuse[2].time - self.diffuse[1].noise.play(0.0725) .map(-1.0, 1.0, 0.0, self.diffuse[1].time * self.mod_amount);
    sig = self.diffuse[2].delay.play::<Linear>(
      sig,
      t,
      0.22
    ) * self.diffuse[2].coeff;
    let t = self.diffuse[3].time - self.diffuse[1].noise.play(0.0135).map(-1.0, 1.0, 0.0, self.diffuse[1].time * self.mod_amount);
    sig = self.diffuse[3].delay.play::<Linear>(
      sig,
      t,
      0.32
    ) * self.diffuse[3].coeff;
    
    self.prev = self.onepole.process(sig);

    let mut left = self.tank_l.allpass1.process(self.prev - self.prev_r * self.tank_feedback)* self.tank_l.coeff1;
    left = self.tank_l.delay.play::<Linear>(
      left,
      self.tank_l.time - self.tank_l.noise.play(1.0/13.0).map(-1.0, 1.0, 0.0, 12.0 * self.mod_amount),
      0.3
      ) * self.tank_l.coeff3;
    left = self.tank_l.allpass2.process(left)* self.tank_l.coeff2;

    let mut right = self.tank_l.allpass1.process(
      self.prev - self.prev_l * self.tank_feedback
      )* self.tank_r.coeff1;

    right = self.tank_r.delay.play::<Linear>(
      right,
      self.tank_r.time - self.tank_r.noise.play(1.0/14.0)
      .map(-1.0, 1.0, 0.0, 13.9 * self.mod_amount),
      0.3
      ) * self.tank_r.coeff3;
    right = self.tank_r.allpass2.process(right)* self.tank_r.coeff2;

    self.prev_l = left;
    self.prev_r = right;
    // self.prev += (self.prev_l+self.prev_r) * 0.015;
    
    self.prev_l + self.prev_r * 10.0
  }
}
