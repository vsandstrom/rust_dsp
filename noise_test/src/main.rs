mod plotter;
use plotter::plot_noise;

fn main() {
  plot_noise(8.0, 0.2, 48000.0);
}
