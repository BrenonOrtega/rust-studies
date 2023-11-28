use std::{fs::{self, ReadDir} };

trait Bite {
    fn take_bite(&self) -> i32 {
        0
    }
}

fn main() {
    match fs::read_dir(".") {
        Ok(value) => enumerate_files(value),
        Err(error) => { 
            panic!("{}", error) 
        }
    };
}

fn enumerate_files(vec: ReadDir) -> () {
    println!("{:?}", vec);
}