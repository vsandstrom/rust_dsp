extern crate envelope; 
extern crate buffer;
use core::marker::PhantomData;
use std::rc::Rc;
use envelope::Envelope;
use interpolation::interpolation::Interpolation;
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
  position: f32,
  playback_rate: f32,
  num_grains: usize,
  max_grains: usize,
  grain_size: f32,
  interpolation: PhantomData<T>,
}

impl<T: Interpolation> Granulator<T> {
  // Interpolation trait allows Buffer, Envelope and Granulator to use different interpolation
  // methods that fit the method signature. Grain will inherit the Granulators T
  /// Creates a new Granulator, with a Buffer of fixed size and an Envelope for Grain volume
  pub fn new(buffer: Buffer<T>, grain_env: Envelope<T>, samplerate: f32, max_grains: usize) -> Self {
    // Shared pointers between grains
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
      max_grains,
      grain_size: 0.2,
      interpolation: PhantomData 
    }
  }

  /// Internal play method when no trigger has been detected.
  fn idle_play(&mut self) -> f32 {
    let mut out = 0.0;
    for i in 0..self.max_grains {
      out += self.grains[i].play();
      // update values in grains. 
    self.grains[i].incr_ptrs();
      if self.grains[i].buf_position >= self.buffer.len() as f32 {
        self.grains[i].env_position = 0.0;
        self.grains[i].active = false;
      }
    }
    out
  }

  /// takes a trigger generator ( trigger >= 1.0 ) and a buffer position ( 0.0..=1.0 )
  pub fn play(&mut self, position: f32, trigger: f32) -> f32 {
    if trigger < 1.0 {return self.idle_play()}
    let mut out: f32 = 0.0;
    let mut triggered = false;
    // find next available grain to play
    for i in 0..self.max_grains {
      // accumulate all active
      if self.grains[i].active {
        out += self.grains[i].play();
      }
      // activate new grain and set to active
      if !triggered && !self.grains[i].active {
        let rng: f32 = rand::thread_rng().gen_range(0.0..=1.0);
        self.grains[i].buf_position = position * (self.grains[i].jitter * rng);
        self.grains[i].active = true;
        out += self.grains[i].play();
        triggered = true;
      }
    }
    out
  }

  pub fn set_jitter(&mut self, jitter: f32) {
    for i in 0..self.grains.len() {
      self.grains[i].set_jitter(jitter);
    }
  }

  pub fn set_duration(&mut self, duration: f32) {
    for i in 0..self.grains.len() {
      self.grains[i].set_duration(duration);
    }
  }

  pub fn set_rate(&mut self, rate: f32) {
    for i in 0..self.grains.len() {
      self.grains[i].set_rate(rate)
    }
  }
}

impl<T: Interpolation> Grain<T> {
  pub fn incr_ptrs(&mut self) {
    self.buf_position += self.rate + self.random * self.buffer.len() as f32;
    self.env_position += self.duration;
  }

  /// Returns signal from Grain. Must call self.incr_ptrs after play!
  pub fn play(&self) -> f32 {
    let mut out = self.buffer.read(self.buf_position);
    out *= self.grain_env.read(self.env_position);
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
