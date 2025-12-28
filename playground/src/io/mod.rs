use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Device};
use cpal::StreamConfig;
use std::ops::ControlFlow;

/// Wrapper class around the CPAL host and in/out devices.
pub struct IO {
  pub input_device: Device,
  pub output_device: Device,
  pub config: StreamConfig,
  pub samplerate: u32,
  pub channels: u16,
  pub buffer_size: BufferSize
}

const DEFAULT_OUTPUT: &str = "BlackHole 2ch";

impl IO {
  /// Sets output to BlackHole 2ch
  /// Useful for when output could be harmful.
  pub fn new_default() -> Result<IO, &'static str> {
    let host = cpal::default_host();
    let devices = host.input_devices().expect("could not find input devices").collect::<Vec<Device>>();
    let input_device = host.default_input_device().expect("no default input device available");

    let output_device = devices
      .iter()
      .filter_map(|d| 
        d.name()
        .ok()
        .map(|n| (d, n)))
      .find(|(_, name)| name == DEFAULT_OUTPUT)
      .expect("Could not find BlackHole 2ch").0;

    let config: StreamConfig = output_device.default_output_config()
      .expect("could not get supported stream configuration")
      .into();
    
    let samplerate = config.sample_rate.0;
    let channels = config.channels;
    let buffer_size = config.buffer_size;

    Ok(
      IO { 
        input_device: input_device.clone(),
        output_device: output_device.clone(),
        config,
        samplerate,
        channels,
        buffer_size
      }
    )
  }


  /// Sets output and input to the current system settings.
  /// Useful if you do not want to think about it too much.
  pub fn new_current() -> Result<IO, &'static str> {
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("no default input device available");
    let output_device = host.default_output_device().expect("no default output device available");
    let config: cpal::StreamConfig = input_device.default_input_config()
      .expect("could not get supported stream configuration")
      .into();

    let samplerate = config.sample_rate.0;
    let channels = config.channels;
    let buffer_size = config.buffer_size;

    Ok(
      IO { 
        input_device,
        output_device,
        config,
        samplerate,
        channels,
        buffer_size
      }
    )
  }
}
