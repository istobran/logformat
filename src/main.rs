use std::env;
use std::io;
use chrono::prelude::*;

fn format(format_str: &str) -> String {
    let now: DateTime<Local> = Local::now();
    return now.format(format_str).to_string();
}

fn get_prefix_format() -> String {
    let args: Vec<String> = env::args().collect();
    let mut dirty_format: Option<String> = None;
    if args.len() > 1 {
        dirty_format = Some(args[1].to_string())
    }
    return dirty_format.unwrap_or(String::from("%Y-%m-%d %H:%M:%S"));
}

fn main() {
    let mut input = String::new();
    loop {
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 { std::process::exit(0); }
                print!("{} {}", format(&get_prefix_format()), input);
            }
            Err(error) => {
                eprintln!("error: {}", error);
                std::process::exit(1);
            }
        }
    }
}