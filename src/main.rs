use chrono::{DateTime, Local};
fn main() {
    let world: &str = "world!!!!!!!!!!!!!";
    println!("Hello {}", world);
    let dt: DateTime<Local> = Local::now();
    let timestamp: i64 = dt.timestamp();

    println!("{}", timestamp);
}
