use crate::envelope::{EnvType, Envelope};
use crate::interpolation::Interpolation;

pub struct Granulator<const NUMGRAINS: usize, const BUFSIZE:usize> {
  buffer: Vec<f32>,
  buf_size: f32,

  envelope: Envelope,
  env_size: f32,
  rec_pos: usize,

  next_grain: usize,
  buf_positions: [f32; NUMGRAINS],
  env_positions: [f32; NUMGRAINS],
  durations: [f32; NUMGRAINS],
  rates: [f32; NUMGRAINS],
  active: [bool; NUMGRAINS],

  samplerate: f32,
}

impl<const NUMGRAINS:usize, const BUFSIZE: usize> Granulator<NUMGRAINS, BUFSIZE> {
  pub fn new<const N:usize, const M: usize>(env_shape: &EnvType<N, M>, samplerate: f32) -> Self {
    // Buffer to hold recorded audio
    let buffer = vec![0.0; BUFSIZE];
    let envelope = Envelope::new(&env_shape, samplerate);
    let durations = [calc_duration(envelope.len(), samplerate, 0.2); NUMGRAINS];
    let buf_positions = [0.0; NUMGRAINS];
    let env_positions = [0.0; NUMGRAINS];
    let rates = [1.0; NUMGRAINS];
    let active = [false; NUMGRAINS];

    Self {
      buffer,
      buf_size: BUFSIZE as f32,
      env_size: envelope.len() as f32,
      // grains,
      envelope,
      rec_pos: 0,
      env_positions,
      buf_positions, 
      next_grain: 0,
      durations,
      samplerate,
      rates,
      active
    }
  }


  #[inline]
  pub fn play<BufferInterpolation, EnvelopeInterpolation>( &mut self,
    position: f32,
    duration: f32,
    rate: f32,
    jitter: f32,
    trigger:f32
  ) -> f32
  where BufferInterpolation: Interpolation,
        EnvelopeInterpolation: Interpolation {

    // TRIGGER GRAIN 
    if trigger >= 1.0 && !self.active[self.next_grain] { 
      // normalize buffer position
      let pos = match (position + jitter).fract() {
        x if x < 0.0 => { (1.0 + x) * self.buf_size },
        x            => { x  * self.buf_size }
      };
      // set parameters for grain
      self.buf_positions[self.next_grain] = pos;
      self.env_positions[self.next_grain] = 0.0;
      self.rates        [self.next_grain] = rate;
      self.durations    [self.next_grain] = calc_duration(self.envelope.len(), self.samplerate, duration);
      // set grain to active
      self.active       [self.next_grain] = true;
      // increment and wait for next trigger
      self.next_grain = (self.next_grain + 1) % NUMGRAINS;
    }


    let mut out = 0.0;
    for i in 0..NUMGRAINS {
      // accumulate output of active grains
      if self.active[i] {
        let sig = BufferInterpolation::interpolate(self.buf_positions[i], &self.buffer, BUFSIZE);
        let env = self.envelope.read::<EnvelopeInterpolation>(self.env_positions[i]);
        self.buf_positions[i] += self.rates[i];
        self.env_positions[i] += self.durations[i];
        // if the grain has reached the envelopes end, deactivate
        if self.env_positions[i] >= self.env_size { self.active[i] = false; }
        out += sig * env;
      } 
    }
    out
  }

  pub fn record(&mut self, sample: f32) -> Option<f32> {
    if self.rec_pos == BUFSIZE { return None; }
    self.buffer[self.rec_pos] = sample;
    self.rec_pos += 1;
    Some(sample)
  }

  pub fn update_envelope<const N: usize, const M: usize>(&mut self, env_shape: &EnvType<N, M>) {
    self.envelope.new_shape(&env_shape, self.samplerate);
    self.env_size = self.envelope.len() as f32;
  }

  pub fn set_samplerate(&mut self, samplerate: f32) {
    self.samplerate = samplerate;
  }

  #[inline]
  pub fn reset_record(&mut self) {
    self.rec_pos = 0;
  }
}
  
#[inline]
fn calc_duration(env_len: usize, samplerate: f32, duration: f32) -> f32{
  env_len as f32 / ((samplerate) * duration)
}
