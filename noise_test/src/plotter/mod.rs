use simple_plot::plot;
use rust_dsp::{noise::white::Noise};

pub fn plot_noise(test_duration: f32, _duration: f32, samplerate: u32) {
  let mut noise = Noise::new();
  let mut buffer = vec!();

  let td = (test_duration * samplerate as f32) as usize;
  for _ in 0..td {
    buffer.push(noise.process());
  }

  plot!("linear noise: ", buffer);
}
