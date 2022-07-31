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

    println!("標準入力テストです。任意の値を入力してください。");
    match std::env::args().nth(1) {
        Some(path) => cat_cmd(path),
        None => println!("No path is specified!"),
    }
    cat_cmd("./Cargo.toml".to_string());
}

fn cat_cmd(path: String) {
    println!("\n{}", path);
    println!("###########################################\n");
    match read_to_string(path) {
        Ok(content) => print!("{}", content),
        Err(reason) => print!("{}", reason),
    }
}
