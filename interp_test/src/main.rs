mod plotter;
use plotter::plot_buffer;
use rust_dsp::waveshape::traits::Waveshape;

fn main() {
  let buffer = [0.0, 4.0, 4.2, 2.0, 1.0];
  plot_buffer(&buffer);

  let buffer = [0.0, -2.0, 4.2, 1.0, -0.8];
  plot_buffer(&buffer);
  

  let buffer = [0.0; 16].complex_sine(
    [1.0, 0.3, 0.2],
    [0.0, 0.3, 1.0]);
  plot_buffer(&buffer);
}

