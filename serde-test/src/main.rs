use serde::{Deserialize, Serialize};
use serde_json;

// I used https://typegen.vestera.as/ to generate the following code from the JSON below.
// {
//   "name": "John Doe",
//   "age": 43,
//   "phones": {
//     "home": "1234567",
//     "mobile": "2345678"
//   }
// }
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: i64,
    pub phones: Phones,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Phones {
    pub home: String,
    pub mobile: String,
}

fn main() {
    let phones = Phones {
        home: "1234567".to_string(),
        mobile: "2345678".to_string(),
    };

    let person = Person {
        name: "John Doe".to_string(),
        age: 43,
        phones,
    };

    // Convert the Person to a JSON string.
    let serialized: String = match serde_json::to_string(&person) {
        Ok(json) => json,
        Err(e) => panic!("Error serializing: {}", e),
    };

    // Print, write to a file, or send to an HTTP server.
    println!("{}", serialized);

    // Convert the JSON string back to a Person.
    let deserialized: Person = match serde_json::from_str(&serialized) {
        Ok(person) => person,
        Err(e) => panic!("Error deserializing: {}", e),
    };

    // Print, write to a file, or send to an HTTP server.
    println!("{:?}", deserialized);
}
