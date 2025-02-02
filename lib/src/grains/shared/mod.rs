use crate::waveshape::traits::Waveshape;
use super::{stereo::Grain, Interpolation, };
use super::{calc_duration, pan_exp2, wrap_position};

pub struct Granulator<const NUMGRAINS: usize> {
  out: [f32; 2],
  envelope: [f32; 512],
  env_size: usize,
  env_mask: usize,
  rec_pos: usize,
  pub recording: bool,

  next_grain: usize,
  grains: [Grain; NUMGRAINS],

  samplerate: f32,
  sr_recip: f32,
}

pub impl<const NUMGRAINS: usize> Granulator<NUMGRAINS> {
  pub fn new(samplerate: f32) -> Self {
  // Buffer to hold recorded audio
  let shape = [0.0; 512].hanning();
  let grains = core::array::from_fn(|_|
    Grain {
      duration: 0.0,
      buf_position: 0.0,
      env_position: 0.0,
      pan: (0.0, 0.0),
      rate: 1.0,
      active: false
    });

  Self {
    env_size: 512,
    env_mask: 511,
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
pub fn play<BufferInterpolation, EnvelopeInterpolation>( &mut self, buffer: &[f32]) -> &[f32; 2]
where BufferInterpolation: Interpolation,
      EnvelopeInterpolation: Interpolation {
  self.out = [0.0;2];
  for g in self.grains.iter_mut() {
    // if the grain has reached the envelopes end, deactivate
    if g.env_position >= self.env_size as f32 { g.active = false; continue;}
    // accumulate output of active grains
    if g.active {
      let sig = BufferInterpolation::interpolate(g.buf_position, buffer, buffer.len());
      let env_a = self.envelope[g.env_position as usize];
      let env_b = self.envelope[(g.env_position as usize + 1) & self.env_mask];
      let x = g.env_position.fract();
      let env = env_a + x * ( env_b - env_a );
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
  buf_size: usize,
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
    g.buf_position = wrap_position(position + jitter, buf_size);
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
