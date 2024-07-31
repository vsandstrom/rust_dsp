use rust_dsp::interpolation::*;
use rust_dsp::wavetable::shared::WaveTable;
use simple_plot::plot;


pub fn plot_buffer<const N:usize>(buffer: &[f32; N]) {
  const FREQ: f32 = 48000.0 / 10000.0;
  let mut wt1 = WaveTable::new();
  let mut wt2 = WaveTable::new();
  let mut wt3 = WaveTable::new();
  let mut wt4 = WaveTable::new();
  let mut wt5 = WaveTable::new();
  wt1.set_samplerate(48000.0);
  wt2.set_samplerate(48000.0);
  wt3.set_samplerate(48000.0);
  wt4.set_samplerate(48000.0);
  wt5.set_samplerate(48000.0);
  let mut shapes = vec![Vec::new(); 5];
  for _ in 0..20000 {
    shapes[0].push(wt1.play::<N, Hermetic>(buffer, FREQ, 0.0));
    shapes[1].push(wt2.play::<N, Cosine>  (buffer, FREQ, 0.0));
    shapes[2].push(wt3.play::<N, Cubic>   (buffer, FREQ, 0.0));
    shapes[3].push(wt4.play::<N, Linear>  (buffer, FREQ, 0.0));
    shapes[4].push(wt5.play::<N, Floor>   (buffer, FREQ, 0.0));
  }

  plot!(&format!("hermetic: {:?}", buffer), shapes[0].clone());
  plot!(&format!("cosine: {:?}",   buffer), shapes[1].clone());
  plot!(&format!("cubic: {:?}",    buffer), shapes[2].clone());
  plot!(&format!("linear: {:?}",   buffer), shapes[3].clone());
  plot!(&format!("floor: {:?}",    buffer), shapes[4].clone());

}
