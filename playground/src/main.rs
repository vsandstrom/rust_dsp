use std::{ 
  sync::mpsc::channel,
  thread,
  time::Duration 
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rust_dsp::{ 
  dsp::signal::map, interpolation::{Floor, Linear}, waveshape::{self, sawtooth, phasor}, wavetable::shared::WaveTable
};

use rust_dsp::{
  dsp::buffer::range, filter::{
    Filter,
    biquad::{BiquadCoeffs, twopole::Biquad, BiquadTrait},
    svf::{SVFilter, SVFCoeffs, SVFTrait},
  }, interpolation::Linear, 
  noise::Noise,
  waveshape::triangle,
  wavetable::owned::Wavetable
};

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
    let mut bq= Biquad::new();
    let mut svf = SVFilter::new();
    

    let mut table_1 = [0.0f32; 512];
    let mut table_2 = [0.0f32; 512];
    triangle(&mut table_1);
    triangle(&mut table_2);

    range(&mut table_1, -1.0, 1.0, (TAU * 100.0) / sr, (TAU * 20000.0) / sr);
    range(&mut table_2, -1.0, 1.0, 0.1, 15.0);

    let mut lfo_freq = Wavetable::new(&table_1, sr);
    let mut lfo_q = Wavetable::new(&table_2, sr);
    lfo_freq.set_samplerate(sr);
    lfo_q.set_samplerate(sr);

    let mut noise = Noise::new(sr);

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
      for out_frame in data.chunks_mut(16) {
        let sig = noise.play(1.0/sr);

        let freq = lfo_freq.play::<Linear>(0.2, 0.0);
        let q = lfo_q.play::<Linear>(0.15, 0.0);
        bq.update(BiquadCoeffs::bpf(freq, q));
        svf.update(SVFCoeffs::bpf(freq, q));
        let sig = sig * 0.1;
        let filter_bq = bq.process(sig) * 0.1;
        let filter_svf = svf.process(sig) * 0.1;

        out_frame[0] = sig; 
        out_frame[1] = filter_bq;
        out_frame[2] = filter_svf;
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
    loop{ thread::sleep(Duration::from_secs(40)); }

    // allow running forever
    #[allow(unreachable_code)]
    Ok(())
}
