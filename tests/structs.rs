// Structs used in testing

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Person {
    name: String,
    // Bencoding supports neither enums, or bools
    gender: String,
    age: u16,
}
