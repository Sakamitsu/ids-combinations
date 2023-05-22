use std::{fs::File, io::BufWriter};
use crate::Hanzi;

pub fn serialize_and_write_to_file(hanzis: Vec<Hanzi>) {
    
    let file = match File::create("src/files/output.json") {
        Ok(f) => f,
        Err(e) => panic!("Bruh! Cannot create a file.{e}")
    };

    let writer = BufWriter::new(file);
    
    match serde_json::to_writer_pretty(writer, &hanzis) {
        Ok(w) => w,
        Err(e) => panic!("Bruh! Cannot write to the file.{e}")
    };

}