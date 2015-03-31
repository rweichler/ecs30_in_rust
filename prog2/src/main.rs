use std::io;
use std::io::Write;
use std::str::FromStr;


fn scan(x: &str) -> Result<f32, <f32 as FromStr>::Err>{
    let mut out = io::stdout();
    print!("{}", x);
    out.flush();

    let mut stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line);
    let len = line.len();
    line.truncate(len - 1);
    f32::from_str(&line)
}

fn main(){

    let min = match scan("Minutes for the runner: "){
        Ok(v) => v,
        Err(E) => { println!("{}", E); return }
    };

    let sec = match scan("Seconds for the runner: "){
        Ok(v) => v,
        Err(E) => { println!("{}", E); return }
    };

    
}
