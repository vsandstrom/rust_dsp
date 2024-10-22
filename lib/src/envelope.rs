use crate::interpolation::Interpolation;
use crate::buffer::Buffer;

#[derive(Clone, Copy)]
pub struct BreakPoints<const N: usize, const M: usize> {
  pub values: [f32; N],
  pub durations: [f32; M],
  pub curves: Option<[f32; M]>
}

pub enum EnvType<const N:usize = 0, const M:usize = 0> {
  BreakPoint(BreakPoints<N, M>),
  Vector(Vec<f32>)
}

impl<const N: usize, const M: usize> EnvType<N,M> {
  pub fn breakpoints(self) -> Option<BreakPoints<N,M>> {
    match self {
      EnvType::BreakPoint(brk) => Some(brk),
      _ => None
    }
  }

  pub fn vector(self) -> Option<Vec<f32>> {
    match self {
      EnvType::Vector(vec) => Some(vec),
      _ => None 
    }
  }
}

pub struct Envelope {
  buffer: Vec<f32>,
  env_length: usize,
  buf_position: f32,
  speed: f32,
}

impl Default for BreakPoints<3, 2> {
  fn default() -> Self {
    Self{
      values: [0.0, 1.0, 0.0],
      durations: [0.1, 0.8],
      curves: None
    }
  }
}

impl Envelope {
  fn generate<const N: usize, const M: usize>(breakpoints: &BreakPoints<N, M>, samplerate: f32) -> Vec<f32> {
    let mut durations = breakpoints.durations.into_iter();
    // let mut curves = curves.into_iter();
    let mut buffer = vec!();
    for p in breakpoints.values.windows(2) {
      let q = f32::abs(p[1] - p[0]);

      if let Some(time) = durations.next() {
        let num_samples = time * samplerate;
        let m = 1.0 / num_samples;

        for i in 0..(num_samples as usize) {

          if let Some(curves) = breakpoints.curves {
            let mut curves = curves.into_iter();
            if let Some(curve) = curves.next() {
              let slope = q * f32::powf(m * i as f32, curve);
              if p[0] > p[1] {
                buffer.push(p[0] - slope);
              } else {
                buffer.push(p[0] + slope);
              }
            }
          } else {
            let curve = 1.0;
            let slope = q * f32::powf(m * i as f32, curve);
            if p[0] > p[1] {
              buffer.push(p[0] - slope);
            } else {
              buffer.push(p[0] + slope);
            }
          }
        }
      }
    }
    buffer
  }

  pub fn new<const N: usize, const M: usize>(shape: &EnvType<N, M>, samplerate: f32) -> Self {
    let buffer = match shape {
      EnvType::BreakPoint(brk) =>  {
        Envelope::generate(brk, samplerate)
      },
      EnvType::Vector(vec) => {
        vec.to_owned()
      }
    };
    let env_length = buffer.len();
    Envelope {
      buffer,
      env_length,
      buf_position: 0.0,
      speed: 1.0,
    }
  }

  pub fn len(&self) -> usize {
    self.env_length
  }

  pub fn is_empty(&self) -> bool {
    self.env_length > 0
  }

  pub fn read<T: Interpolation>(&self, position: f32) -> f32 {
    T::interpolate(position, &self.buffer, self.env_length)
  }

  pub fn running(&self) -> bool {
    self.buf_position < self.env_length as f32
  }

  pub fn play<T: Interpolation>(&mut self, trigger: f32) -> f32 {
    if trigger >= 1.0 {
      self.buf_position = 0.0;
    } 
    let mut out = 0.0;
    if (self.buf_position as usize) < self.len() {
      out = self.read::<T>(self.buf_position);
      self.buf_position += self.speed;
    }
    out
  }

  pub fn set_speed(&mut self, speed: f32) {
    self.speed = speed;
  }

  pub fn new_shape<const N: usize, const M: usize>(&mut self, shape: &EnvType<N, M>, samplerate: f32) {
    
    let buffer = match shape {
      EnvType::BreakPoint(brk) => {
        Envelope::generate(brk, samplerate)
      },
      EnvType::Vector(v) => v.to_owned()
    };

    for (i, v) in buffer.iter().enumerate() {
      if let Some(b) = self.buffer.get_mut(i) {
        *b = *v;
      } else {
        self.buffer.push(*v);
      }
    }
    self.env_length = buffer.len();
  }
}

impl Default for Envelope {
  /// Assumes the samplerate is 48kHz, and uses the BreakPoints default values.
  fn default() -> Self {
    let breakpoints = BreakPoints::default();
    let buffer = Envelope::generate(&breakpoints, 48000.0);
    let env_length = buffer.len();
    Self {
      buffer,
      env_length,
      buf_position: 0.0,
      speed: 1.0,
    }
  }
}

impl<const N: usize> From<Buffer<N>> for Envelope {
  fn from(buffer: Buffer<N>) -> Self {
    Envelope{buffer: buffer.buffer.to_vec(), env_length: N, buf_position: 0.0, speed: 1.0}
  }
}

impl From<Vec<f32>> for Envelope {
  fn from(buffer: Vec<f32>) -> Self {
    let inner_buffer = buffer.clone();
    Envelope{buffer: inner_buffer, env_length: buffer.len(), buf_position: 0.0, speed: 1.0}
  }
}

impl From<&[f32]> for Envelope {
  fn from(buffer: &[f32]) -> Self {
    Envelope{buffer: buffer.to_vec(), env_length: buffer.len(), buf_position: 0.0, speed: 1.0}
  }
}

impl Clone for Envelope {
  fn clone(&self) -> Self {
    Envelope { buffer: self.buffer.clone(), env_length: self.env_length, buf_position: self.buf_position, speed: self.speed }
  }

}

pub mod new_env {
  pub struct BreakPoint {
    pub value: f32,
    pub duration: f32,
    pub curve: Option<f32>
  }

  pub enum Reset {
    /// creates discontinuities, snaps to first value in envelope.
    HARD,
    /// handles retriggering without discontinuities, uses previous value
    /// and next segment to calculate a new trajectory.
    SOFT
  }

  pub struct Envelope<const N: usize> {
    breakpoints: [BreakPoint; N],
    counter: f32,
    segment: usize,
    steps: usize,
    inc: f32,
    previous_value: f32,
    samplerate: f32,
    rate: f32,
    reset: Reset
  }

  impl<const N: usize> Envelope<N> {
    /// Create a new Envelope, only if the number of breakpoints are at least 2.
    pub fn new(breakpoints: [BreakPoint; N], samplerate: f32) -> Result<Self, String> {
      // the breakpoint array needs to at least 2, otherwise there are no duration to 
      // travel between
      if N < 2 { return Err("Breakpoints need to be at least 2 items long".to_string()) }
      Ok(Self { 
        breakpoints, 
        counter: 0.0,
        segment: 0,
        steps: 0,
        inc: 0.0,
        previous_value: 0.0, 
        samplerate, 
        rate: 1.0,
        reset: Reset::HARD
      })
    }

    /// Trigger or reset envelope
    #[inline]
    pub fn trigger(&mut self) {
      match self.reset {
        Reset::HARD => {
          // consume first value, duration and curve is disregarded.
          self.previous_value = self.breakpoints.first().unwrap().value;
          self.segment = 0;
        },
        Reset::SOFT => { 
          if self.previous_value == 0.0 {
            self.previous_value = self.breakpoints.first().unwrap().value;
          }
          self.segment = 0; 
        }
      }
    }

    /// generate next sample.
    #[inline]
    pub fn play(&mut self) -> f32 {
      match self.breakpoints.get(self.segment) {
        // step through each segment
        Some(bkp) => {
          if self.segment == 0 || self.counter >= self.steps as f32 {
            self.steps = (bkp.duration * self.samplerate) as usize;
            let angle = self.previous_value + bkp.value;
            self.inc = angle / self.steps as f32;
            // reset couter and step into next segment
            self.segment += 1;
            self.counter = 0.0;
            self.previous_value
          } else {
            // increment value and return counter
            self.previous_value += self.inc;
            self.counter += self.rate;
            self.previous_value
          }
        },
        // if there are no more segments
        None => { 0.0 } 
      }
    }

    #[inline]
    pub fn set_reset_type(&mut self, reset_type: Reset) {
      self.reset = reset_type;
    }
  

    // fn calc_segment(&self, index: usize) -> (usize, f32) {
    //   let steps = (bkp.duration * self.samplerate) as u32;
    //   let angle = self.previous_value + bkp.value;
    //   let inc = angle / steps as f32;
    // }

  }
}

#[cfg(test)]
mod tests {

}
