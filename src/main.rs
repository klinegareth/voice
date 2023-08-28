use rand::Rng;
use rodio::{source::Buffered, Decoder, OutputStream, Sink, Source};
use std::io::BufReader;
use std::{fs, io};
use std::{fs::File, io::Write};
extern crate vader_sentiment;

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

fn random_file_from_dir(dir: &str, pre: &str) -> String {
    let prefix: String = pre.to_owned();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..dir_file_count(dir)) + 1;
    let suffix = ".wav";
    let filename = format!("{dir}/{prefix}{index}{suffix}");
    return filename;
}

fn dir_file_count(dir: &str) -> i32 {
    let mut count = 0;
    for _file in fs::read_dir(dir).unwrap() {
        count += 1;
    }
    return count;
}

fn main() {
    let sentence = read_stdin();
    let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();
    let emotion = analyzer.polarity_scores(&sentence);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    if emotion.get("compound").unwrap() >= &0.0 {
        let audio_folder = "/Users/kline/projects/summer/georgeos/voice/src/audio/pos";
        for _i in 0..count_words(&sentence) {
            let rnd_file = random_file_from_dir(audio_folder, "voice");
            let file = BufReader::new(File::open(rnd_file).unwrap());
            let source = Decoder::new(file).unwrap();
            sink.append(source);
        }
    } else {
        let audio_folder = "/Users/kline/projects/summer/georgeos/voice/src/audio/neg";
        for _i in 0..count_words(&sentence) {
            let rnd_file = random_file_from_dir(audio_folder, "neg");
            let file = BufReader::new(File::open(rnd_file).unwrap());
            let source = Decoder::new(file).unwrap();
            sink.append(source);
        }
    }
    println!("{}", sentence);
    sink.sleep_until_end();
}
