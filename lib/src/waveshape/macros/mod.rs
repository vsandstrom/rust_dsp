/// `sine!($size: literal)` - creates a compile-time fixed array of sine values.
/// `sine!($arr: expr)` - fills a mutable array with sine values in-place.
/// `sine![$default: literal; $size: literal]` - same as first, `$default` is unused (may be removed).
#[macro_export]
macro_rules! sine {
  ($size: literal)  => {{
    let inc: f32 = ::core::f32::consts::TAU / $size as f32;
    let x: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32).sin()
    });
    x
  }};
  ($arr: expr) => {{
    let arr: &mut [f32] = $arr;
    let inc: f32 = ::core::f32::consts::TAU / arr.len() as f32;
    arr.iter_mut().enumerate().for_each(|(i, val)| 
      *val = (i as f32 * inc).sin()
    );
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let inc: f32 = ::core::f32::consts::TAU / $size as f32;
    let x: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32).sin()
    });
    x
  }};
}

#[macro_export]
macro_rules! hanning {
  ($size: literal)  => {{
    let inc: f32 = ::core::f32::consts::PI / $size as f32;
    let x: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32).sin().powf(2.0)
    });
    x
  }};
  ($arr: expr) => {{
    let arr: &mut [f32] = $arr;
    let inc: f32 = ::core::f32::consts::PI / arr.len() as f32;
    arr.iter_mut().enumerate().for_each(|(i, val)| 
      *val = (i as f32 * inc).sin().powf(2.0)
    );
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let inc: f32 = ::core::f32::consts::PI / $size as f32;
    let x: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32).sin().powf(2.0)
    });
    x 
  }}
}

#[macro_export]
macro_rules! square {
  ($size: literal)  => {{
    let half = $size / 2;
    let x: [f32; $size] = std::array::from_fn(|i| {
      if i < half { -1.0f32 } else { 1.0f32 }
    });
    x
  }};
  ($arr: expr) => {{
    let arr: &mut [f32] = $arr;
    let half = $arr.len() / 2;
    $arr.iter_mut().enumerate().for_each(|(i, val)| 
      *val = if i < half { -1.0f32 } else { 1.0f32 }
    );
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let half = $size / 2;
    let arr: [f32; $size] = std::array::from_fn(|i| {
      if i < half { -1.0f32 } else { 1.0f32 }
    });
    arr
  }}
}

#[macro_export]
macro_rules! triangle {
  ($size: literal)  => {{
    let mut inc: f32 = 2.0 / ($size as f32 / 2.0);
    let mut angle = 0.0f32;
    let mut arr = [0.0f32; $size];
    for sample in arr.iter_mut() {
      if angle >= 1.0 || angle <= -1.0 { inc *= -1.0; }
      *sample = angle;
      angle += inc;
    }
    arr
  }};
  ($arr: expr) => {{
    let arr: &mut [f32] = $arr;
    let mut inc: f32 = 2.0 / ($arr.len() as f32 / 2.0);
    let mut angle = 0.0f32;
    for sample in $arr.iter_mut() {
      if angle >= 1.0 || angle <= -1.0 { inc *= -1.0; }
      *sample = angle;
      angle += inc;
    }
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let mut inc: f32 = 2.0 / ($size as f32 / 2.0);
    let mut angle = 0.0f32;
    let mut arr = [0.0f32; $size];
    for sample in arr.iter_mut() {
      if angle >= 1.0 || angle <= -1.0 { inc *= -1.0; }
      *sample = angle;
      angle += inc;
    }
    arr
  }}
}

#[macro_export]
macro_rules! sawtooth {
  ($size: literal)  => {{
    let inc: f32 = 2.0 / ($size as f32 - 1.0);
    let arr: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32) - 1.0f32
    });
    arr
  }};
  ($arr: expr) => {{
    let _: &mut [f32] = $arr;
    let inc: f32 = 2.0 / ($arr.len() as f32 - 1.0);
    $arr.iter_mut().enumerate().for_each(|(i, val)| {
      *val = (inc * i as f32) - 1.0f32
    });
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let inc: f32 = 2.0 / ($size as f32 - 1.0);
    let arr: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32) - 1.0f32
    });
    arr
  }}
}

#[macro_export]
macro_rules! reverse_sawtooth {
  ($size: literal)  => {{
    let inc: f32 = 2.0 / ($size as f32 - 1.0);
    let arr: [f32; $size] = std::array::from_fn(|i| {
      (-inc * i as f32) + 1.0f32
    });
    arr
  }};
  ($arr: expr) => {{
    let _: &mut [f32] = $arr;
    let inc: f32 = 2.0 / ($arr.len() as f32 - 1.0);
    $arr.iter_mut().enumerate().for_each(|(i, val)| {
      *val = (-inc * i as f32) + 1.0f32
    });
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let inc: f32 = 2.0 / ($size as f32 - 1.0);
    let arr: [f32; $size] = std::array::from_fn(|i| {
      (-inc * i as f32) + 1.0f32
    });
    arr
  }}
}

#[macro_export]
macro_rules! phasor {
  ($size: literal)  => {{
    let inc: f32 = 1.0 / ($size as f32 - 1.0);
    let arr: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32)
    });
    arr
  }};
  ($arr: expr) => {{
    let _: &mut [f32] = $arr;
    let inc: f32 = 1.0 / ($arr.len() as f32 - 1.0);
    $arr.iter_mut().enumerate().for_each(|(i, val)| {
      *val = (inc * i as f32)
    });
    $arr
  }};
  [$default: literal; $size: literal] => {{
    let inc: f32 = 1.0 / ($size as f32 - 1.0);
    let arr: [f32; $size] = std::array::from_fn(|i| {
      (inc * i as f32)
    });
    arr
  }}
}
