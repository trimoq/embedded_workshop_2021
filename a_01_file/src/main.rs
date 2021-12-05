use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*};
use std::process::exit;

use anyhow::Context;

fn main(){
    file_01_expect();
    file_02_result().expect("mhm");
    file_03_anyhow().expect("mhm");

    match file_04_external("test2.txt") {
        Ok(contents) => println!("{}",contents),
        Err(e) => {
            println!("Error: {}",e);
            exit(1)
        }
    };
}

fn file_01_expect(){
    let mut file = File::open("test.txt")
        .expect("Could not open");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read");
    println!("{}",contents);
}

fn file_02_result() -> io::Result<()>{
    let mut file = File::open("test.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}",contents);
    Ok(())
}

fn file_03_anyhow() -> anyhow::Result<()>{
    let mut file = File::open("test.txt")
        .context("context01")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context("context01")?;
    println!("{}",contents);
    Ok(())
}

fn file_04_external(filename: &str) -> anyhow::Result<String>{
    let mut file = File::open(filename)
        .context(format!("Could not open file `{}`",filename))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("Could not read from `{}`",filename))?;
    Ok(contents)
}