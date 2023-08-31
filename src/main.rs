use rand::Rng;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::{fs, io};
extern crate vader_sentiment;

fn sentence_to_vec(sentence: &str) -> Vec<&str> {
    let words = sentence.split_whitespace().collect();
    return words;
}

fn read_stdin() -> Result<String, io::Error> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
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

fn speak(sentence: &str, emotion: &f64) -> () {
    let words = sentence_to_vec(sentence);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    if emotion >= &0.0 {
        let audio_folder = "/Users/kline/projects/summer/georgeos/voice/src/audio/pos";
        for _i in 0..words.len() {
            let rnd_file = random_file_from_dir(audio_folder, "voice");
            let file = BufReader::new(File::open(rnd_file).unwrap());
            let source = Decoder::new(file).unwrap();
            sink.append(source)
        }
    } else {
        let audio_folder = "/Users/kline/projects/summer/georgeos/voice/src/audio/neg";
        for _i in 0..words.len() {
            let rnd_file = random_file_from_dir(audio_folder, "neg");
            let file = BufReader::new(File::open(rnd_file).unwrap());
            let source = Decoder::new(file).unwrap();
            sink.append(source);
        }
    }
    for i in 0..words.len() {
        println!("{}", words.get(i).unwrap().to_string());
        std::thread::sleep(std::time::Duration::from_secs_f64(0.2));
    }
    std::thread::sleep(std::time::Duration::from_secs_f64(sink.len() as f64));
}

fn main() {
    let sentence = read_stdin().unwrap();
    let analyzer = vader_sentiment::SentimentIntensityAnalyzer::new();
    let emotion = analyzer.polarity_scores(&sentence);
    speak(&sentence, emotion.get("compound").unwrap());
}
