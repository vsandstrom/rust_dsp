use std::{
  f32::consts::TAU, sync::mpsc::channel, thread, time::Duration
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use rust_dsp::{
  filter::biquad::{twopole::Biquad, fourpole::Biquad4, BiquadCoeffs, BiquadTrait}, interpolation::Hermetic, waveshape::sawtooth, wavetable::shared::Wavetable

};

// type Frame = [f32; 2];


fn main() -> anyhow::Result<()> {
    // List all audio devices
    let host = cpal::default_host();

    // List default input and output devices
    let input_device = match host.default_input_device() {
      Some(device) => {
        // println!("Default input: {}", device.name().unwrap());
        device
      },
      None => panic!("no default input device available")
    };

    let output_device = match host.default_output_device() {
      Some(device) => {
        // println!("Default output: {}", device.name().unwrap()); 
        device
      },
      None => panic!("no default output device available")
    };

    // Use default config from input device
    let config: cpal::StreamConfig = input_device.default_input_config()?.into();
    let sr = config.sample_rate.0 as f32;

    // SETUP YOUR AUDIO PROCESSING STRUCTS HERE !!!! <-------------------------
    let mut bq= Biquad4::new();
    bq.calc_bpf((TAU * 1000.0) / sr, 5.0);

    let mut wt = Wavetable::new();
    wt.set_samplerate(sr);

    let mut table = [0.0f32; 512];
    sawtooth(&mut table);

    // Create a channel to send and receive samples
    let (tx, rx) = channel::<Vec<f32>>();
    // Callbacks
    let input_callback = move 
      | data: &[f32], _: &cpal::InputCallbackInfo | {
        // Process input data
      // tx.send(data.to_vec());
    };


    let output_callback = move 
      | data: &mut [f32], _: &cpal::OutputCallbackInfo | {
      // Process output data
      for out_frame in data.chunks_mut(2) {
        let mut out = 0.0;
        let sig = wt.play::<Hermetic>(&table, 200.0, 0.0);
        out = bq.process(sig);
        out_frame[0] = out;
        out_frame[1] = sig;
      };
    };

    let err_callback = |err: cpal::StreamError| {
        eprintln!("{}", err);
    };

    let input_stream = input_device.build_input_stream(
        &config, 
        input_callback,
        err_callback,
        None
    )?;

    let output_stream = output_device.build_output_stream(
        &config,
        output_callback,
        err_callback,
        None
    )?;

    input_stream.play().expect("FAILED INPUT STREAM");
    output_stream.play().expect("FAILED OUTPUT STREAM");


    loop{
      thread::sleep(Duration::from_secs(40));
    }

    // allow running forever
    #[allow(unreachable_code)]
    Ok(())
}
