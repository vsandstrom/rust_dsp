use crate::interpolation::Interpolation;
use core::{f32::consts::SQRT_2, ops::{Add, AddAssign}};


#[derive(Clone, Copy)]
pub struct Coord {
  x: f32,
  y: f32
}

impl Add for Coord {
  type Output = Coord;
  fn add(self, rhs: Self) -> Self::Output {
    Self::Output{
      x: self.x + rhs.x,
      y: self.y + rhs.y
    }
  }
}

impl AddAssign for Coord {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}

impl From<(f32, f32)> for Coord {
  fn from(value: (f32, f32)) -> Self {
    Self { x: value.0, y: value.1 }
  }
}

type Vector = Coord;

pub struct Table2D<const LENGTH: usize> {
  coords: Coord,
  table: [f32; LENGTH]
}

pub struct VectorOscillator2D {
  samplerate: f32,
  sr_recip: f32,
  table_pos: f32,
  coords: Coord,
  direction: Vector
}

impl VectorOscillator2D {
  pub fn new(samplerate: f32, start_position: Coord) -> Self {
    Self {
      samplerate,
      sr_recip: 1.0/samplerate,
      table_pos: 0.0,
      coords: start_position,
      direction: Vector{x: 0.0, y: 0.0} 
    } 
  }

  /// Produce the next sample from the VectorOscillator2D
  ///
  /// - tables: borrowed array of [`Table2D`] structs, each containing a [`Coord`] and a table `&[f32]`
  /// - radius: the cutoff radius from [`VectorOscillator2D`] interpolating between [`Table2D`] structs.
  ///
  /// ```
  /// // ex:
  // // setup
  // const SIZE: usize = 512;
  // let osc = VectorOscillator2D::new(48000.0, (0.0,0.0).into());
  // let tables = [Table2D{(1.0, 3.5).into(), [0.0, 512].sine()}];
  // // constant size of table, &impl Interpolation
  // let sample = osc.play::<SIZE, Linear>(&tables, 100.0, 2.5, 0.0);
  // ```
  //
  pub fn play<const LENGTH: usize, T: Interpolation>(&mut self,
    tables: &[Table2D<LENGTH>],
    frequency: f32, 
    radius: f32,
    phase: f32
  ) -> f32 { 
    if frequency > self.samplerate * 0.5 {return 0.0}
    let len = LENGTH as f32;

    let mut sig = 0.0;

    for t in tables.iter() {
      let distance = self.convert_manhattan(&t.coords);
      if distance <= radius {
        // Simple addition of nearby tables. 
        // Instead of linear ratio between radius and distance, perhaps 
        sig += {
          T::interpolate(self.table_pos, &t.table, LENGTH)
          * (1.0 - (distance / radius))
        }
      }
    }

    // Increment frequency and 2D position
    self.table_pos += (len * self.sr_recip * frequency) + (phase * len);
    while self.table_pos as usize > LENGTH { self.table_pos -= len; }
    while self.table_pos < 0.0 { self.table_pos += len; }
    self.coords += self.direction;

    sig
  }

  /// calculate the hypotenuse from [`Coord`] in [`VectorOscillator2D`] and [`Table2D`]
  #[inline]
  fn convert_manhattan(&self, table_pos: &Coord) -> f32 {
    let x_diff = f32::abs(self.coords.x - table_pos.x);
    let y_diff = f32::abs(self.coords.y - table_pos.y);
    // tiny edge case
    if x_diff - y_diff <= f32::EPSILON { return x_diff * SQRT_2; }
    f32::sqrt(x_diff * x_diff * y_diff * y_diff)
  }

  /// Change direction vector
  pub fn set_vector(&mut self, vector: Vector) {
    self.direction = vector;
  }
}
