
extern crate portaudio;

mod song;
mod karplus;
mod player;

use song::GuitarSong;
use song::GuitarSound;
use song::Note;

use portaudio as pa;

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 10;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;


fn main() {

    let player = player::Player::new(SAMPLE_RATE, FRAMES_PER_BUFFER, CHANNELS).unwrap();
    let ks = karplus::KarplusStrong::new(440.00,SAMPLE_RATE as u32,0.996,20.0);
    let play_buf :Vec<f64> = 
        ks.collect::<Vec<f64>>()
        .into_iter()
        .map(|e| {e as f64})
        .collect::<Vec<f64>>();
    
}