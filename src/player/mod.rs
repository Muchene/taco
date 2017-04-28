extern crate portaudio;

use super::karplus;
use portaudio as pa;
use portaudio::stream::OutputSettings;


pub struct Player {
    pa : pa::PortAudio,
    settings: OutputSettings<f32>,
    sample_rate : f64
}

fn instantiate_pa(sample_rate : f64, frames_per_buffer: u32, channels : i32) -> Result<(pa::PortAudio,OutputSettings<f32>), pa::Error> {
    let pa = try!(pa::PortAudio::new());
    let mut settings = try!(pa.default_output_stream_settings(channels, sample_rate, frames_per_buffer));
    settings.flags = pa::stream_flags::CLIP_OFF;

    Ok((pa, settings))

}

impl Player {

    pub fn new(sample_rate : f64, frames_per_buffer: u32, channels : i32) -> Option<Player> {
        match self::instantiate_pa(sample_rate,frames_per_buffer,channels) {
            Ok(pa) => Some(
                Player {
                    pa :pa.0,
                    settings: pa.1,
                    sample_rate: sample_rate
                    }),
            Err(_) => None
        }
    }
}