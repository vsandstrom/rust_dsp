extern crate interpolation;
extern crate buffer;
use interpolation::interpolation::Interpolation;
use buffer::Buffer;

pub struct BreakPoints<const N: usize, const M: usize> {
  pub values: [f32; N],
  pub durations: [f32; M],
  pub curves: Option<[f32; M]>
}

pub struct Envelope {
  buffer: Vec<f32>,
  buf_position: f32,
  speed: f32,
}

impl Envelope {
  fn generate<const N: usize, const M: usize>(breakpoints: BreakPoints<N, M>, samplerate: f32) -> Vec<f32> {
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


  // fn generate(points: Vec<f32>, durations: Vec<f32>, curves: Vec<f32>, samplerate: f32) -> Vec<f32> {
  //   let mut durations = durations.into_iter();
  //   let mut curves = curves.into_iter();
  //   let mut buffer = vec!();
  //
  //   for p in points.windows(2) {
  //     let q = f32::abs(p[1] - p[0]);
  //
  //     if let Some(time) = durations.next() {
  //       let num_samples = time * samplerate;
  //       let m = 1.0 / num_samples;
  //
  //       for i in 0..(num_samples as usize) {
  //
  //         if let Some(curve) = curves.next() {
  //           let slope = q * f32::powf(m * i as f32, curve);
  //
  //           if p[0] > p[1] {
  //             buffer.push(p[0] - slope);
  //
  //           } else {
  //             buffer.push(p[0] + slope);
  //
  //           }
  //         }
  //       }
  //     }
  //   }
  //   buffer
  // }

  pub fn new<const N: usize, const M: usize>(breakpoints: BreakPoints<N, M>, samplerate: f32) -> Self {
    let buffer = Envelope::generate(breakpoints, samplerate);
    Envelope {
      buffer,
      buf_position: 0.0,
      speed: 1.0,
    }
  }

  // pub fn new(points: Vec<f32>, durations: Vec<f32>, curves: Vec<f32>, samplerate: f32) -> Self {
  //   let buffer = Envelope::generate(points, durations, curves, samplerate);
  //   Envelope {
  //     buffer,
  //     buf_position: 0.0,
  //     speed: 1.0,
  //   }
  // }


  pub fn len(&self) -> usize {
    self.buffer.len()
  }

  pub fn read<T: Interpolation>(&self, position: f32) -> f32 {
    T::interpolate(position, &self.buffer, self.buffer.len())
  }

  pub fn running(&self) -> bool {
    self.buf_position < self.buffer.len() as f32
  }

  pub fn play<T: Interpolation>(&mut self, trigger: f32) -> f32 {
    let mut out = 0.0;
    if trigger >= 1.0 {
      self.buf_position = 0.0;
      out = self.read::<T>(self.buf_position);
      self.buf_position += self.speed;
    } else {
      if self.running() {
        out = self.read::<T>(self.buf_position);
        self.buf_position += self.speed;
      }
    }
    out
  }

  pub fn set_speed(&mut self, speed: f32) {
    self.speed = speed;
  }
}

impl<const N: usize> From<Buffer<N>> for Envelope {
  fn from(buffer: Buffer<N>) -> Self {
    Envelope{buffer: buffer.buffer.to_vec(), buf_position: 0.0, speed: 1.0}
  }
}

impl From<Vec<f32>> for Envelope {
  fn from(buffer: Vec<f32>) -> Self {
    Envelope{buffer, buf_position: 0.0, speed: 1.0}
  }
}

impl From<&[f32]> for Envelope {
  fn from(buffer: &[f32]) -> Self {
    Envelope{buffer: buffer.to_vec(), buf_position: 0.0, speed: 1.0}
  }
}

impl Clone for Envelope {
  fn clone(&self) -> Self {
    Envelope { buffer: self.buffer.clone(), buf_position: self.buf_position, speed: self.speed }
  }

}

#[cfg(test)]
mod tests {

}
