use crate::interpolation::Interpolation;

pub mod owned {
  /// Single refers to the ownership of the underlying wavetable structure.
  /// In the `single` module, the table is always owned by the instance of 
  /// the WaveTable struct. This is useful when designing a VectorSynth with 
  /// the ability to scroll between different wavetables seamlessly.

  use super::*;

  pub struct WaveTable<const N:usize> {
    position: f32,
    table: Vec<f32>,
    size: usize,
    pub samplerate: f32,
    sr_recip: f32
  }

  impl<const N:usize> Clone for WaveTable<N> {
    fn clone(&self) -> Self {
      Self {
        position: self.position,
        table: self.table.clone(),
        size: self.size,
        samplerate: self.samplerate,
        sr_recip: self.sr_recip,
      }
    }
  }
    
  impl<const N: usize> WaveTable<N> {
    pub fn new(table: &[f32; N], samplerate: f32) -> WaveTable<N> {
      WaveTable { 
        position: 0.0, 
        table: table.to_vec(),
        size: table.len(),
        samplerate,
        sr_recip: 1.0 / samplerate
      } 
    }

    /// Update the underlying wavetable array owned by struct
    pub fn update_table(&mut self, value: f32, index: usize) -> std::result::Result<(), &'static str> {
      match self.table.get_mut(index) {
        Some(x) => {*x = value; Ok(())},
        None => Err("table out of bounds")
      }
    }

    pub fn play<T: Interpolation>(&mut self, frequency: f32, phase: f32) -> f32 {
      if frequency > (self.samplerate * 0.5) { return 0.0; }
      let len = self.size as f32;
      self.position += len * self.sr_recip * frequency + (phase * len);
      while self.position > len { self.position -= len; }
      while self.position < 0.0 { self.position += len; }
      T::interpolate(self.position, &self.table, self.table.len())
    }
    
    pub fn set_samplerate(&mut self, samplerate: f32) {
      self.samplerate = samplerate;
      self.sr_recip = 1.0 / samplerate;
    }

    #[allow(unused)]
    fn read(&mut self) -> f32 {
      let out = self.table[self.position as usize];
      self.position = ((self.position as usize + 1) % (self.table.len())) as f32;
      out
    }
  }
}

pub mod shared {
  /// Shared refers to the ownership of the underlying wavetable structure.
  /// In the `shared` module, the table can be shared between the instances
  /// of the WaveTable struct over threads. The changes to the underlying 
  /// wavetable propagates through the shared references.

  use super::Interpolation;
  use std::sync::{Arc, RwLock};

  pub struct WaveTable{
    position: f32,
    table: Arc<RwLock<Vec<f32>>>,
    size: usize,
    pub samplerate: f32,
    sr_recip: f32
  }

  impl WaveTable {
    pub fn new(table: Arc<RwLock<Vec<f32>>>, samplerate: f32) -> WaveTable {
      let size = table.try_read().unwrap().len();
      WaveTable{
        position: 0.0,
        table,
        size,
        samplerate,
        sr_recip: 1.0 / samplerate
      }
    }

    pub fn play<T: Interpolation>(&mut self, frequency: f32, phase: f32) -> f32 {
      if frequency > self.samplerate * 0.5 { return 0.0; }
      let len = self.size as f32;
      self.position += len * self.sr_recip * frequency + (phase * len);
      while self.position > len { self.position -= len; }
      while self.position < 0.0 { self.position += len; }
      if let Ok(table) = &self.table.try_read() {
        T::interpolate(self.position, table.as_ref(), self.size)
      } else {
        0.0
      }
    }
    
    pub fn set_samplerate(&mut self, samplerate: f32) {
      self.samplerate = samplerate;
      self.sr_recip = 1.0 / samplerate;
    }

    #[allow(unused)]
    fn read(&mut self) -> f32 {
      let mut out = 0.0;
      if let Ok(table) = self.table.try_read() {
        out = table[self.position as usize];
      }
      self.position = ((self.position as usize + 1) % (self.size)) as f32;
      out
    }

  }
}

pub mod simple {
    use crate::interpolation::Interpolation;

  pub struct WaveTable {
    position: f32,
    samplerate: f32,
    sr_recip: f32,
  }

  impl WaveTable {
    pub fn new() -> Self {
      Self {
        position: 0.0,
        samplerate: 0.0,
        sr_recip: 0.0,
      }
    }

    #[inline]
    pub fn play<const N: usize, TableInterpolation>(&mut self, table: &[f32; N], frequency: f32, phase: f32) -> f32
      where
          TableInterpolation: Interpolation
    {
      if frequency > self.samplerate * 0.5 { return 0.0; }
      let len = N as f32;
      self.position += len * self.sr_recip * frequency + (phase * len);
      while self.position > len { self.position -= len; }
      while self.position < 0.0 { self.position += len; }
      TableInterpolation::interpolate(self.position, table, N)
    }
      
    pub fn set_samplerate(&mut self, samplerate: f32) {
      self.samplerate = samplerate;
      self.sr_recip = 1.0 / samplerate;
    }
  }

}



#[cfg(test)]
mod tests {
  use super::{owned, shared, simple};
  use std::sync::{Arc, RwLock};

  use crate::interpolation::{Floor, Linear};
  use crate::waveshape::traits::Waveshape;

  const SAMPLERATE: f32 = 48000.0;

  /// OWNED WAVETABLE

  #[test] 
  fn triangletest() {
    const SIZE: usize = 16;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = owned::WaveTable::<SIZE>::new(&table, 48000.0);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<Floor>(SAMPLERATE/ SIZE as f32, 0.0);
      shape.push(out);
    }
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25,  0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25,  0.0
    ], shape)
  }
  
  #[test] 
  fn interptest() {
    const SIZE: usize = 16;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = owned::WaveTable::<SIZE>::new(&table, 48000.0);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<Linear>(SAMPLERATE / SIZE as f32, 1.0);
      shape.push(out);
    }
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25, 0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25, 0.0
    ], shape)
  }

  #[test]
  fn freq_test() {
    const SIZE: usize = 8;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = owned::WaveTable::<8>::new(&table, 48000.0);
    let mut shape = vec!();
    for _ in 0..20 { 
      let out = wt.play::<Floor>(1.0, 1.0);
      shape.push(out) 
    } 
    println!("{:?}", shape);
  }

  #[test]
  fn linear_test() {
    const SIZE: usize = 4;
    let dilude = 2;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = owned::WaveTable::<SIZE>::new(&table, 48000.0);
    let mut shape = vec!();
    for _ in 0..(SIZE * dilude) {
      shape.push(wt.play::<Linear>(SAMPLERATE / (SIZE * dilude) as f32, 1.0));
    }
    println!("{:?}", shape);
    assert_eq!(vec![
       0.5,  1.0,  0.5, 0.0,
      -0.5, -1.0, -0.5, 0.0
    ], shape);
  }


  /// SHARED WAVETABLE
  
  #[test] 
  fn triangletest_shared() {
    const SIZE: usize = 16;
    let mut table = [0.0; SIZE];
    let table = Arc::new(RwLock::new(table.triangle().to_vec()));
    let mut wt = shared::WaveTable::new(table, 48000.0);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<Floor>(SAMPLERATE/SIZE as f32, 0.0);
      shape.push(out);
    }
    assert_eq!(vec![0.25, 0.5, 0.75, 1.0, 0.75, 0.5, 0.25, 0.0, -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25, 0.0], shape)
  }
  
  #[test] 
  fn interptest_shared() {
    const SIZE: usize = 16;
    let mut table = [0.0; SIZE];
    let table = Arc::new(RwLock::new(table.triangle().to_vec()));
    let mut wt = shared::WaveTable::new(table, 48000.0);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<Linear>(SAMPLERATE / SIZE as f32, 1.0);
      shape.push(out);
    }
    assert_eq!(vec![0.25, 0.5, 0.75, 1.0, 0.75, 0.5, 0.25, 0.0, -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25, 0.0], shape)
  }

  #[test]
  fn freq_test_shared() {
    const SIZE: usize = 8;
    let mut table = [0.0; SIZE];
    let table = Arc::new(RwLock::new(table.triangle().to_vec()));
    let mut wt = shared::WaveTable::new(table, 48000.0);
    let mut shape = vec!();
    for _ in 0..20 { 
      let out = wt.play::<Floor>(1.0, 1.0);
      shape.push(out) 
    } 
    println!("{:?}", shape);
  }

  #[test]
  fn linear_test_shared() {
    const SIZE: usize = 4;
    let dilude = 2;
    let mut table = [0.0; SIZE];
    let table = Arc::new(RwLock::new(table.triangle().to_vec()));
    let mut wt = shared::WaveTable::new(table, 48000.0);
    let mut shape = vec!();
    for _ in 0..(SIZE * dilude) {
      shape.push(wt.play::<Linear>(SAMPLERATE / (SIZE * dilude) as f32, 1.0));
    }
    println!("{:?}", shape);
    assert_eq!(vec![0.5, 1.0, 0.5, 0.0, -0.5, -1.0, -0.5, 0.0], shape);
  }
  

  /// SIMPLE WAVETABLE


  #[test] 
  fn triangletest_simple() {
    const SIZE: usize = 16;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = simple::WaveTable::new();
    wt.set_samplerate(SAMPLERATE);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<SIZE, Floor>(table, SAMPLERATE/ SIZE as f32, 0.0);
      shape.push(out);
    }
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25,  0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25,  0.0
    ], shape)
  }
  
  #[test] 
  fn interptest_simple() {
    const SIZE: usize = 16;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = simple::WaveTable::new();
    wt.set_samplerate(SAMPLERATE);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<SIZE, Linear>(table, SAMPLERATE / SIZE as f32, 1.0);
      shape.push(out);
    }
    assert_eq!(vec![
       0.25,  0.5,  0.75,  1.0,  0.75,  0.5,  0.25, 0.0,
      -0.25, -0.5, -0.75, -1.0, -0.75, -0.5, -0.25, 0.0
    ], shape)
  }

  #[test]
  fn freq_test_simple() {
    const SIZE: usize = 8;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = simple::WaveTable::new();
    wt.set_samplerate(SAMPLERATE);
    let mut shape = vec!();
    for _ in 0..20 { 
      let out = wt.play::<SIZE, Floor>(table, 1.0, 1.0);
      shape.push(out) 
    } 
    println!("{:?}", shape);
  }

  #[test]
  fn linear_test_simple() {
    const SIZE: usize = 4;
    let dilude = 2;
    let mut table = [0.0; SIZE];
    let table = table.triangle();
    let mut wt = simple::WaveTable::new();
    wt.set_samplerate(SAMPLERATE);
    let mut shape = vec!();
    for _ in 0..(SIZE * dilude) {
      shape.push(wt.play::<SIZE, Linear>(table, SAMPLERATE / (SIZE * dilude) as f32, 1.0));
    }
    println!("{:?}", shape);
    assert_eq!(vec![
       0.5,  1.0,  0.5, 0.0,
      -0.5, -1.0, -0.5, 0.0
    ], shape);
  }
}
