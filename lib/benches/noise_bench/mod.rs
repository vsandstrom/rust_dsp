use criterion::Criterion;
use rust_dsp::noise::Prng;
use rand::{SeedableRng, RngCore, rngs::SmallRng};
use rand::Rng;

use rust_dsp::noise::pink::Noise as PinkNoiseRidgeRat;
use rust_dsp::noise::pink::pk3::Noise as PinkNoisePK3;
use rust_dsp::noise::pink::pke::Noise as PinkNoisePKE;
use rust_dsp::noise::pink::voss_mccartney::Noise as PinkNoiseVossRand;
use rust_dsp::noise::pink::voss_mccartney2::Noise as PinkNoiseVossSC;
use rust_dsp::noise::pink::discord::Noise as PinkNoiseVossDiscord;

pub fn criterion_benchmark_pink_noise(c: &mut Criterion) {
  let mut group = c.benchmark_group("pink_noise");
  let seed = 12345678;
  let mut pink1 = PinkNoiseRidgeRat::new(seed as i32);
  let mut pink2 = PinkNoisePK3::new(seed);
  let mut pink3 = PinkNoisePKE::new(seed);
  let mut pink4 = PinkNoiseVossRand::new(seed);
  let mut pink5 = PinkNoiseVossSC::new();
  let mut pink6 = PinkNoiseVossDiscord::new(seed);

  group.bench_function("PinkNoise RidgeRat - BillyDM Firewheel impl", |b| {
    b.iter(|| {
      let _ = pink1.play();
    })
  });

  group.bench_function("PinkNoise - ZeroPole PK3", |b| {
    b.iter(|| {
      let _ = pink2.play();
    })
  });
  
  group.bench_function("PinkNoise - ZeroPole PKE", |b| {
    b.iter(|| {
      let _ = pink3.play();
    })
  });

  group.bench_function("PinkNoise Voss-McCartney - rand::rngs::SmallRng", |b| {
    b.iter(|| {
      let _ = pink4.play();
    })
  });

  group.bench_function("PinkNoise Voss-McCartney - SC PRNG", |b| {
    b.iter(|| {
      let _ = pink5.play();
    })
  });
  
  group.bench_function("PinkNoise Voss-McCartney - Discord PRNG", |b| {
    b.iter(|| {
      let _ = pink6.play();
    })
  });
  
}

pub fn criterion_benchmark_noise(c: &mut Criterion) {
  let mut group = c.benchmark_group("noise");

  let mut prng = Prng::new(12345678);
  let mut rrng = SmallRng::from_rng(&mut rand::rng());
  let mut osrng = SmallRng::from_os_rng();

  // group.bench_function("XorShift unipolar float", |b| {
  //   b.iter(|| {
  //     let x = prng.frand_unipolar();
  //   })
  // });
  // 
  // group.bench_function("rand unipolar float", |b| {
  //   b.iter(|| {
  //     let x = rrng.random_range(0.0f32..1.0);
  //   })
  // });
  // 
  // group.bench_function("rand w os rng unipolar float", |b| {
  //   b.iter(|| {
  //     let x = osrng.random_range(0.0f32..1.0);
  //   })
  // });
  // 
  // group.bench_function("rand rng unipolar u32 w float hack", |b| {
  //   b.iter(|| {
  //     let x = f32::from_bits(0x40000000 | (osrng.next_u32() >> 9));
  //   })
  // });
  // 
  // group.bench_function("rand w os rng unipolar u32 w float hack", |b| {
  //   b.iter(|| {
  //     let x = f32::from_bits(0x3f800000 | (osrng.next_u32() >> 9));
  //   })
  // });
  
  group.bench_function("Supercollider XorShift", |b| {
    b.iter(|| {
      let _ = prng.frand_bipolar();
    })
  });

  group.bench_function("rand SmallRng::from_rng() - random_range(-1.0..1.0)", |b| {
    b.iter(|| {
      let _ = rrng.random_range(-1.0f32..1.0);
    })
  });
  
  group.bench_function("rand SmallRng::from_os_rng() - random_range(-1.0..1.0)", |b| {
    b.iter(|| {
      let _ = osrng.random_range(-1.0f32..1.0);
    })
  });
  
  group.bench_function("rand SmallRng::from_rng() - next_u32() -> float hack", |b| {
    b.iter(|| {
      let _ = f32::from_bits(0x40000000 | (rrng.next_u32() >> 9));
    })
  });
  
  group.bench_function("rand SmallRng::from_os_rng() - next_u32() -> float hack", |b| {
    b.iter(|| {
      let _ = f32::from_bits(0x40000000 | (osrng.next_u32() >> 9));
    })
  });
  
  group.bench_function("discord matthijs rand", |b| {
    let mut state = 12345678;
    b.iter(|| {
      let _ = random_float(&mut state);
    })
  });

  group.bench_function("discord skythedragon rand", |b| {
    let mut state = 12345678;
    b.iter(|| {
      let _ = random_float2(&mut state);
    })
  });
  
  group.bench_function("discord billydm rand", |b| {
    let seed = 12345678;
    let mut rng = XOrShift32Rng::new(seed);
    b.iter(|| {
      let _ = rng.gen_noise_f32();
    })
  });


}

#[inline]
pub fn random_float(state: &mut u32) -> f32 {
    *state = state.wrapping_mul(16807).wrapping_add(1);
    let res = (*state >> 9) | 0x40000000;
    f32::from_bits(res) - 3.0
}

#[inline]
pub fn random_float2(state: &mut u32) -> f32 {
    // here we change the state with a regular integer rng
    // This is the lehmer random number generator: https://en.wikipedia.org/wiki/Lehmer_random_number_generator
    // 16807 here is a magic number. In theory this could be any coprime, but there are some numbers that work better
    // 48271 is also such a number
    *state = state.wrapping_mul(16807).wrapping_add(1);

    // https://experilous.com/1/blog/post/perfect-fast-random-floating-point-numbers
    // and here we get the right part of the integer to generate our float from
    // this abuses IEE 754 floats (and works with doubles too)
    // the first 9 bits of the float are the sign bit, and the exponent
    // numbers from 1 - 2 in this have the same exponent (which the | 0x3F800000 sets)
    // then we can set the mantissa with the state
    // we shift that to the right so the first 9 bits become 0, and don't affect our exponent
    // for doubles (f64) we need to shift by 12, due to the sign and exponent taking up 12 bits, and set these to 0x3FF0000000000000 instead
    let res = (*state >> 9) | 0x3F800000;

    // and here we get the float number
    // we have a range of 1-2, but we want -1 to 1
    (f32::from_bits(res) - 1.5) * 2.0
}

struct XOrShift32Rng {
  fpd: u32,
}

impl Default for XOrShift32Rng {
  fn default() -> XOrShift32Rng {
    XOrShift32Rng { fpd: 17 }
  }
}

impl XOrShift32Rng {
  pub fn new(mut seed: u32) -> XOrShift32Rng {
    // seed cannot be zero
    if seed == 0 { seed = 17; }
    XOrShift32Rng { fpd: seed }
  }
  
  /// Generates a random `u32`
  #[inline]
  pub fn gen_u32(&mut self) -> u32 {
    self.fpd ^= self.fpd << 13;
    self.fpd ^= self.fpd >> 17;
    self.fpd ^= self.fpd << 5;
    self.fpd
  }
  
  /// Generates a random `f32` in the range `[0.0, 1.0]`
  #[inline]
  pub fn gen_f32(&mut self) -> f32 {
    self.gen_u32() as f32 * (1.0 / 4_294_967_295.0)
  }
  
  /// Generates a random `f64` in the range `[0.0, 1.0]`
  #[inline]
  pub fn gen_f64(&mut self) -> f64 {
    f64::from(self.gen_u32()) * (1.0 / 4_294_967_295.0)
  }

  /// Generates a random `f32` in the range `[-1.0, 1.0]`
  #[inline]
  pub fn gen_noise_f32(&mut self) -> f32 {
    self.gen_u32() as f32 * (2.0 / 4_294_967_295.0) - 1.0
  }

  /// Generates a random `f64` in the range `[-1.0, 1.0]`
  #[inline]
  pub fn gen_noise_f64(&mut self) -> f64 {
    f64::from(self.gen_u32()) * (2.0 / 4_294_967_295.0) - 1.0
  }
}
