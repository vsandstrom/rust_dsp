use crate::filter::{
  biquad::{
    eightpole::Biquad8, 
    fourpole::Biquad4, 
    twopole::Biquad, 
    BiquadSettings, 
    BiquadTrait
  }, 
  Bpf, 
  Filter,
  HighShelf,
  Hpf,
  LowShelf,
  Lpf,
  Notch, 
  Peq
};
use core::{borrow::BorrowMut, ffi::c_void};
  
#[repr(C)]
pub struct BiquadOpaque{ 
  kind: BiquadType, 
  ptr: *mut c_void 
}

#[repr(C)]
pub enum BiquadType {
  Lpf, 
  Bpf, 
  Hpf, 
  Notch,
  Peq, 
  LowShelf,
  HighShelf
}

#[no_mangle]
/// Constructor
pub extern "C" fn biquad_new(filter_type: BiquadType, settings: BiquadSettings) -> *mut BiquadOpaque {
  match filter_type {
    BiquadType::Lpf => {
      let bq = Box::new(Biquad::<Lpf>::new(settings));
      BiquadOpaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    },
    BiquadType::Bpf => {
      let bq = Box::new(Biquad::<Bpf>::new(settings));
      BiquadOpaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    },
    BiquadType::Hpf => {
      let bq = Box::new(Biquad::<Hpf>::new(settings));
      BiquadOpaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    },
    BiquadType::Notch => {
      let bq = Box::new(Biquad::<Notch>::new(settings));
      BiquadOpaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    }
    BiquadType::Peq => {
      let bq = Box::new(Biquad::<Peq>::new(settings));
      BiquadOpaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    }
    BiquadType::LowShelf => {
      let bq = Box::new(Biquad::<LowShelf>::new(settings));
      BiquadOpaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    }
    BiquadType::HighShelf => {
      let bq = Box::new(Biquad::<HighShelf>::new(settings));
      BiquadOpaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    }
  }
}

#[no_mangle]
/// Destructor
pub unsafe extern "C" fn biquad_delete(biquad: *mut BiquadOpaque) {
  if !biquad.is_null() {
    let kind = biquad.read().kind;
    match kind {
      BiquadType::Lpf       => drop(Box::from_raw(biquad as *mut Biquad<Lpf>)),
      BiquadType::Bpf       => drop(Box::from_raw(biquad as *mut Biquad<Bpf>)),
      BiquadType::Hpf       => drop(Box::from_raw(biquad as *mut Biquad<Hpf>)),
      BiquadType::Notch     => drop(Box::from_raw(biquad as *mut Biquad<Notch>)),
      BiquadType::Peq       => drop(Box::from_raw(biquad as *mut Biquad<Peq>)),
      BiquadType::LowShelf  => drop(Box::from_raw(biquad as *mut Biquad<LowShelf>)),
      BiquadType::HighShelf => drop(Box::from_raw(biquad as *mut Biquad<HighShelf>)),
    }
  }
}

#[no_mangle]
pub unsafe extern "C" fn biquad_process(biquad: *mut BiquadOpaque, sample: f32) -> f32 {
  if biquad.is_null() {
    return 0.0;
  }

  let bq = &mut *biquad;
  match bq.kind {
    BiquadType::Lpf       => (*(bq.ptr as *mut Biquad<Lpf>)).process(sample),
    BiquadType::Bpf       => (*(bq.ptr as *mut Biquad<Bpf>)).process(sample),
    BiquadType::Hpf       => (*(bq.ptr as *mut Biquad<Hpf>)).process(sample),
    BiquadType::Notch     => (*(bq.ptr as *mut Biquad<Notch>)).process(sample),
    BiquadType::Peq       => (*(bq.ptr as *mut Biquad<Peq>)).process(sample),
    BiquadType::LowShelf  => (*(bq.ptr as *mut Biquad<LowShelf>)).process(sample),
    BiquadType::HighShelf => (*(bq.ptr as *mut Biquad<HighShelf>)).process(sample),
  }
}

#[no_mangle]
pub unsafe extern "C" fn biquad_update(biquad: *mut BiquadOpaque, settings: BiquadSettings) {
  if biquad.is_null() {
    return;
  }

  let bq = &mut *biquad;

  match bq.kind {
    BiquadType::Lpf => (*(bq.ptr as *mut Biquad<Lpf>)).update(&settings),
    BiquadType::Bpf => (*(bq.ptr as *mut Biquad<Bpf>)).update(&settings),
    BiquadType::Hpf => (*(bq.ptr as *mut Biquad<Hpf>)).update(&settings),
    BiquadType::Notch => (*(bq.ptr as *mut Biquad<Notch>)).update(&settings),
    BiquadType::Peq => (*(bq.ptr as *mut Biquad<Peq>)).update(&settings),
    BiquadType::LowShelf => (*(bq.ptr as *mut Biquad<LowShelf>)).update(&settings),
    BiquadType::HighShelf => (*(bq.ptr as *mut Biquad<HighShelf>)).update(&settings),
  }
}



// Biquad 4 pole


#[repr(C)]
pub struct Biquad4Opaque {
  kind: BiquadType, 
  ptr: *mut c_void 
}

#[no_mangle]
pub extern "C" fn biquad4_new(filter_type: BiquadType, settings: BiquadSettings) -> *mut Biquad4Opaque {
  match filter_type {
    BiquadType::Lpf => {
      let bq = Box::new(Biquad4::<Lpf>::new(settings));
      Biquad4Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    },
    BiquadType::Bpf => {
      let bq = Box::new(Biquad4::<Bpf>::new(settings));
      Biquad4Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    },
    BiquadType::Hpf => {
      let bq = Box::new(Biquad4::<Hpf>::new(settings));
      Biquad4Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    },
    BiquadType::Notch => {
      let bq = Box::new(Biquad4::<Notch>::new(settings));
      Biquad4Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    } 
    BiquadType::Peq => {
      let bq = Box::new(Biquad4::<Peq>::new(settings));
      Biquad4Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    },
    BiquadType::LowShelf => {
      let bq = Box::new(Biquad4::<LowShelf>::new(settings));
      Biquad4Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    },
    BiquadType::HighShelf => {
      let bq = Box::new(Biquad4::<HighShelf>::new(settings));
      Biquad4Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    } 
  }
}

#[no_mangle]
pub unsafe extern "C" fn biquad4_delete(biquad: *mut Biquad4Opaque) {
  if biquad.is_null() { return; }

  let bq = Box::from_raw(biquad);
  match bq.kind {
    BiquadType::Lpf => drop(Box::from_raw(bq.ptr as *mut Biquad4<Lpf>)),
    BiquadType::Bpf => drop(Box::from_raw(bq.ptr as *mut Biquad4<Bpf>)),
    BiquadType::Hpf => drop(Box::from_raw(bq.ptr as *mut Biquad4<Hpf>)),
    BiquadType::Notch => drop(Box::from_raw(bq.ptr as *mut Biquad4<Notch>)),
    BiquadType::Peq => drop(Box::from_raw(bq.ptr as *mut Biquad4<Peq>)),
    BiquadType::LowShelf => drop(Box::from_raw(bq.ptr as *mut Biquad4<LowShelf>)),
    BiquadType::HighShelf => drop(Box::from_raw(bq.ptr as *mut Biquad4<HighShelf>)),
  }
}


#[no_mangle]
pub unsafe extern "C" fn biquad4_process(biquad: *mut Biquad4Opaque, sample: f32) -> f32 {
  if biquad.is_null() { return sample; }

  let bq = &mut *biquad;

  match bq.kind {
    BiquadType::Lpf => (*(bq.ptr as *mut Biquad4<Lpf>)).process(sample),
    BiquadType::Bpf => (*(bq.ptr as *mut Biquad4<Bpf>)).process(sample),
    BiquadType::Hpf => (*(bq.ptr as *mut Biquad4<Hpf>)).process(sample),
    BiquadType::Notch => (*(bq.ptr as *mut Biquad4<Notch>)).process(sample),
    BiquadType::Peq => (*(bq.ptr as *mut Biquad4<Peq>)).process(sample),
    BiquadType::LowShelf => (*(bq.ptr as *mut Biquad4<LowShelf>)).process(sample),
    BiquadType::HighShelf => (*(bq.ptr as *mut Biquad4<HighShelf>)).process(sample),
  }
}

#[no_mangle]
pub unsafe extern "C" fn biquad4_update(biquad: *mut Biquad4Opaque, settings: BiquadSettings) {
  if biquad.is_null() { return; }

  let bq = &mut *biquad;

  match bq.kind {
    BiquadType::Lpf => (*(bq.ptr as *mut Biquad4<Lpf>)).update(&settings),
    BiquadType::Bpf => (*(bq.ptr as *mut Biquad4<Bpf>)).update(&settings),
    BiquadType::Hpf => (*(bq.ptr as *mut Biquad4<Hpf>)).update(&settings),
    BiquadType::Notch => (*(bq.ptr as *mut Biquad4<Notch>)).update(&settings),
    BiquadType::Peq => (*(bq.ptr as *mut Biquad4<Peq>)).update(&settings),
    BiquadType::LowShelf => (*(bq.ptr as *mut Biquad4<LowShelf>)).update(&settings),
    BiquadType::HighShelf => (*(bq.ptr as *mut Biquad4<HighShelf>)).update(&settings)
  }
}


// Biquad 8 pole

#[repr(C)]
pub struct Biquad8Opaque {
  kind: BiquadType, 
  ptr: *mut c_void 
}


#[no_mangle]
/// Constructor
pub extern "C" fn biquad8_new(filter_type: BiquadType, settings: BiquadSettings) -> *mut Biquad8Opaque {
  match filter_type {
    BiquadType::Lpf => {
      let bq = Box::new(Biquad8::<Lpf>::new(settings));
      Biquad8Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    },
    BiquadType::Bpf => {
      let bq = Box::new(Biquad8::<Bpf>::new(settings));
      Biquad8Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    },
    BiquadType::Hpf => {
      let bq = Box::new(Biquad8::<Hpf>::new(settings));
      Biquad8Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    },
    BiquadType::Notch => {
      let bq = Box::new(Biquad8::<Notch>::new(settings));
      Biquad8Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    } 
    BiquadType::Peq => {
      let bq = Box::new(Biquad8::<Peq>::new(settings));
      Biquad8Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    } 
    BiquadType::LowShelf => {
      let bq = Box::new(Biquad8::<LowShelf>::new(settings));
      Biquad8Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    } 
    BiquadType::HighShelf => {
      let bq = Box::new(Biquad8::<HighShelf>::new(settings));
      Biquad8Opaque{kind: filter_type, ptr: Box::into_raw(bq) as *mut c_void}.borrow_mut()
    } 
  }
}

#[no_mangle]
pub unsafe extern "C" fn biquad8_delete(biquad: *mut Biquad8Opaque) {
  if biquad.is_null() { return; }

  let bq = Box::from_raw(biquad);
  match bq.kind {
    BiquadType::Lpf => drop(Box::from_raw(bq.ptr as *mut Biquad8<Lpf>)),
    BiquadType::Bpf => drop(Box::from_raw(bq.ptr as *mut Biquad8<Bpf>)),
    BiquadType::Hpf => drop(Box::from_raw(bq.ptr as *mut Biquad8<Hpf>)),
    BiquadType::Notch => drop(Box::from_raw(bq.ptr as *mut Biquad8<Notch>)),
    BiquadType::Peq => drop(Box::from_raw(bq.ptr as *mut Biquad8<Peq>)),
    BiquadType::LowShelf => drop(Box::from_raw(bq.ptr as *mut Biquad8<LowShelf>)),
    BiquadType::HighShelf => drop(Box::from_raw(bq.ptr as *mut Biquad8<HighShelf>)),
  }
}


#[no_mangle]
pub unsafe extern "C" fn biquad8_process(biquad: *mut Biquad8Opaque, sample: f32) -> f32 {
  if biquad.is_null() { return sample; }

  let bq = &mut *biquad;

  match bq.kind {
    BiquadType::Lpf       => (*(bq.ptr as *mut Biquad8<Lpf>)).process(sample),
    BiquadType::Bpf       => (*(bq.ptr as *mut Biquad8<Bpf>)).process(sample),
    BiquadType::Hpf       => (*(bq.ptr as *mut Biquad8<Hpf>)).process(sample),
    BiquadType::Notch     => (*(bq.ptr as *mut Biquad8<Notch>)).process(sample),
    BiquadType::Peq       => (*(bq.ptr as *mut Biquad8<Peq>)).process(sample),
    BiquadType::LowShelf  => (*(bq.ptr as *mut Biquad8<LowShelf>)).process(sample),
    BiquadType::HighShelf => (*(bq.ptr as *mut Biquad8<HighShelf>)).process(sample),
  }
}

#[no_mangle]
pub unsafe extern "C" fn biquad8_update(biquad: *mut Biquad8Opaque, settings: BiquadSettings) {
  if biquad.is_null() { return; }

  let bq = &mut *biquad;

  match bq.kind {
    BiquadType::Lpf       => (*(bq.ptr as *mut Biquad8<Lpf>)).update(&settings),
    BiquadType::Bpf       => (*(bq.ptr as *mut Biquad8<Bpf>)).update(&settings),
    BiquadType::Hpf       => (*(bq.ptr as *mut Biquad8<Hpf>)).update(&settings),
    BiquadType::Notch     => (*(bq.ptr as *mut Biquad8<Notch>)).update(&settings),
    BiquadType::Peq       => (*(bq.ptr as *mut Biquad8<Peq>)).update(&settings),
    BiquadType::LowShelf  => (*(bq.ptr as *mut Biquad8<LowShelf>)).update(&settings),
    BiquadType::HighShelf => (*(bq.ptr as *mut Biquad8<HighShelf>)).update(&settings)
  }
}
