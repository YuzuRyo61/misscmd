extern crate dirs;
extern crate toml;
#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::{Write, Read};

pub mod config;

pub fn pause(msg: &str) {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    write!(stdout, "{}", msg).unwrap();
    stdout.flush().unwrap();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
