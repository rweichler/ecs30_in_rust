use std::str::FromStr;

use std::io::{
    Write,
    Read
};

use std::{
    io,
    env,
    path,
    fs
};

use parser::Parser;
mod parser;

fn main() {
    let courses = Parser::new(&get_file_shit());

    let mut choice;
    while {choice = get_choice(); choice != 0} {
        if choice == 1 {
            let line = read_from_stdin("Please enter a CRN: ");
            match i32::from_str(&line) {
                Ok(v @ 10000...99999) => courses.from_crn(v),
                _ => println!("Enter a valid CRN you fucking asshole")
            };
        } else if choice == 2 {
            let line = read_from_stdin("Please enter a subject: ");
            courses.from_subject(line);
        }
    }
}

fn get_file_shit() -> String {
    let arg1 = match env::args().nth(1) {
        Some(v) => v,
        _ => "lol.html".to_string()
    };

    let path = path::Path::new(&arg1);
    let display = path.display();
    let mut file = match fs::File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file
    };

    let mut result = String::new();
    match file.read_to_string(&mut result) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        _ => result
    }
}

fn read_from_stdin(msg : &str) -> String {
    let mut out = io::stdout();
    print!("{}", msg);
    if let Err(why) = out.flush() {
        panic!("error flushing stdout: {}", why);
    }

    let mut stdin = io::stdin();
    let mut line;
    let mut len;
    
    while { //do while
        line = String::new();
        if let Err(why) = stdin.read_line(&mut line) {
            panic!("error reading from stdin: {}", why);
        }
        len = line.len();

        len == 1 //condition
    } {}
    line.truncate(len - 1);

    line
}

fn get_choice() -> u8 {
    let line = read_from_stdin(
            "\nRSVP Menu\n\
            0. Done.\n\
            1. Find by CRN.\n\
            2. Find by subject.\n\
            Your choice (0 - 2): ");

    match u8::from_str(&line) {
        Ok(v @ 0 ... 2) => v,
        _ => {
            println!("Enter a valid choice you fucking asshole");
            get_choice()
        }
    }
}
