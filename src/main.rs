use rand::Rng;
use rodio::{source::Buffered, Decoder, OutputStream, Sink, Source};
use std::convert::AsRef;
use std::io::BufReader;
use std::path::Path;
use std::{fs, io};
use std::{fs::File, io::Write};

// Count words in a string - very rough implementation
fn count_words(sentence: &str) -> i32 {
    let mut result = 0;
    let mut last_was_space = false;
    for c in sentence.chars() {
        if c.is_whitespace() {
            if last_was_space == false {
                result += 1;
            }
            last_was_space = true;
        } else {
            result += 0;
            last_was_space = false;
        }
    }
    return result;
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    return buffer;
}

fn random_file_from_dir(dir: &str, range: i32) -> String {
    let mut prefix: String = "voice".to_owned();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..range) + 1;
    let suffix = ".wav";
    let filename = format!("{dir}/{prefix}{index}{suffix}");
    println!("random audio file: {}", filename);
    return filename;
}

fn dir_file_count(dir: &str) -> i32 {
    let mut count = 0;
    println!("audio directory: {}", dir);
    for file in fs::read_dir(dir) {
        count += 1;
    }
    println!("{}", count);
    return count;
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let audio_folder = "/Users/kline/projects/summer/georgeos/voice/src/audio";
    for _i in 0..count_words(&read_stdin()) {
        let rnd_file = random_file_from_dir(audio_folder, 18);
        let file = BufReader::new(File::open(rnd_file).unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);
    }
    sink.sleep_until_end();
}
