use chrono::{DateTime, Local};
fn main() {
    let dt: DateTime<Local> = Local::now();
    let timestamp: i64 = dt.timestamp();
    let mut world: &str = "";

    if timestamp % 2 == 0 {
        println!("{}", timestamp);
        world = "devided!!!!!!!!!!!!!";
    } else {
        world = "non devided!!!!!!!!!!!!!";
    }
    println!("Hello {}", world);
}
