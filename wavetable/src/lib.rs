extern crate interpolation;
extern crate waveshape;
extern crate dsp;
use core::marker::PhantomData;
use interpolation::interpolation::InterpolationConst;
use waveshape::*;
use dsp::signal::clamp;

pub struct WaveTable<T, const N:usize> {
  position: f32,
  table: [f32; N],
  size: usize,
  frequency: f32,
  samplerate: f32,
  interpolation: PhantomData<T>
}
  
impl<T: InterpolationConst, const N: usize> WaveTable<T, N> {
  pub fn new(table: &[f32; N], samplerate: f32) -> WaveTable<T, N> {
    WaveTable { 
      position: 0.0, 
      table: *table,
      size: table.len(),
      frequency: 0.0,
      samplerate,
      interpolation: PhantomData,
    } 
  }

  pub fn play(&mut self, frequency: f32, phase: f32) -> f32 {
    if frequency > (self.samplerate / 2.0) { return 0.0; }
    self.frequency = frequency;
    let norm_ph = clamp((phase+1.0)*0.5, 0.0, 1.0);
    let len = self.size;
    self.position += len as f32 / (self.samplerate /  (frequency * norm_ph));
    while self.position > self.size as f32 {
      self.position -= self.size as f32;
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
  use crate::tests::waveshape::traits::Waveshape;

  const SAMPLERATE: f32 = 48000.0;

  #[test] 
  fn triangletest() {
    let table = [0.0;16].triangle();
    let mut wt = WaveTable::<Floor, 16>::new(&table, 48000.0);
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
    const TABLE_SIZE: usize = 16;
    let table = [0.0;TABLE_SIZE].triangle();
    let mut wt = WaveTable::< Linear, TABLE_SIZE>::new(&table, 48000.0);
    let mut shape = vec!();
    wt.frequency = 16.0;
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play(SAMPLERATE / TABLE_SIZE as f32, 1.0);
      shape.push(out);
    }
    assert_eq!(vec![0.25, 0.5, 0.75, 1.0, 0.75, 0.5, 0.25, 0.0, -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25, 0.0], shape)
  }

  #[test]
  fn freq_test() {
    let table = [0.0;8].triangle();
    let mut wt = WaveTable::<Floor, 8>::new(&table, 48000.0);
    wt.frequency = 20.0;
    let mut shape = vec!();
    for _ in 0..20 { 
      let out = wt.play(1.0, 1.0);
      shape.push(out) 
    } 
    println!("{:?}", shape);
  }

  #[test]
  fn linear_test() {
    const TABLE_SIZE: usize = 4;
    let dilude = 2;
    let table = [0.0;TABLE_SIZE].triangle();
    let mut wt = WaveTable::<Linear, TABLE_SIZE>::new(&table, 48000.0);
    let mut shape = vec!();
    for _ in 0..(TABLE_SIZE * dilude) {
      shape.push(wt.play(SAMPLERATE / (TABLE_SIZE * dilude) as f32, 1.0));
    }
    println!("{:?}", shape);
    assert_eq!(vec![0.5, 1.0, 0.5, 0.0, -0.5, -1.0, -0.5, 0.0], shape);
  }
  
  // #[test]
  // fn cubic_test() {
  //   let TABLE_SIZE = 9;
  //   let table = vec![0.0;TABLE_SIZE].sine();
  //   let mut wt = WaveTable::<Linear>::new(&table, 48000.0);
  //   let mut shape = vec!();
  //   for _ in 0..16 {
  //     shape.push(wt.play(SAMPLERATE / 16.0, 1.0));
  //   }
  //   println!("{:?}", shape);
  //   assert_eq!(0.707, shape[3]);
  // }
}
