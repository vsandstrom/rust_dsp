use std::{ 
  sync::mpsc::channel,
  thread,
  time::Duration 
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rust_dsp::{ 
  dsp::signal::map, interpolation::{Floor, Linear}, waveshape::{self, sawtooth, phasor}, wavetable::shared::WaveTable
};

use mattias_osc::Osc;

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
    let mut w = vec![0.0; 513];
    let mut e = vec![0.0; 513];
    w[512]=0.0;
    e[512]=0.0;
    waveshape::phasor(&mut w[0..512]);
    waveshape::hanning(&mut e[0..512]);

    let mut lfo: [WaveTable; 5] = std::array::from_fn(|_| WaveTable::new());
    lfo.iter_mut().for_each(|w| w.set_samplerate(sr));
    let mut env = WaveTable::new();

    env.set_samplerate(sr);
    let mut osc: [Osc<10>; 4] = [
      Osc::new(config.sample_rate.0, 1.0),
      Osc::new(config.sample_rate.0, 1.0),
      Osc::new(config.sample_rate.0, 1.0),
      Osc::new(config.sample_rate.0, 1.0),
    ];

    let fund = 400.0;
    let freq = [
      [fund * 2.0/5.0, fund * 3.0/2.0, fund * 8.0/5.0, fund * 12.0/5.0],
      [fund * 1.0/2.0, fund * 6.0/2.0, fund * 10.0/9.0, fund * 6.0/5.0],
      [fund * 1.0/3.0, fund * 7.0/6.0, fund * 7.0/4.0, fund * 12.0/8.0],
      [fund * 3.0/8.0, fund * 9.0/6.0, fund * 9.0/4.0, fund * 10.0/8.0],
    ];
  
    let mut c = 0;
    let mut j = 0;


    // Create a channel to send and receive samples
    let (tx, _rx) = channel::<f32>();
    // Callbacks
    let input_callback = move 
      | data: &[f32], _: &cpal::InputCallbackInfo | {
        // Process input data
      let mut output_fell_behind = false;
      for &sample in data {
          // Send input data to the output callback, or do any processing
        if tx.send(sample).is_err() {
          output_fell_behind = true;
        }
      }
      if output_fell_behind { eprintln!("Output fell behind"); }
    };


    let output_callback = move 
      | data: &mut [f32], _: &cpal::OutputCallbackInfo | {
      // Process output data
      for frame in data.chunks_mut(2) {
        let a = map(
          &mut lfo[0].play::<Linear>(&w, 0.2, 0.0), 
          -1.0,
          1.0,
          1.0,
          2.0
        );

        let w1 = map(
          &mut lfo[1].play::<Linear>(&w, 0.2, 1.0),
          -1.0,
          1.0,
          0.0,
          1.0
        );

        let w2 = map(
          &mut lfo[2].play::<Linear>(&w, 0.2, 1.0),
          -1.0,
          1.0,
          1.0,
          0.0
        );
        let w3 = map(
          &mut lfo[3].play::<Linear>(&w, 0.2, 1.0),
          -1.0,
          1.0,
          0.0,
          1.0
        );
        let w4 = map(
          &mut lfo[4].play::<Linear>(&w, 0.2, 1.0),
          -1.0,
          1.0,
          1.0,
          0.0
        );
        let wx = [w1, w2, w3, w4];

        let mut prev = 0.0;
        let out = osc.iter_mut().enumerate().map(|(i, o)| 
          {
            let t = o.process(freq[j][i], 1.0, 1.0);//wx[i] + (prev * 0.12));
            prev = t;
            t
          }
        ).sum::<f32>().tanh() * env.play::<Floor>(&e, 1.0/5.0, 0.0);
        c+=1; 
        if c == config.sample_rate.0 * 5 { j+=1; j %= freq.len(); c = 0;}

        frame.iter_mut().for_each( |sample| { *sample = out })
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
