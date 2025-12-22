use rust_dsp::interpolation::*;
use rust_dsp::wavetable::shared::Wavetable;

use simple_plot::plot;

pub fn plot_buffer<const N:usize>(buffer: &[f32; N], with_values: bool) {
  const FREQ: f32 = 123000.0 / 10000.0;
  let mut wt = [Wavetable::new(); 6];
  wt.iter_mut().for_each(|w| w.set_samplerate(48000));
  let mut shapes = vec![Vec::new(); 5];
  for _ in 0..20000 {
    shapes[0].push(wt[1].play::<Hermite>(buffer, FREQ, 0.0));
    shapes[1].push(wt[2].play::<Cosine> (buffer, FREQ, 0.0));
    shapes[2].push(wt[3].play::<Cubic>  (buffer, FREQ, 0.0));
    shapes[3].push(wt[4].play::<Linear> (buffer, FREQ, 0.0));
    shapes[4].push(wt[5].play::<Floor>  (buffer, FREQ, 0.0));
  }

  if with_values {
    plot!(&format!("hermite: {:?}", buffer), shapes[0].clone());
    plot!(&format!("cosine: {:?}",   buffer), shapes[1].clone());
    plot!(&format!("cubic: {:?}",    buffer), shapes[2].clone());
    plot!(&format!("linear: {:?}",   buffer), shapes[3].clone());
    plot!(&format!("floor: {:?}",    buffer), shapes[4].clone());
  } else {
    plot!("hermite", shapes[0].clone());
    plot!( "cosine", shapes[1].clone());
    plot!("cubic",   shapes[2].clone());
    plot!("linear",  shapes[3].clone());
    plot!("floor",   shapes[4].clone());
  }

}
