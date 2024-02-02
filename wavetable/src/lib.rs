extern crate interpolation;
extern crate waveshape;
extern crate dsp;
use core::marker::PhantomData;
use interpolation::interpolation::Interpolation;
use waveshape::*;
use dsp::signal::clamp;

pub struct WaveTable<T> {
  position: f32,
  table: Vec<f32>,
  table_size: usize,
  pub frequency: f32,
  samplerate: f32,
  interpolation: PhantomData<T>
}
  
impl<T: Interpolation> WaveTable<T> {
  pub fn new(table: &Vec<f32>, samplerate: f32) -> WaveTable<T> {
    WaveTable { 
      position: 0.0, 
      table: table.to_vec(),
      table_size: table.len(),
      frequency: 0.0,
      samplerate,
      interpolation: PhantomData,
    } 
  }

  pub fn play(&mut self, phase: f32) -> f32 {
    if self.frequency > (self.samplerate / 2.0) { return 0.0; }
    let norm_ph = clamp((phase+1.0)*0.5, 0.0, 1.0);
    let len = self.table_size;
    self.position += len as f32 / (self.samplerate /  (self.frequency * norm_ph));
    while self.position > self.table_size as f32 {
      self.position -= self.table_size as f32;
    }
    T::interpolate(self.position, &self.table, self.table.len())
  }

  #[allow(unused)]
  fn read(&mut self) -> f32 {
    let out = self.table[self.position as usize];
    self.position = ((self.position as usize + 1) % (self.table.len())) as f32;
    out
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tests::interpolation::interpolation::*;
  use crate::tests::waveshape::{Waveshape};


  #[test] 
  fn triangletest() {
    let table = vec![0.0;16].triangle();
    let mut wt = WaveTable::<Floor>::new(&table, 48000.0);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..17 {
      let out = wt.read();
      shape.push(out);
    }
    assert_eq!(vec![0.0, 0.25, 0.5, 0.75, 1.0, 0.75, 0.5, 0.25, 0.0, -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25, 0.0], shape)
  }
  
  #[test] 
  fn interptest() {
    let table = vec![0.0;16].sine();
    let mut wt = WaveTable::< Linear>::new(&table, 48000.0);
    let mut shape = vec!();
    wt.frequency = 16.0;
    // Check if it wraps
    for _ in 0..17 {
      let out = wt.play(1.0);
      shape.push(out);
    }
    assert_eq!(vec![0.0, 0.25, 0.5, 0.75, 1.0, 0.75, 0.5, 0.25, 0.0, -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25, 0.0], shape)
  }

  #[test]
  fn freq_test() {
    let table = vec![0.0;8].triangle();
    let mut wt = WaveTable::<Floor>::new(&table, 48000.0);
    wt.frequency = 20.0;
    let mut shape = vec!();
    for _ in 0..20 { 
      let out = wt.play(1.0);
      shape.push(out) 
    } 
    println!("{:?}", shape);
  }


  #[test]
  fn linear_test() {
    let table = vec![0.0;4].triangle();
    let mut wt = WaveTable::<Linear>::new(&table, 48000.0);
    wt.frequency = 1.0/92000.0;
    let _ = wt.play(1.0);
    let shape = wt.play(1.0);
    assert_eq!(0.5, shape);

  }
}
