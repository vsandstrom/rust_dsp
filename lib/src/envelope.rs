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

#[cfg(test)]
mod tests {

}
