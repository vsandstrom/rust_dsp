#![cfg_attr(not(feature="std"), no_std)]
// #[cfg(not(feature="std"))]
extern crate alloc;

pub use crate::waveshape::macros::*;

pub mod dsp;
pub mod grains;
pub mod trig;
pub mod wavetable;
pub mod interpolation;
pub mod vector;
pub mod buffer;
#[macro_use]
pub mod waveshape;
pub mod envelope;
pub mod adsr;
pub mod polytable;
pub mod delay;
pub mod filter;
// pub mod reverb;
pub mod midibitfield;
// pub mod karplus;
pub mod noise;
#[allow(non_snake_case)]
pub mod vector2D;
pub mod bindings;
pub mod fold;
