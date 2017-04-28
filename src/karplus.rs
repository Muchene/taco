
//extern crate hound;
extern crate rand;

use std::f32::consts::PI;
use std::f64;
//use std::i16;
use self::rand::{thread_rng,Rng};

/*
pub fn test_hound() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        let s = String::new();
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
    writer.finalize().unwrap();
}
*/



pub struct KarplusStrong {
    feedback    : f64,
    buffer      :  Vec<f64>,
    frequency   : f64,
    index       : usize,
    sample_rate : u32,
    dur         : f32,
    num_played  : u32
}

pub fn get_freq(note : i32, octave :i32) ->f64 {
    return 440.0 * 2.0f64.powi(((octave - 4) * 12 + note)/ 12) ;
}

impl KarplusStrong {

    pub fn new(frequency : f64, sample_rate : u32, feedback : f64, dur : f32) -> KarplusStrong{
        let mut rng = thread_rng();
        let buf_size : usize = ((sample_rate as f64)/frequency)as usize;
        let new_karplus = KarplusStrong {
            feedback: feedback,
            buffer: rng.gen_iter::<f64>()
                .take(buf_size)
                .collect::<Vec<f64 >>(),
            frequency: frequency,
            index: 0,
            sample_rate : sample_rate,
            dur : dur,
            num_played: 0
        };
        new_karplus
    }

    pub fn reset(&mut self) {
        let mut rng = thread_rng();
        let buf_size : usize = ((self.sample_rate as f64)/self.frequency)as usize;
        self.buffer = rng.gen_iter::<f64>()
                    .take(buf_size)
                    .collect::<Vec<f64 >>();
        self.index = 0;

    }
}

impl Iterator for KarplusStrong {
    type Item = f64;
    fn next(&mut self)-> Option<f64> {
        if self.num_played > 0 &&
            ((self.num_played as f32)/(self.sample_rate as f32)) > self.dur && self.dur != 0.0 {
            None
        }
        else {
            let sample = self.buffer[self.index].clone();
            let next_idx = (self.index + 1) % self.buffer.len();
            self.buffer[self.index] = (self.buffer[self.index] + self.buffer[next_idx]) * 0.5f64 * self.feedback;
            self.index = next_idx;
            self.num_played += 1;
            Some(sample)
        }
    }
}

/*
pub fn test_karplus() {

    let freq = get_freq(0,4);
    let ks = KarplusStrong::new(freq,44100,0.996,5);
    let num_samples = 1*2*44100;
    let spec = hound::WavSpec {
        channels : 2,
        sample_rate : 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int
    };

    let mut writer = hound::WavWriter::create("test.wave",spec).unwrap();

    for sample in ks.take(num_samples).collect::<Vec<f64>>() {
        let amplitude = i16::MAX as f64;
        writer.write_sample((sample*amplitude) as i16).unwrap();
    }

}
*/
