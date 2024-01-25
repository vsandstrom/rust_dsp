extern crate interpolation;
use core::marker::PhantomData;
use interpolation::interpolation::{Interpolation, Floor, Linear, Cubic};

pub struct Envelope<T> {
  buffer: Vec<f32>,
  interpolator: PhantomData<T>,
}

impl<T: Interpolation> Envelope<T> {
  pub fn len(&self) -> usize {
    self.buffer.len()
  }

  pub fn read(&self, position: f32) -> f32 {
    T::interpolate(position, &self.buffer, self.buffer.len())
  }
}

impl Envelope<Linear> { }

impl Envelope<Floor> { }

impl Envelope<Cubic> { }

#[cfg(test)]
mod tests {

}
