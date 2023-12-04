use std::env::args;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_name = args().nth(1).ok_or("Please provide a filename")?;
    
    let mut total = 0;

    let lines = fs::read_to_string(file_name)?;
    for line in lines.lines() {
        let num: i32 = line.trim().parse()?;
        total = total + num;
    }
    println!("{total}");   
    Ok(()) 
}
