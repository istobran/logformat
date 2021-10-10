use chrono::prelude::*;

fn format(format_str: &str) -> String {
    let now: DateTime<Local> = Local::now();
    return now.format(format_str).to_string();
}

fn main() {
    println!("{}", format("%Y-%m-%d %H:%M:%S"));
}
