use crate::envelope::{EnvType, Envelope};
use crate::interpolation::Interpolation;

struct Token {
  buf_position: f32,
  env_position: f32,
  duration: f32,
  rate: f32,
  active: bool
}

pub struct Granulator<const NUMGRAINS: usize, const BUFSIZE:usize> {
  buffer: Vec<f32>,
  buf_size: f32,

  envelope: Envelope,
  env_size: f32,
  rec_pos: usize,
  pub recording: bool,

  next_grain: usize,
  grains: [Token; NUMGRAINS],
  // buf_positions: [f32; NUMGRAINS],
  // env_positions: [f32; NUMGRAINS],
  // durations: [f32; NUMGRAINS],
  // rates: [f32; NUMGRAINS],
  // active: [bool; NUMGRAINS],

  samplerate: f32,
  sr_recip: f32,
}

impl<const NUMGRAINS:usize, const BUFSIZE: usize> Granulator<NUMGRAINS, BUFSIZE> {
  pub fn new<const N:usize, const M: usize>(env_shape: &EnvType<N, M>, samplerate: f32) -> Self {
    // Buffer to hold recorded audio
    let buffer = vec![0.0; BUFSIZE];
    let envelope = Envelope::new(&env_shape, samplerate);

    let grains = std::array::from_fn(|_| {
      Token {
        duration: 0.0,
        buf_position: 0.0,
        env_position: 0.0,
        rate: 1.0,
        active: false
      }
    });


    // let durations = [0.0; NUMGRAINS];
    // let buf_positions = [0.0; NUMGRAINS];
    // let env_positions = [0.0; NUMGRAINS];
    // let rates = [1.0; NUMGRAINS];
    // let active = [false; NUMGRAINS];

    Self {
      buffer,
      buf_size: BUFSIZE as f32,
      env_size: envelope.len() as f32,
      grains,
      envelope,
      rec_pos: 0,
      recording: false,
      // env_positions,
      // buf_positions, 
      next_grain: 0,
      // durations,
      samplerate,
      sr_recip: 1.0 / samplerate,
      // rates,
      // active
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
    if trigger >= 1.0 && !self.grains[self.next_grain].active { 
      // normalize buffer position
      let pos = match (position + jitter).fract() {
        x if x < 0.0 => { (1.0 + x) * self.buf_size },
        x            => { x  * self.buf_size }
      };
      unsafe {
        // WILL NOT GO OUT OF BOUNDS - set parameters for grain
        let g = self.grains.get_unchecked_mut(self.next_grain);
        g.buf_position = pos;
        g.env_position = 0.0;
        g.rate         = rate;
        g.duration     = calc_duration(self.env_size, self.sr_recip, 1.0/duration);
        g.active       = true;
      }
      // set grain to active
      // increment and wait for next trigger
      self.next_grain = (self.next_grain + 1) % NUMGRAINS;
    }


    let mut out = 0.0;
    for g in self.grains.iter_mut() {
      // if the grain has reached the envelopes end, deactivate
      if g.env_position >= self.env_size { g.active = false; continue;}
      // accumulate output of active grains
      if g.active {
        let sig = BufferInterpolation::interpolate(g.buf_position, &self.buffer, BUFSIZE);
        let env = self.envelope.read::<EnvelopeInterpolation>(g.env_position);
        g.buf_position += g.rate;
        g.env_position += g.duration;
        out += sig * env;
      } 
    }
    out
  }

  pub fn record(&mut self, sample: f32) -> Option<f32> {
    if self.rec_pos == self.buf_size as usize { return None; }
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
    self.sr_recip = 1.0 / samplerate;
  }

  #[inline]
  pub fn reset_record(&mut self) {
    self.rec_pos = 0;
  }

  #[inline]
  pub fn set_buffersize(&mut self, size: usize) {
    self.buffer = vec![0.0; size];
    self.buf_size = size as f32;
  }
}
  
#[inline]
fn calc_duration(env_len: f32, samplerate_recip: f32, duration_recip: f32) -> f32{
  env_len * samplerate_recip * duration_recip
}
