use rust_dsp::interpolation::*;
use rust_dsp::wavetable::shared::Wavetable;
use simple_plot::plot;


pub fn plot_buffer<const N:usize>(buffer: &[f32; N]) {
  const FREQ: f32 = 48000.0 / 10000.0;
  let mut wt = [Wavetable::default(); 6];
  wt.iter_mut().for_each(|w| w.set_samplerate(48000.0));
  let mut shapes = vec![Vec::new(); 5];
  for _ in 0..20000 {
    shapes[0].push(wt[1].play::<Hermite>(buffer, FREQ, 0.0));
    shapes[1].push(wt[2].play::<Cosine>  (buffer, FREQ, 0.0));
    shapes[2].push(wt[3].play::<Cubic>   (buffer, FREQ, 0.0));
    shapes[3].push(wt[4].play::<Linear>  (buffer, FREQ, 0.0));
    shapes[4].push(wt[5].play::<Floor>   (buffer, FREQ, 0.0));
  }

  plot!(&format!("hermite: {:?}", buffer), shapes[0].clone());
  plot!(&format!("cosine: {:?}",   buffer), shapes[1].clone());
  plot!(&format!("cubic: {:?}",    buffer), shapes[2].clone());
  plot!(&format!("linear: {:?}",   buffer), shapes[3].clone());
  plot!(&format!("floor: {:?}",    buffer), shapes[4].clone());

}
  
pub fn play_linear<const LENGTH: usize>(tables: &[[f32; LENGTH]], frequency: f32, position: f32, phase: f32, samplerate; f32) -> f32 {
  if frequency > samplerate * 0.5 {return 0.0}
  let len = LENGTH as f32;
  let width = tables.len();

  let position = if position >= 1.0 {0.99999999999999} else {position};
  let position = position * (width as f32 - 1.0);
  let table1 = position.floor() as usize % width;
  let table2 = (table1 + 1) % width;

  let y = position.fract();
  let x = self.table_pos.fract();
  let n = self.table_pos.floor() as usize;
  let m = n + 1;
  let a = tables[table1][n];
  let b = tables[table1][m];
  let c = tables[table2][n];
  let d = tables[table2][m];
  let diff1 = b - a;
  let diff2 = x*(diff1 - d + c);
  let sig = a + x * diff1 + y * (c - a * diff2);
  self.table_pos += (len * self.sr_recip * frequency) + (phase * len);
  while self.table_pos as usize > LENGTH { self.table_pos -= len; }
  while self.table_pos < 0.0 { self.table_pos += len; }
  sig
}
