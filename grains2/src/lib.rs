use array_init::array_init;
use rand::Rng;
use envelope::{Envelope};
use waveshape::hanning;
use interpolation::interpolation::{InterpolationConst, Interpolation};

pub struct Grain2 {
  position: f32,
  env_position: f32,
  rate: f32,
  duration: f32,
}

pub struct Granulator2<const NUMGRAINS: usize, const BUFSIZE:usize> {
  buffer: [f32; BUFSIZE],
  envelope: Envelope,
  samplerate: f32,
  grains: [Grain2; NUMGRAINS],
  rec_pos: usize,
  jitter: f32,
  active: [bool; NUMGRAINS]
}

impl Default for Grain2 {
  fn default() -> Self {
    Self {
      position: 0.0,
      env_position: 0.0,
      rate: 1.0,
      duration: 0.0533333,
    }
  }
}

impl<const NUMGRAINS:usize, const BUFSIZE: usize> Granulator2<NUMGRAINS, BUFSIZE> {
  pub fn new(samplerate: f32) -> Self {
    let grains: [Grain2; NUMGRAINS] = array_init(|_| 
        Grain2::default()
      );
    // Buffer to hold recorded audio
    let buffer = [0.0; BUFSIZE];
    // Default Envelope shape
    let mut envbug = [0.0; 1024];
    let envshape = hanning(&mut envbug);
    let envelope = Envelope::from(envshape);
    let active = [false; NUMGRAINS];

    Self {
      buffer,
      grains,
      envelope,
      jitter: 0.0,
      rec_pos: 0,
      samplerate,
      active
    }
  }

  fn idle_play<BufInterpolation, EnvInterpolation>(&mut self) -> f32 
  where BufInterpolation: InterpolationConst,
        EnvInterpolation: Interpolation {
    let mut out = 0.0;
    for i in 0..NUMGRAINS {
      let sig = BufInterpolation::interpolate(self.grains[i].position, &self.buffer, BUFSIZE);
      let env = self.envelope.read::<EnvInterpolation>(self.grains[i].env_position);
      self.grains[i].position += self.grains[i].rate;
      self.grains[i].env_position += self.grains[i].duration;

      if self.grains[i].env_position as usize >= self.envelope.len() {
        self.grains[i].position = 0.0;
        self.grains[i].env_position = 0.0;
        self.active[i] = false;
      }
      out += sig * env;
    }
    out
  }

  pub fn play<BufInterpolation, EnvInterpolation>(&mut self, position: f32, duration: f32, rate: f32, trigger:f32) -> f32
  where BufInterpolation: InterpolationConst,
        EnvInterpolation: Interpolation {
    if trigger < 1.0 { return self.idle_play::<BufInterpolation, EnvInterpolation>(); }

    let mut out = 0.0;
    let mut  triggered = false;

    for i in 0..NUMGRAINS {
      match self.active[i] {
        true => {
          let sig = BufInterpolation::interpolate(self.grains[i].position, &self.buffer, BUFSIZE);
          let env = self.envelope.read::<EnvInterpolation>(self.grains[i].env_position);

          self.grains[i].position += self.grains[i].rate;
          self.grains[i].env_position += self.grains[i].duration;
          if self.grains[i].env_position as usize >= self.envelope.len() {
            self.grains[i].position = 0.0;
            self.grains[i].env_position = 0.0;
            self.active[i] = false;
          }

          out += sig * env;
        },

        false => {
          if !triggered {
            let random = rand::thread_rng().gen_range(0.0..=1.0) * self.jitter;
            self.grains[i].position = (f32::fract(position + random)) * BUFSIZE as f32;
            self.grains[i].env_position = 0.0;
            self.grains[i].rate = rate;
            self.grains[i].duration = self.envelope.len() as f32 / ((self.samplerate) * duration);
            self.active[i] = true;

            let sig = BufInterpolation::interpolate(self.grains[i].position, &self.buffer, BUFSIZE);
            let env = self.envelope.read::<EnvInterpolation>(self.grains[i].env_position);

            triggered = true;
            out += sig * env;
          }
        },
      }
    }
    out
  }

  pub fn record(&mut self, sample: f32) -> Option<f32> {
    if self.rec_pos >= BUFSIZE { return None; }
    self.buffer[self.rec_pos] = sample;
    self.rec_pos += 1;
    Some(sample)
  }

  pub fn reset_record(&mut self) {
    self.rec_pos = 0;
  }
}