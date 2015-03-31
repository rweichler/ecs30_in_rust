// Baseball!
use std::env;
use std::io::BufReader;
use std::error::Error;
use std::fs;
use std::io::BufRead;

fn process_line(record : String) {
    let parts : Vec<_> = record.split(" ").collect();
    let num : i32 = parts[0].parse().unwrap(); 
    let mut hits = 0;
    let mut total = 0;
    println!("Player {}'s record : {}", num, parts[1]);
    for ch in parts[1].chars() {
        match ch {
            'H' => { total += 1; hits += 1; }
            'O' => { total += 1; }
            _ => {} 
        }
    }
    println!("Player {}'s average is : {}", num, hits as f32 / total as f32);
}

fn main() {
    // Get input strings from file
    let filename = match env::args().nth(1) {
        Some(v) => v,
        None => { panic!("Enter a file!"); }
    };
    let reader = BufReader::new(fs::File::open(&filename).unwrap());
    for line in reader.lines() {
        let l = match line {
            Ok(val) => val,
            Err(why) => panic!("Could not read line : {}", Error::description(&why)),
        };
        process_line(l);
    }
}
