use rodio::{source::Buffered, Decoder, OutputStream, Sink, Source};
use std::convert::AsRef;
use std::io;
use std::io::BufReader;
use std::path::Path;
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

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    for _i in 0..count_words(&read_stdin()) {
        let file = BufReader::new(File::open("voice1.wav").unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);
    }
    sink.sleep_until_end();
}
