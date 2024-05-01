extern crate interpolation;
extern crate buffer;
use core::marker::PhantomData;
use interpolation::interpolation::Interpolation;
use buffer::Buffer;

pub struct Envelope<T> {
  buffer: Vec<f32>,
  buf_position: f32,
  speed: f32,
  interpolation: PhantomData<T>,
}

impl<T: Interpolation> Envelope<T> {
  fn generate(points: Vec<f32>, times: Vec<f32>, curves: Vec<f32>, samplerate: f32) -> Vec<f32> {
    let mut times = times.into_iter();
    let mut curves = curves.into_iter();
    let mut buffer = vec!();

    for p in points.windows(2) {
      let q = f32::abs(p[1] - p[0]);

      if let Some(time) = times.next() {
        let num_samples = time * samplerate;
        let m = 1.0 / num_samples;

        for i in 0..(num_samples as usize) {

          if let Some(curve) = curves.next() {
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

  pub fn new(points: Vec<f32>, times: Vec<f32>, curves: Vec<f32>, samplerate: f32) -> Self {
    let buffer = Envelope::<T>::generate(points, times, curves, samplerate);
    Envelope {
      buffer,
      buf_position: 0.0,
      speed: 1.0,
      interpolation: PhantomData
    }
  }

  pub fn len(&self) -> usize {
    self.buffer.len()
  }

  pub fn read(&self, position: f32) -> f32 {
    T::interpolate(position, &self.buffer, self.buffer.len())
  }

  pub fn running(&self) -> bool {
    self.buf_position < self.buffer.len() as f32
  }

  pub fn play(&mut self, trigger: f32) -> f32 {
    let mut out = 0.0;
    if trigger >= 1.0 {
      self.buf_position = 0.0;
      out = self.read(self.buf_position);
      self.buf_position += self.speed;
    } else {
      if self.running() {
        out = self.read(self.buf_position);
        self.buf_position += self.speed;
      }
    }
    out
  }

  pub fn set_speed(&mut self, speed: f32) {
    self.speed = speed;
  }
}

impl<T, const N: usize> From<Buffer<T, N>> for Envelope<T> {
  fn from(buffer: Buffer<T, N>) -> Self {
    Envelope{buffer: buffer.buffer.to_vec(), buf_position: 0.0, speed: 1.0, interpolation: PhantomData}
  }
}

impl<T> From<Vec<f32>> for Envelope<T> {
  fn from(buffer: Vec<f32>) -> Self {
    Envelope{buffer, buf_position: 0.0, speed: 1.0, interpolation: PhantomData}
  }
}

impl<T> From<&[f32]> for Envelope<T> {
  fn from(buffer: &[f32]) -> Self {
    Envelope{buffer: buffer.to_vec(), buf_position: 0.0, speed: 1.0, interpolation: PhantomData}
  }
}

impl<T> Clone for Envelope<T> {
  fn clone(&self) -> Self {
    Envelope { buffer: self.buffer.clone(), buf_position: self.buf_position, speed: self.speed, interpolation: PhantomData }
  }

}

#[cfg(test)]
mod tests {

}
