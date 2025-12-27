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
    let serialized = serde_json::to_string(&point).unwrap();

    println!("serialized: {}", serialized);

    // convert json string back to the strcut
    let deserialized: Point = serde_json::from_str(&serialized).unwrap(); 
    println!("deserialized: {:?}", deserialized);
}
