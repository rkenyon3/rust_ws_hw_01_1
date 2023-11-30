use std::env;
use std::fs;
fn main() {
    let file_name = env::args().skip(1).next().expect("Usage: cargo run -- inputfilename\n");
    
    let mut total = 0.0;

    for line in fs::read_to_string(file_name).unwrap().lines() {
        let num: f64 = line.trim().parse().unwrap();
        total = total + num;
    }
    println!("{total}");    
}
