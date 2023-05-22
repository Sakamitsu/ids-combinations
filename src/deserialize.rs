use std::fs::File;
use std::io::{Read};
use crate::Hanzi;

pub fn deserialize_from_file(file_path: &str) -> Vec<Hanzi> {
    
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("Bruh! Failed to open file.{e}")
    };

    let mut json_string = String::new();

    match file.read_to_string(&mut json_string) {
        Ok(bytes) => bytes,
        Err(e) => panic!("Bruh! Failed to read file.{e}")
    };

    let hanzis: Vec<Hanzi> = match serde_json::from_str(&json_string) {
        Ok(vec) => vec,
        Err(e) => panic!("Bruh! Failed to deserialize JSON.{e}")
    };

    hanzis
}