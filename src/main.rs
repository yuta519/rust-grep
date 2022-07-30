use chrono::{DateTime, Local};
fn main() {
    let dt: DateTime<Local> = Local::now();
    let timestamp: i64 = dt.timestamp();
    let world: &str = if timestamp % 2 == 0 {
        "devided!!!!!!!!!!!!!"
    } else {
        "non devided!!!!!!!!!!!!!"
    };
    println!("Hello {}", world);
}
