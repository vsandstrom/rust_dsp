use crate::delay::{Delay, DelayTrait};
use crate::dsp::math::samples_to_wavelength;
use crate::dsp::signal;
use crate::dsp::signal::traits::SignalFloat;
use crate::interpolation::Linear;
use crate::filter::{onepole::Onepole, Filter, comb::LPComb};
use super::Verb;
use crate::noise::Noise;

// TANK PHASE DOES NOT SOUND GOOD YET

struct Diffuse {delay: Delay, noise: Noise, time: f32, coeff: f32}
struct Tank {delay: LPComb, coeff: f32}

pub struct VikVerb {
  diffuse: [Diffuse; 4],
  tank_l: [Tank; 2],
  tank_r: [Tank; 2],
  onepole: Onepole,
  diff_feedback: f32,
  tank_feedback: f32,
  prev: f32,
  prev_l: f32,
  prev_r: f32,
  mod_freq: f32,
}

impl VikVerb {
  pub fn new(samplerate: f32) -> Self {
    let mut onepole = Onepole::new();
    let ff = 0.3;
    let fb = 0.63;
    let mut tank_l = [
      Tank{delay: LPComb::new::<2819>(ff, fb), coeff: 1.0},
      Tank{delay: LPComb::new::<3889>(-ff, -fb), coeff: 1.0},
    ];
    let mut tank_r = [
      Tank{delay: LPComb::new::<3797>(ff, fb), coeff: 1.0},
      Tank{delay: LPComb::new::<2617>(-ff, -fb), coeff: 1.0},
    ];
    tank_l.iter_mut().for_each(|t| t.delay.set_damp(0.66));
    tank_r.iter_mut().for_each(|t| t.delay.set_damp(0.66));

    onepole.set_coeff(0.6);
    Self {
      diffuse: [
        Diffuse{delay: Delay::new(128), noise: Noise::new(samplerate), time: 58.3471, coeff: 0.425},
        Diffuse{delay: Delay::new(64), noise: Noise::new(samplerate), time: 43.22993, coeff: 0.785},
        Diffuse{delay: Delay::new(64), noise: Noise::new(samplerate), time: 34.287, coeff: 0.41},
        Diffuse{delay: Delay::new(64), noise: Noise::new(samplerate), time: 27.337, coeff: 0.507},
      ],
      tank_l,
      tank_r,
      onepole,
      diff_feedback: 0.35,
      tank_feedback: 0.4,
      prev: 0.0,
      prev_l: 0.0,
      prev_r: 0.0,
      mod_freq: 12.0,
    }
  }
}

impl Verb for VikVerb {
  fn process<T: super::Interpolation>(&mut self, sample: f32) -> f32 {
    let mut sig = sample + (self.prev * self.diff_feedback);
    let t = self.diffuse[0].time - self.diffuse[0].noise.play(0.05).map(-1.0, 1.0, 0.0, 33.2);
    sig = self.diffuse[0].delay.play::<Linear>(
      sig,
      t,
      0.16
    ) * self.diffuse[0].coeff;
    let t = self.diffuse[1].time - self.diffuse[1].noise.play(0.085).map(-1.0, 1.0, 0.0, 22.2);
    sig = self.diffuse[1].delay.play::<Linear>(
      sig,
      t,
      0.18
    ) * self.diffuse[1].coeff;
    let t = self.diffuse[2].time - self.diffuse[1].noise.play(0.0725) .map(-1.0, 1.0, 0.0, 17.2);
    sig = self.diffuse[2].delay.play::<Linear>(
      sig,
      t,
      0.22
    ) * self.diffuse[2].coeff;
    let t = self.diffuse[3].time - self.diffuse[1].noise.play(0.0135).map(-1.0, 1.0, 0.0, 9.2);
    sig = self.diffuse[3].delay.play::<Linear>(
      sig,
      t,
      0.32
    ) * self.diffuse[3].coeff;
    
    self.prev = self.onepole.process(sig);

    let mut left = self.tank_l[0].delay.process(self.prev - self.prev_r * self.tank_feedback)* self.tank_l[0].coeff;
    left = self.tank_l[1].delay.process(left)* self.tank_l[1].coeff;

    let mut right = self.tank_r[0].delay.process(self.prev - self.prev_l * self.tank_feedback)* self.tank_l[0].coeff;
    right = self.tank_r[1].delay.process(right)* self.tank_l[1].coeff;

    self.prev_l = left;
    self.prev_r = right;
    // self.prev += (self.prev_l+self.prev_r) * 0.015;
    
    self.prev_l + self.prev_r
  }
}
