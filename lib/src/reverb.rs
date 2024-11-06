use crate::delay::{Delay, DelayTrait, FixedDelay};
use crate::filter::{Filter, Comb};
use crate::interpolation::Interpolation;

pub struct SchroederVerb {
  c1: Comb,
  c2: Comb,
  c3: Comb,
  c4: Comb,
  ccoeffs: [f32; 4],
  a1: Comb,
  a2: Comb,
  a3: Comb,
}

impl Verb for SchroederVerb {
  fn new(samplerate: f32) -> Self {
    let mut c1 = Comb::new::<4799>(samplerate, 0.95, 0.0);
    let mut c2 = Comb::new::<4999>(samplerate, 0.95, 0.0);
    let mut c3 = Comb::new::<5399>(samplerate, 0.95, 0.0);
    let mut c4 = Comb::new::<5801>(samplerate, 0.95, 0.0);

    c1.set_damp(0.3);
    c2.set_damp(0.3);
    c3.set_damp(0.3);
    c4.set_damp(0.3);


    let a1 = Comb::new::<1051>(samplerate, 0.7, 0.7);
    let a2 = Comb::new::<337>(samplerate, 0.7, 0.7);
    let a3 = Comb::new::<113>(samplerate, 0.7, 0.7);

    SchroederVerb{
      c1, c2, c3, c4, 
      ccoeffs: [
        0.742, 
        0.733, 
        0.715, 
        0.697
      ],
      a1, a2, a3 
    }            
  }              

  fn process<T:Interpolation>(&mut self, sample: f32) -> f32 {
    let mut out;
    out = self.a1.process(sample);
    out = self.a2.process(out);
    out = self.a3.process(out);

    out += self.c1.process(out) * self.ccoeffs[0];
    out += self.c2.process(out) * self.ccoeffs[1];
    out += self.c3.process(out) * self.ccoeffs[2];
    out += self.c4.process(out) * self.ccoeffs[3];

    out
  }
}

pub struct ChownVerb {
  c1: Comb,
  c2: Comb,
  c3: Comb,
  c4: Comb,
  ccoeffs: [f32; 4],
  a1: Comb,
  a2: Comb,
  a3: Comb,
}

impl Verb for ChownVerb {
  fn new(samplerate: f32) -> Self {
    let mut c1 = Comb::new::<901>(samplerate, 0.95, 0.0);
    let mut c2 = Comb::new::<778>(samplerate, 0.95, 0.0);
    let mut c3 = Comb::new::<1011>(samplerate, 0.95, 0.0);
    let mut c4 = Comb::new::<1123>(samplerate, 0.95, 0.0);

    c1.set_damp(0.3);
    c2.set_damp(0.3);
    c3.set_damp(0.3);
    c4.set_damp(0.3);

    let a1 = Comb::new::<125>(samplerate, 0.7, 0.7);
    let a2 = Comb::new::<42>(samplerate, 0.7, 0.7);
    let a3 = Comb::new::<13>(samplerate, 0.7, 0.7);

    Self {
      c1, c2, c3,c4,
      ccoeffs: [ 0.805, 0.827, 0.783, 0.764 ],
      a1, a2, a3
    }
  }

  fn process<T: Interpolation>(&mut self, sample: f32) -> f32 {
    let mut out;
    out = self.c1.process(sample) * self.ccoeffs[0];
    out += self.c2.process(out) * self.ccoeffs[1];
    out += self.c3.process(out) * self.ccoeffs[2];
    out += self.c4.process(out) * self.ccoeffs[3];
      
    out = self.a1.process(out);
    out = self.a2.process(out);
    out = self.a3.process(out);
    out
  }
}


// pub struct DattVerb {
//   predelay: Delay,
//   bw_delay: FixedDelay,
//   diffusion: [Comb; 4],
//   tank_head: [Comb; 2],
//   exursion: [Delay; 2],
//   // dc_block,
//   tank_tail: [Comb; 2]
// }
//
// impl DattVerb {
//   pub fn new(samplerate: f32) -> Self{
//     let sr = samplerate as usize;
//     Self { 
//       predelay: Delay::new(sr), 
//       bw_delay: FixedDelay::new(),
//       diffusion: [
//         Comb::new::<142>(samplerate, 0.7, 0.7), 
//         Comb::new::<107>(samplerate, 0.7, 0.7), 
//         Comb::new::<379>(samplerate, 0.7, 0.7), 
//         Comb::new::<277>(samplerate, 0.7, 0.7), 
//       ],
//       tank_head: [
//         Comb::new::<672>(samplerate, 0.7, 0.7), 
//         Comb::new::<908>(samplerate, 0.7, 0.7), 
//       ],
//       exursion: [
//         Delay::new(sr),
//         Delay::new(sr)
//       ],
//       tank_tail: [
//         Comb::new::<1800>(samplerate, 0.7, 0.7), 
//         Comb::new::<2656>(samplerate, 0.7, 0.7), 
//       ]
//     }
//   }
//
//   #[inline]
//   fn diffuse(&mut self, sample: &mut f32) -> f32 {
//     for diff in self.diffusion.iter_mut() {
//       *sample = diff.process(*sample);
//
//     }
//     *sample
//   }
//
//   #[inline]
//   fn tank(&mut self, sample: f32) -> f32 {
//     todo!()
//   }
//
//   pub fn process<DelayInterpolation: Interpolation>(&mut self, sample: f32, predelay: f32, bw: f32) -> f32 {
//     let pd = self.predelay.play::<DelayInterpolation>(sample, predelay, 0.4);
//     let mut sample = self.bw_delay.play(pd, bw);
//     sample = self.diffuse(&mut sample);
//     self.tank(sample);
//     0.0
//
//   } 
// }

pub trait Verb {
  fn new(samplerate: f32) -> Self;
  fn process<T: Interpolation>(&mut self, sample: f32) -> f32;
}



#[cfg(test)]
mod tests {
}
