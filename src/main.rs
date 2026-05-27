// buat helo word

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
    zip_code: u32,
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    address: Address,
}

fn main() {
    let user = User {
        name: "John Doe".to_string(),
        age: 30,
        address: Address {
            street: "123 Main St".to_string(),
            city: "Jakarta".to_string(),
            zip_code: 12345,
        },
    };

    // Struct ke JSON
    let user_json = serde_json::to_string(&user).unwrap();
    println!("ini adalah JSON : {}", user_json);

    // JSON ke Struct
    let received: User = serde_json::from_str(&user_json).unwrap();
    println!("JSON ke Struct : {} - {}", received.name, received.age);
    println!(
        "JSON ke Address: {}, {}",
        received.address.street, received.address.city
    );

    // JSON ke Struct
    let received: User = serde_json::from_str(&user_json).unwrap();
    println!("JSON ke Struct : {} - {}", received.name, received.age);
    println!(
        "JSON ke Struct Address: {}, {}",
        received.address.street, received.address.city
    );
}
