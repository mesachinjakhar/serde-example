use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    email: String,
    username: String
}


fn main() {
    println!("Hello, world!");
}
