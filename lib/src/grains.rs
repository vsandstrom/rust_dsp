use crate::interpolation::Interpolation;
use alloc::{vec, vec::Vec};

pub trait GrainTrait {
  fn record(&mut self, _sample: f32) -> Option<f32> {None}
  fn update_envelope(&mut self, shape: Vec<f32>);
  fn set_samplerate(&mut self, samplerate: f32);
  fn reset_record(&mut self);
  fn set_buffersize(&mut self, size: usize) {}
}

pub trait ADSRGrainTrait {

}

#[derive(Clone, Copy)]
struct Grain {
  buf_position: f32,
  env_position: f32,
  duration: f32,
  rate: f32,
  active: bool
}

pub struct Granulator {
  buffer: Vec<f32>,
  buf_size: usize,

  envelope: Vec<f32>,
  env_size: usize,
  rec_pos: usize,
  pub recording: bool,

  next_grain: usize,
  grains: Vec<Grain>,

  samplerate: f32,
  sr_recip: f32,
}


impl Granulator {
  pub fn new(envelope: Vec<f32>, samplerate: f32, num_grains: usize, buf_size: usize) -> Self {
    // Buffer to hold recorded audio
    let buffer = vec![0.0; buf_size];

    let grains = vec![
      Grain {
        duration: 0.0,
        buf_position: 0.0,
        env_position: 0.0,
        rate: 1.0,
        active: false
      }; num_grains];

    Self {
      buffer,
      buf_size,
      env_size: envelope.len(),
      grains,
      envelope,
      rec_pos: 0,
      recording: false,
      next_grain: 0,
      samplerate,
      sr_recip: 1.0 / samplerate,
    }
  }


  #[inline]
  pub fn play<BufferInterpolation, EnvelopeInterpolation>(&mut self) -> f32
  where BufferInterpolation: Interpolation,
        EnvelopeInterpolation: Interpolation {
    let mut out = 0.0;
    for g in self.grains.iter_mut() {
      // if the grain has reached the envelopes end, deactivate
      if g.env_position >= self.env_size as f32 { g.active = false; continue;}
      // accumulate output of active grains
      if g.active {
        let sig = BufferInterpolation::interpolate(g.buf_position, &self.buffer, self.buf_size);
        // inline lerp of grain envelope
        let env_a = self.envelope[g.env_position as usize];
        let env_b = self.envelope[(g.env_position as usize + 1) % self.env_size];
        let x = g.env_position.fract();
        let env = env_a + x * ( env_b - env_a );
        g.buf_position += g.rate;
        g.env_position += g.duration;
        out += sig * env;
      } 
    }
    out
  }

  #[inline]
  pub fn trigger_new(&mut self,
    position: f32,
    duration: f32,
    rate: f32,
    jitter: f32,
  ) -> bool {
    if let Some(g) = self.grains.get_mut(self.next_grain) {
      // guard for triggering already active grain
      if g.active { return false }
      // set parameters for grain
      g.buf_position = wrap_position(position + jitter, self.buf_size);
      g.env_position = 0.0;
      g.rate         = rate;
      g.duration     = calc_duration(
        self.env_size, 
        self.sr_recip, 
        1.0/duration
      );
      g.active       = true;
    }
    // set grain to active
    // increment and wait for next trigger
    self.next_grain = (self.next_grain + 1) % self.grains.len();
    true
  }

}
  
#[inline]
fn wrap_position(position: f32, bufsize: usize) -> f32 {
  let b = bufsize as f32;
  match position.fract() {
    x if x < 0.0 => { (1.0 + x) * b},
    x            => { x  * b}
  }
}

impl GrainTrait for Granulator {
  #[inline]
  fn record(&mut self, sample: f32) -> Option<f32> {
    if self.rec_pos == self.buf_size { return None; }
    self.buffer[self.rec_pos] = sample;
    self.rec_pos += 1;
    Some(sample)
  }

  #[inline]
  fn update_envelope(&mut self, shape: Vec<f32>) {
    self.env_size = shape.len();
    self.envelope = shape;
  }

  fn set_samplerate(&mut self, samplerate: f32) {
    self.samplerate = samplerate;
    self.sr_recip = 1.0 / samplerate;
  }

  fn reset_record(&mut self) {
    self.rec_pos = 0;
  }

  fn set_buffersize(&mut self, size: usize) {
    self.buffer = vec![0.0; size];
    self.buf_size = size;
  }
}


pub mod stereo {
  use super::*;
  use crate::dsp::signal::pan_exp2;
  use GrainTrait;

  #[derive(Clone, Copy)]
  pub (super) struct Grain {
   pub (super) buf_position: f32,
   pub (super) env_position: f32,
   pub (super) duration: f32,
   pub (super) pan: (f32, f32),
   pub (super) rate: f32,
   pub (super) active: bool
  }

  pub struct Granulator {
    buffer: &'static [f32],
    buf_size: usize,
    out: [f32; 2],

    envelope: Vec<f32>,
    env_size: usize,
    rec_pos: usize,
    pub recording: bool,

    next_grain: usize,
    grains: Vec<Grain>,

    samplerate: f32,
    sr_recip: f32,
  }

  impl Granulator {
    pub fn new(buffer: &'static [f32], shape: Vec<f32>, samplerate: f32, num_grains: usize) -> Self {
    // Buffer to hold recorded audio
    let buf_size = buffer.len();

    let grains = vec![
      Grain {
        duration: 0.0,
        buf_position: 0.0,
        env_position: 0.0,
        pan: (0.0, 0.0),
        rate: 1.0,
        active: false
      }; num_grains];

    Self {
      buffer,
      buf_size,
      env_size: shape.len(),
      out: [0.0; 2],
      grains,
      envelope: shape,
      rec_pos: 0,
      recording: false,
      next_grain: 0,
      samplerate,
      sr_recip: 1.0 / samplerate,
    }
  }


  #[inline]
  pub fn play<BufferInterpolation, EnvelopeInterpolation>( &mut self) -> &[f32; 2]
  where BufferInterpolation: Interpolation,
        EnvelopeInterpolation: Interpolation {
    self.out = [0.0;2];
    for g in self.grains.iter_mut() {
      // if the grain has reached the envelopes end, deactivate
      if g.env_position >= self.env_size as f32 { g.active = false; continue;}
      // accumulate output of active grains
      if g.active {
        let sig = BufferInterpolation::interpolate(g.buf_position, &self.buffer, self.buf_size);
        let env = EnvelopeInterpolation::interpolate(g.env_position, &self.envelope, self.env_size);
        g.buf_position += g.rate;
        g.env_position += g.duration;
        self.out[0] += sig * g.pan.0 * env;
        self.out[1] += sig * g.pan.1 * env;
      } 
    }
    &self.out
  }

  #[inline]
  pub fn trigger_new(&mut self,
    position: f32,
    duration: f32,
    pan: f32,
    rate: f32,
    jitter: f32,
  ) -> bool {
    if let Some(g) = self.grains.get_mut(self.next_grain) {
      // guard for triggering already active grain
      if g.active { return false }
      // set parameters for grain
      g.buf_position = wrap_position(position + jitter, self.buf_size);
      g.env_position = 0.0;
      g.rate         = rate;
      g.pan          = pan_exp2(pan);
      g.duration     = calc_duration(
        self.env_size, 
        self.sr_recip, 
        1.0/duration
      );
      g.active       = true;
    }
    // set grain to active
    // increment and wait for next trigger
    self.next_grain = (self.next_grain + 1) % self.grains.len();
    true
  }
}


  impl GrainTrait for Granulator {
    #[inline]
    // fn record(&mut self, sample: f32) -> Option<f32> {
    //   if self.rec_pos == self.buf_size { return None; }
    //   self.buffer[self.rec_pos] = sample;
    //   self.rec_pos += 1;
    //   Some(sample)
    // }

    #[inline]
    fn update_envelope(&mut self, shape: Vec<f32>) {
      self.env_size = shape.len();
      self.envelope = shape;
    }

    fn set_samplerate(&mut self, samplerate: f32) {
      self.samplerate = samplerate;
      self.sr_recip = 1.0 / samplerate;
    }

    fn reset_record(&mut self) {
      self.rec_pos = 0;
    }

    // #[inline]
    // fn set_buffersize(&mut self, size: usize) {
    //   self.buffer = vec![0.0; size];
    //   self.buf_size = size;
    // }
  }
}

pub mod static_stereo {
  use core::array;

use super::*;
  use super::stereo::Grain;
  use crate::dsp::signal::pan_exp2;
  use GrainTrait;

  pub struct Granulator<const BUFSIZE: usize, const NUMGRAINS: usize> {
    buffer: [f32;BUFSIZE],
    buf_size: usize,
    out: [f32; 2],

    envelope: Vec<f32>,
    env_size: usize,
    rec_pos: usize,
    pub recording: bool,

    next_grain: usize,
    grains: [Grain; NUMGRAINS],

    samplerate: f32,
    sr_recip: f32,
  }

  impl<const BUFSIZE: usize, const NUMGRAINS: usize> Granulator<BUFSIZE, NUMGRAINS> {
    pub fn new(shape: Vec<f32>, samplerate: f32) -> Self {
    // Buffer to hold recorded audio
    let buffer = [0.0; BUFSIZE];
    let grains = array::from_fn(|_|
      Grain {
        duration: 0.0,
        buf_position: 0.0,
        env_position: 0.0,
        pan: (0.0, 0.0),
        rate: 1.0,
        active: false
      });

    Self {
      buffer,
      buf_size: BUFSIZE,
      env_size: shape.len(),
      out: [0.0; 2],
      grains,
      envelope: shape,
      rec_pos: 0,
      recording: false,
      next_grain: 0,
      samplerate,
      sr_recip: 1.0 / samplerate,
    }
  }

  #[inline]
  pub fn play<BufferInterpolation, EnvelopeInterpolation>( &mut self) -> &[f32; 2]
  where BufferInterpolation: Interpolation,
        EnvelopeInterpolation: Interpolation {
    self.out = [0.0;2];
    for g in self.grains.iter_mut() {
      // if the grain has reached the envelopes end, deactivate
      if g.env_position >= self.env_size as f32 { g.active = false; continue;}
      // accumulate output of active grains
      if g.active {
        let sig = BufferInterpolation::interpolate(g.buf_position, &self.buffer, self.buf_size);
        let env = EnvelopeInterpolation::interpolate(g.env_position, &self.envelope, self.env_size);
        g.buf_position += g.rate;
        g.env_position += g.duration;
        self.out[0] += sig * g.pan.0 * env;
        self.out[1] += sig * g.pan.1 * env;
      } 
    }
    &self.out
  }

  #[inline]
  pub fn trigger_new(&mut self,
    position: f32,
    duration: f32,
    pan: f32,
    rate: f32,
    jitter: f32,
  ) -> bool {
    if let Some(g) = self.grains.get_mut(self.next_grain) {
      // guard for triggering already active grain
      if g.active { return false }
      // set parameters for grain
      g.buf_position = wrap_position(position + jitter, self.buf_size);
      g.env_position = 0.0;
      g.rate         = rate;
      g.pan          = pan_exp2(pan);
      g.duration     = calc_duration(
        self.env_size, 
        self.sr_recip, 
        1.0/duration
      );
      g.active       = true;
    }
    // set grain to active
    // increment and wait for next trigger
    self.next_grain = (self.next_grain + 1) % self.grains.len();
    true
  }
}


  impl<const BUFSIZE: usize, const NUMGRAINS: usize> GrainTrait for Granulator<BUFSIZE, NUMGRAINS> {
    #[inline]
    fn record(&mut self, sample: f32) -> Option<f32> {
      if self.rec_pos == self.buf_size { return None; }
      self.buffer[self.rec_pos] = sample;
      self.rec_pos += 1;
      Some(sample)
    }

    #[inline]
    fn update_envelope(&mut self, shape: Vec<f32>) {
      self.env_size = shape.len();
      self.envelope = shape;
    }

    fn set_samplerate(&mut self, samplerate: f32) {
      self.samplerate = samplerate;
      self.sr_recip = 1.0 / samplerate;
    }

    fn reset_record(&mut self) {
      self.rec_pos = 0;
    }
  }
}
  
#[inline]
fn calc_duration(env_len: usize, samplerate_recip: f32, duration_recip: f32) -> f32{
  env_len as f32 * samplerate_recip * duration_recip
}
