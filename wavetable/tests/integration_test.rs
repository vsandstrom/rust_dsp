#[cfg(test)]
mod tests {
  use wavetable::{owned, shared};
  use std::sync::{Arc, RwLock};

  use interpolation::interpolation::{Floor, Linear};
  use waveshape::traits::Waveshape;

  const SAMPLERATE: f32 = 48000.0;

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
    wt.frequency = 16.0;
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
    wt.frequency = 20.0;
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
  
  #[test] 
  fn triangletest_shared() {
    const SIZE: usize = 16;
    let mut table = [0.0; SIZE];
    let table = Arc::new(RwLock::new(table.triangle().to_vec()));
    let mut wt = shared::WaveTable::new(table, 48000.0);
    let mut shape = vec!();
    // Check if it wraps
    for _ in 0..16 {
      let out = wt.play::<Floor>(SAMPLERATE/8.0, 0.0);
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
    wt.frequency = 16.0;
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
    wt.frequency = 20.0;
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
}
