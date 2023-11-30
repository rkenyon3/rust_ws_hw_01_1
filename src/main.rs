use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let file_name = env::args().skip(1).next().expect("Usage: cargo run -- inputfilename\n");
    
    let file = File::open(file_name).expect("Failed to open file");
    let mut buf_reader: BufReader<File> = BufReader::new(file);

    let mut buf = String::new();
    let mut num_bytes = buf_reader.read_line(&mut buf).unwrap();
    
    let mut total = 0;

    while num_bytes != 0 {
        let num_from_line: i32 = buf.trim().parse().unwrap();
        println!("{}", num_from_line);
        total = total + num_from_line;
        //println!("{buf}");
        buf.clear();
        num_bytes = buf_reader.read_line(&mut buf).unwrap();
    }
    println!("Total: {total}");
}
