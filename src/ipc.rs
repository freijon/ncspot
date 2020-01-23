use std::io::{BufReader, BufRead};
use std::os::unix::net::{UnixStream};
use std::thread;

pub fn handle_client(stream: UnixStream) {
    thread::spawn(|| connected_client(stream));
}

fn connected_client(stream: UnixStream) {
    let mut reader: BufReader<UnixStream> = BufReader::new(stream);
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    panic!(input);
}