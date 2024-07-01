use envelope::{BreakPoints, EnvType, Envelope};
use interpolation::interpolation::Interpolation;

pub struct Granulator2<const NUMGRAINS: usize, const BUFSIZE:usize> {
  buffer: Vec<f32>,
  envelope: Envelope,
  samplerate: f32,
  buf_size: f32,
  // grains: [Grain2; NUMGRAINS],
  buf_positions: [f32; NUMGRAINS],
  env_positions: [f32; NUMGRAINS],
  durations: [f32; NUMGRAINS],
  rates: [f32; NUMGRAINS],
  rec_pos: usize,
  next_grain: usize,
}

impl<const NUMGRAINS:usize, const BUFSIZE: usize> Granulator2<NUMGRAINS, BUFSIZE> {
  pub fn new<const N:usize, const M: usize>(env_shape: EnvType<N, M>, samplerate: f32) -> Self {
    // Buffer to hold recorded audio
    let buffer = vec![0.0; BUFSIZE];
    let envelope = Envelope::new(&env_shape, samplerate);
    let durations = [calc_duration(envelope.len(), samplerate, 0.2); NUMGRAINS];
    let buf_positions = [0.0; NUMGRAINS];
    let env_positions = [0.0; NUMGRAINS];
    let rates = [1.0; NUMGRAINS];

    Self {
      buffer,
      buf_size: BUFSIZE as f32,
      // grains,
      envelope,
      rec_pos: 0,
      env_positions,
      buf_positions, 
      next_grain: 0,
      durations,
      samplerate,
      rates,
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
    if trigger >= 1.0 { 
      let mut pos = position + jitter * self.buf_size;
      while pos > self.buf_size { pos -= self.buf_size; }
      while pos < 0.0 { pos += self.buf_size; }
      self.buf_positions[self.next_grain] = pos;
      self.env_positions[self.next_grain] = 0.0;
      self.rates[self.next_grain] = rate;
      self.durations[self.next_grain] = calc_duration(self.buffer.len(), self.samplerate, duration);
      self.next_grain = (self.next_grain + 1) % NUMGRAINS;
    }

    let mut out = 0.0;
    for i in 0..NUMGRAINS {
      if (self.env_positions[i] as usize) < BUFSIZE {
        let sig = BufferInterpolation::interpolate(self.buf_positions[i], &self.buffer, BUFSIZE);
        let env = self.envelope.read::<EnvelopeInterpolation>(self.env_positions[i]);
        self.buf_positions[i] += self.rates[i];
        self.env_positions[i] += self.durations[i];
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

  #[inline]
  pub fn reset_record(&mut self) {
    self.rec_pos = 0;
  }
}
  
#[inline]
fn calc_duration(env_len: usize, samplerate: f32, duration: f32) -> f32{
  env_len as f32 / ((samplerate) * duration)
}
