use std::{thread, time, io, fs};
use std::fs::File;
use std::io::{Error, Write};

fn main(){
    let ten_millis = time::Duration::from_millis(5000);
    let now = time::Instant::now();
    let string:&str = "Hello, OS!";
    thread::sleep(ten_millis);
    println!("{}", string);
    let file_path = "rCore.txt";
    let mut output = fs::File::create(file_path);
    write!(output.expect("REASON"), "{}",  string);
}
