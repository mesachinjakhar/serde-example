use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]
// #[serde(rename_all(serialize = "UPPERCASE"))]
pub struct Point {
    x: i32,
    y: i32
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all_fields(serialize = "camelCase"))]
#[serde(tag = "type", content = "data")]
enum Event {
    UserCreated { user_id: i32, user_name: String },
    UserDeleted { user_id: i32 },
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct User {
    id: i32,
    name: String,
}


fn main() {
    let point = Point {
        x: 1,
        y: 2
    };

    let event = Event::UserCreated { user_id: 123, user_name: "mesachinjakhar".to_string() };
    let event_serialized = serde_json::to_string(&event).unwrap();
       println!("event serialized: {}", event_serialized);
    
    // convert struct into JSON string
    let serialized = serde_json::to_string(&point).unwrap();

    println!("serialized: {}", serialized);

    // convert json string back to the strcut
    let deserialized: Point = serde_json::from_str(&serialized).unwrap(); 
    println!("deserialized: {:?}", deserialized);
}
