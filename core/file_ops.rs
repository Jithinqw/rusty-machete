use std::fs::{File, Metadata};
use std::fs;
use std::io::prelude::*;
use std::io::Error;
use std::io::BufReader;
use std::fmt;

fn main() {
    open_file();
    open_write_file();
    read_file();
    read_file_efficient();
    find_file_meta_data();
    remove_file();
}

fn open_file() -> Result<(), Error> {
    println!("Opening a file");
    let mut file = File::create("rusty machete.txt")?;
    Ok(())
}

fn open_write_file() ->Result<(), Error> {
    println!("Opening and writting into a file");
    let mut file_ops = File::create("rusty.txt")?;
    file_ops.write_all(b"Hey I am rusty, Good to see you")?;
    Ok(())
}

fn read_file() -> Result<(), Error> {
    println!("Reading a file!");
    let mut read_file = File::open("rusty.txt")?;
    let mut contents = String::new();
    read_file.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hey I am rusty, Good to see you");
    Ok(())
}

fn read_file_efficient() -> Result<(), Error> {
    println!("Efficiently reading a file");
    let read_file = File::open("rusty.txt")?;
    let mut buf_reader = BufReader::new(read_file);
    let mut buf_rader_contents = String::new();
    buf_reader.read_to_string(&mut buf_rader_contents)?;
    println!("{:?}", buf_rader_contents);
    Ok(())
}

fn find_file_meta_data() -> Result<(), Error> {
    let mut demo = fs::metadata("rusty.txt")?;
    println!("{:?}", demo);
    Ok(())
}

fn remove_file() -> Result<(), Error> {
    println!("Removing file from system");
    fs::remove_file("rusty.txt")?;
    fs::remove_file("rusty machete.txt")?;
    Ok(())
}