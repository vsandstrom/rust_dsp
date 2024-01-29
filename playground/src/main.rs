extern crate interpolation; 
extern crate wavetable;
extern crate reverb;
extern crate cpal;
extern crate anyhow;
use interpolation::interpolation::{Linear, Cubic};
use wavetable::WaveTable;
use reverb::{SchroederVerb, Verb};
use waveshape::{Sine, Triangle, User, complex_sine};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SizedSample, FromSample, Sample};


fn main() -> anyhow::Result<()> {
  let stream = stream_setup_for()?;
  stream.play()?;
  std::thread::sleep(std::time::Duration::from_millis(4000));
  Ok(())
}

pub fn stream_setup_for() -> Result<cpal::Stream, anyhow::Error>
where
{
  let (_host, device, config) = host_device_setup()?;

  match config.sample_format() {
    cpal::SampleFormat::I8 => make_stream::<i8>(&device, &config.into()),
    cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into()),
    cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into()),
    cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into()),
    cpal::SampleFormat::U8 => make_stream::<u8>(&device, &config.into()),
    cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into()),
    cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into()),
    cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into()),
    cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into()),
    cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into()),
    sample_format => Err(anyhow::Error::msg(format!(
        "Unsupported sample format '{sample_format}'"
    ))),
  }
}

pub fn host_device_setup(
) -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
  let host = cpal::default_host();

  let device = host
      .default_output_device()
      .ok_or_else(|| anyhow::Error::msg("Default output device is not available"))?;
  println!("Output device : {}", device.name()?);

  let config = device.default_output_config()?;
  println!("Default output config : {:?}", config);

  Ok((host, device, config))
}

pub fn make_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
  let ffreq: f32 = 80.0;
  let num_channels = config.channels as usize;
  let samplerate = config.sample_rate.0;
  let mut wt = WaveTable::<Cubic, Sine>::new(samplerate as f32, 512);
  let mut md = WaveTable::<Linear, Triangle>::new(samplerate as f32, 512);
  let mut vb = SchroederVerb::new(samplerate as f32);
  wt.frequency = 180.0;
  md.frequency = 75.0;
  let err_fn = |err| eprintln!("Error building output sound stream: {}", err);

  let time_at_start = std::time::Instant::now();
  println!("Time at start: {:?}", time_at_start);

  let stream = device.build_output_stream(
    config,
    move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
      // for 0-1s play sine, 1-2s play square, 2-3s play saw, 3-4s play triangle_wave
      let time_since_start = std::time::Instant::now()
          .duration_since(time_at_start)
          .as_secs_f32();
      if time_since_start < 1.0 {
      } else if time_since_start < 2.0 {
        wt.frequency = ffreq*3.0/2.0;
      } else if time_since_start < 3.0 {
        wt.frequency = ffreq*5.0/2.0;
      } else if time_since_start < 4.0 {
        wt.frequency = ffreq*5.0/3.0;
      } else {
        wt.frequency = ffreq;
      }
      process_frame(output, &mut vb, &mut wt, &mut md, num_channels)
    },
    err_fn,
    None,
  )?;

  Ok(stream)
}

// fn process_frame<SampleType>(
//   output: &mut [SampleType],
//   wavetable: &mut WaveTable<Cubic, User>,
//   modtable: &mut WaveTable<Linear, Triangle>,
//   num_channels: usize,
// ) where
//   SampleType: Sample + FromSample<f32>,
// {
//   for frame in output.chunks_mut(num_channels) {
//     let value: SampleType = SampleType::from_sample(wavetable.play(modtable.play(1.0)));
//
//     // copy the same value to all channels
//     for sample in frame.iter_mut() {
//         *sample = value;
//     }
//   }
// }

fn process_frame<SampleType>(
  output: &mut [SampleType],
  verb: &mut SchroederVerb,
  wavetable: &mut WaveTable<Cubic, Sine>,
  modtable: &mut WaveTable<Linear, Triangle>,
  num_channels: usize,
) where
  SampleType: Sample + FromSample<f32>,
{
  for frame in output.chunks_mut(num_channels) {
    let value: SampleType = SampleType::from_sample(
      verb.process(
        wavetable.play(modtable.play(1.0))
      )
    );

    // copy the same value to all channels
    for sample in frame.iter_mut() {
        *sample = value;
    }
  }
}
