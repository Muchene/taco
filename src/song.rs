use karplus::KarplusStrong;
use karplus::get_freq;

// A guitar song is a collection of GuitarSounds and a pointer to to current note being played
pub struct GuitarSong {
    pub notes           : Vec<GuitarSound>,
    pub curr_sound      : usize
}


pub enum Note {
    A(u32),
    B(u32),
    C(u32),
    D(u32),
    E(u32),
    F(u32),
    G(u32),
}


fn get_note_tuple(note : &Note) -> (u32,u32){
    match *note {
        Note::A(octave) => (0,octave),
        Note::B(octave) => (1,octave),
        Note::C(octave) => (2,octave),
        Note::D(octave) => (3,octave),
        Note::E(octave) => (4,octave),
        Note::F(octave) => (5,octave),
        Note::G(octave) => (6,octave),
    }
}

fn note_to_freq(note : &Note) -> f64 {
    let tuple = get_note_tuple(note);
    get_freq(tuple.0 as i32, tuple.1 as i32)
}

// A GuitarSound is a record representing which strings to be played, which fret on those strings, and
// the duration to play the sound
pub struct GuitarSound {
    duration: f32, //duration of the sound in seconds
    karps : Vec<KarplusStrong>
}

impl GuitarSound {
    pub fn new(notes : &Vec<(Note,f32)>, sample_rate : u32)->GuitarSound {
        let mut karps = Vec::new();
        let mut dur = 0.0;
        for note in notes {
            let freq = note_to_freq(&note.0);
            let karp = KarplusStrong::new(freq, sample_rate, 0.996,note.1);
            if note.1 > dur {
                dur = note.1;
            }
            karps.push(karp);
        }

        GuitarSound {
            duration : dur,
            karps : karps,
        }
    }
}



impl Iterator for GuitarSong {
    type Item = f64;

    fn next(&mut self)-> Option<f64> {
        if self.curr_sound >= self.notes.len() {
            None
        }
        else {
            let mut done=0;
            let ref mut sound = self.notes[self.curr_sound];
            let mut sample : f64 = 0.0;
            for mut gstring in &mut sound.karps {
                match gstring.next() {
                    Some(sound) => sample = sample + sound,
                    None => done += 1,
                };
            }
            if done >= sound.karps.len() {
                self.curr_sound = self.curr_sound+1;
            }
            Some(sample)
        }
    }

}