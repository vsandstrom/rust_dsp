use super::{
  Vec, vec, Interpolation,
  pan_exp2,
  calc_duration,
  wrap_position,
  GrainTrait
};

#[derive(Clone, Copy)]
pub(super) struct Grain {
  pub (super)buf_position: f32,
  pub (super) env_position: f32,
  pub (super)duration: f32,
  pub (super)pan: (f32, f32),
  pub (super)rate: f32,
  pub (super)active: bool
}

pub struct Granulator {
  out: [f32; 2],
  envelope: Vec<f32>,
  env_size: usize,
  rec_pos: usize,
  pub recording: bool,

  next_grain: usize,
  grains: Vec<Grain>,

  samplerate: u32,
  sr_recip: f32,
}

impl Granulator {
  pub fn new(shape: Vec<f32>, samplerate: u32, num_grains: usize) -> Self {
  // Buffer to hold recorded audio

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
    env_size: shape.len(),
    out: [0.0; 2],
    grains,
    envelope: shape,
    rec_pos: 0,
    recording: false,
    next_grain: 0,
    samplerate,
    sr_recip: 1.0 / samplerate as f32,
  }
}


#[inline]
pub fn play<BufferInterpolation, EnvelopeInterpolation>(&mut self, buffer: &[f32]) -> &[f32; 2]
where BufferInterpolation: Interpolation,
      EnvelopeInterpolation: Interpolation {
  self.out = [0.0;2];
  let buf_size = buffer.len();
  for g in self.grains.iter_mut() {
    // if the grain has reached the envelopes end, deactivate
    if g.env_position >= self.env_size as f32 { g.active = false; continue;}
    // accumulate output of active grains
    if g.active {
      g.buf_position = wrap_position(g.buf_position, buf_size);
      let sig = BufferInterpolation::interpolate(g.buf_position, buffer, buf_size);
      let env_a = self.envelope[g.env_position as usize];
      let env_b = self.envelope[(g.env_position as usize + 1) % self.env_size];
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
    g.buf_position = position + jitter;
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
 
  pub fn reset_record(&mut self) {
    self.rec_pos = 0;
  }
}


impl GrainTrait for Granulator {
  // #[inline]
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

   fn set_samplerate(&mut self, samplerate: u32) {
    self.samplerate = samplerate;
    self.sr_recip = 1.0 / samplerate as f32;
  }


  // #[inline]
  // fn set_buffersize(&mut self, size: usize) {
  //   self.buffer = vec![0.0; size];
  //   self.buf_size = size;
  // }
}
