use chrono::{DateTime, Local};
use std::fs::read_to_string;

fn main() {
    let dt: DateTime<Local> = Local::now();
    let timestamp: i64 = dt.timestamp();
    let world: &str = if timestamp % 2 == 0 {
        "devided!!!!!!!!!!!!!"
    } else {
        "non devided!!!!!!!!!!!!!"
    };
    println!("Hello {}", world);
    cat_cmd("./src/main.rs")
}

fn cat_cmd(path: &str) {
    match read_to_string(path) {
        Ok(content) => print!("{}", content),
        Err(reason) => print!("{}", reason),
    }
}
