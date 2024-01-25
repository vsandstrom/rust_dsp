extern crate envelope;
use envelope::Envelope;
use interpolation::interpolation::Interpolation;

struct Grain<T> {

}

struct Granulator<T> {
  buffer: Vec<f32>,
  envelope: Envelope,
  grains: Vec<Grain<T>>,
  samplerate: f32,
  interpolation: T,
  position: f32,
  playback_rate: f32,
  max_grains: u32,
  num_grains: u32,
  grain_size: f32,
  jitter: f32
}

impl<T: Interpolation> Granulator<T> {
  pub fn new() -> Self {


  }

}

impl<T: Interpolation> Grain<T> {

}

#[cfg(test)]
mod tests {
    use super::*;
}
