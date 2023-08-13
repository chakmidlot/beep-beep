use std::f64::consts::PI;
use std::time::Duration;
use regex::Regex;
use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Source};
use crate::code::MORSE;

const SAMPLE_RATE: u32 = 8000;


pub struct Signaler {
    sample_rate: u32,
    dot_duration_ms: u32,
    dash_duration_ms: u32,
    space_duration_ms: u32,
    frequency: u32,
}


impl Signaler {
    pub fn new(sample_rate: u32, dot_duration_ms: u32, dash_duration_ms: u32, space_duration_ms: u32, frequency: u32) -> Signaler {
        Signaler {
            sample_rate,
            dot_duration_ms,
            dash_duration_ms,
            space_duration_ms,
            frequency,
        }
    }

    pub fn play(&self, text: &str) {
        let (signal, duration) = self.encode(text);

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        stream_handle.play_raw(signal.convert_samples()).unwrap();
        std::thread::sleep(duration);
    }

    fn encode(&self, text: &str) -> (SamplesBuffer<i16>, Duration) {

        let mut buffer = vec![];
        let mut duration_ms = 0;

        let text = text.to_lowercase();

        let text = Regex::new(r"[^a-z ]").unwrap().replace_all(&text, "");
        let text = Regex::new(r" +").unwrap().replace_all(&text, " ");

        for word in text.split(' ') {
            let (word_buffer, word_duration) = self.encode_word(word);
            buffer.extend(word_buffer);
            duration_ms += word_duration;

            let pause = self.silence(self.space_duration_ms - self.dot_duration_ms);
            buffer.extend(pause);

            duration_ms += 500;
        }

        let signal = SamplesBuffer::new(1, SAMPLE_RATE, buffer);
        (signal, Duration::from_millis(duration_ms as u64))
    }

    fn encode_word(&self, word: &str) -> (Vec<i16>, u32) {
        let mut buffer = vec![];
        let mut duration_ms = 0;

        for letter in word.chars() {
            for beep in MORSE.get(&letter).unwrap().chars() {
                let (signal, length) = match beep {
                    '.' => (self.signal(self.dot_duration_ms), self.dot_duration_ms),
                    '-' => (self.signal(self.dash_duration_ms), self.dash_duration_ms),
                    _ => panic!("Unknown char")
                };

                duration_ms += length;

                buffer.extend(signal);

                let pause = self.silence(self.dot_duration_ms);
                buffer.extend(pause);
                duration_ms += self.dot_duration_ms;
            }

            let pause = self.silence(self.dash_duration_ms - self.dot_duration_ms);
            buffer.extend(pause);
            duration_ms += self.dash_duration_ms - self.dot_duration_ms;
        }

        (buffer, duration_ms)
    }

    fn signal(&self, duration: u32) -> Vec<i16> {
        let mut sample = vec![];

        let n_samples = (duration * self.sample_rate / 1000) as usize;
        let full_cycle = self.sample_rate / self.frequency;

        for i in 0..n_samples {
            let value = (2_f64 * (i as f64 / full_cycle as f64) * PI).sin();
            sample.push((value * i16::MAX as f64) as i16 / 2);
        }

        sample
    }

    fn silence(&self, duration: u32) -> Vec<i16> {
        let mut sample = vec![];

        let n_samples = (duration * self.sample_rate / 1000) as usize;

        for _ in 0..n_samples {
            sample.push(0);
        }

        sample
    }

}