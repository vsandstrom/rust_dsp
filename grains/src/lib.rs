extern crate envelope;
extern crate buffer;
use core::marker::PhantomData;
use std::rc::Rc;
use envelope::Envelope;
use interpolation::interpolation::{Interpolation, Linear};
use buffer::Buffer;
use rand::Rng;

pub struct Grain<T> {
  buffer: Rc<Buffer<T>>,
  grain_env: Rc<Envelope<T>>,
  samplerate: Rc<f32>,
  buf_position: f32,
  env_position: f32,
  rate: f32,
  duration: f32,
  jitter: f32,
  random: f32,
  pub active: bool,
  interpolation: PhantomData<T>

}

#[allow(unused)]
pub struct Granulator<T> {
  buffer: Rc<Buffer<T>>,
  envelope: Rc<Envelope<T>>,
  samplerate: Rc<f32>,
  grains: Vec<Grain<T>>,
  interpolation: PhantomData<T>,
  position: f32,
  playback_rate: f32,
  num_grains: usize,
  grain_size: f32,
}

impl<T> Granulator<T> {
  pub fn new(buffer: Buffer<T>, grain_env: Envelope<T>, samplerate: f32, max_grains: usize) -> Self {
    let buffer = Rc::new(buffer);
    let grain_env = Rc::new(grain_env);
    let samplerate = Rc::new(samplerate);
    let mut grains: Vec<Grain<T>> = Vec::with_capacity(max_grains);
    for _ in 0..max_grains {
      grains.push(
        Grain { 
          buffer: Rc::clone(&buffer),
          grain_env: Rc::clone(&grain_env),
          samplerate: Rc::clone(&samplerate),
          buf_position: 0.0,
          env_position: 0.0,
          rate: 1.0,
          duration: 0.0533333,
          jitter: 0.0,
          random: 0.0,
          active: false,
          interpolation: PhantomData,
        }
      );
    }

    Granulator {
      buffer, 
      envelope: grain_env, 
      samplerate, 
      grains,
      position: 0.0,
      playback_rate: 1.0,
      num_grains: max_grains,
      grain_size: 0.2,
      interpolation: PhantomData 
    }
  }
  pub fn play() -> f32 {
    0.0
  }
}

impl<T: Interpolation> Grain<T> {

  pub fn play(&mut self) -> f32 {
    let mut out = self.buffer.read(self.buf_position);
    out *= self.grain_env.read(self.env_position);
    self.buf_position += self.rate + self.random * self.buffer.len() as f32;
    self.env_position += self.duration;
    if self.env_position >= self.buffer.len() as f32 {
      self.active = false;
    }
    out
  }

  pub fn set_jitter(&mut self, jitter: f32) {
    self.jitter = jitter;
  }
  
  pub fn set_duration(&mut self, duration: f32) {
    self.duration = self.grain_env.len() as f32 / (*self.samplerate) * duration;
  }

  pub fn set_rate(&mut self, rate: f32) {
    self.rate = rate;
  }

  pub fn set_random(&mut self) {
    self.random = rand::thread_rng().gen_range(0.0..=1.0)
  }
}

impl Granulator<Linear> {
}


#[cfg(test)]
mod tests {
    use super::*;
}
