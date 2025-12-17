use std::usize;

use simple_plot::plot;
use rust_dsp::{noise::Noise, trig::Trigger};

pub fn plot_noise(test_duration: f32, duration: f32, samplerate: u32) {
  let mut noise = Noise::new(samplerate);
  let mut buffer = vec!();

  let td = (test_duration * samplerate as f32) as usize;
  for _ in 0..td {
    buffer.push(noise.play(duration));
  }

  plot!("linear noise: ", buffer);
}
