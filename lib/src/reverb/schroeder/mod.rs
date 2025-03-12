use super::Verb;
use super::{Filter, Comb};
use super::Interpolation;

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
    let mut c1 = Comb::new::<4799>(0.95, 0.0);
    let mut c2 = Comb::new::<4999>(0.95, 0.0);
    let mut c3 = Comb::new::<5399>(0.95, 0.0);
    let mut c4 = Comb::new::<5801>(0.95, 0.0);

    c1.set_damp(0.3);
    c2.set_damp(0.3);
    c3.set_damp(0.3);
    c4.set_damp(0.3);


    let a1 = Comb::new::<1051>(0.7, 0.7);
    let a2 = Comb::new::<337>(0.7, 0.7);
    let a3 = Comb::new::<113>(0.7, 0.7);

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
}

impl Verb for SchroederVerb {
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


  fn set_damp(&mut self, damp: f32) {
    let damp = if damp < 0.0 { 0.0 } else { damp };
    self.l.iter_mut().for_each(|op| op.set_coeff(damp));
  }
}

pub struct ColorlessVerb<const N: usize = 3, const M: usize = 5> {
  delayline: [DelayLine; N], 
  dl_coeff: [f32; N],
  allpass: [LPComb; M],
  prev: f32,
  g: f32,
}

impl ColorlessVerb {
  pub fn new(samplerate: usize) -> Self {
    let mut allpass = [
      LPComb::new::<513>(0.7, 0.7),
      LPComb::new::<899>(0.7, 0.7),
      LPComb::new::<228>(0.7, 0.7),
      LPComb::new::<1197>(0.7, 0.7),
      LPComb::new::<487>(0.7, 0.7)
    ];

    allpass.iter_mut().for_each(|ap| ap.set_damp(0.5));
    let n = next_pow2((samplerate as f32 * 0.005) as usize);
    Self {
      delayline: [
        DelayLine::new( 87, 128).unwrap(),
        DelayLine::new( 59, 128).unwrap(),
        DelayLine::new( 33, 128).unwrap(),
      ],
      dl_coeff: [0.707, 0.625, 0.404],
      allpass,
      prev: 0.0,
      // do NOT set `g` higher that 0.83
      g: 0.7,
    }
  }
}

impl Verb for ColorlessVerb {
  fn process<T: Interpolation>(&mut self, sample: f32) -> f32 {
    let mut sig = self.delayline[0].read_and_write(sample) + (self.prev * self.g) ;
    sig = self.delayline[1].read_and_write(sig * self.dl_coeff[0]);
    sig = self.delayline[2].read_and_write(sig * self.dl_coeff[1]) * self.dl_coeff[2];
    self.prev = self.allpass[0].process(sig);
    self.prev += self.allpass[1].process(sig);
    self.prev += self.allpass[2].process(sig);
    self.prev += self.allpass[3].process(sig);
    self.prev += self.allpass[4].process(sig);
    self.prev *= 0.2;
    self.prev * (1.0 - (self.g * self.g)) + sample * self.g
  }
}




