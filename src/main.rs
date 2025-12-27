use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: i32,
    y: i32
}

fn main() {
    let point = Point {
        x: 1,
        y: 2
    };
    
    // convert struct into JSON string
    let serilized = serde_json::to_string(&point).unwrap();

    println!("serilized: {}", serilized);
}
