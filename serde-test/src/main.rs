use serde::{Deserialize, Serialize};

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct WithOption<T> {
    value: Option<T>,
}

fn main() {
    let phones: Phones = Phones {
        home: "1234567".to_string(),
        mobile: "2345678".to_string(),
    };

    let person: Person = Person {
        name: "John Doe".to_string(),
        age: 43,
        phones,
    };

    let with_option: WithOption<Person> = WithOption {
        value: Some(person),
    };

    let with_option_none: WithOption<Person> = WithOption { value: None };

    let serialized = match serde_json::to_string(&with_option) {
        Ok(s) => s,
        Err(e) => panic!("Error serializing: {}", e),
    };

    println!("serialized = {}", serialized);

    let deserialized: WithOption<Person> = match serde_json::from_str(&serialized) {
        Ok(s) => s,
        Err(e) => panic!("Error deserializing: {}", e),
    };

    println!("deserialized = {:?}", deserialized);

    let serialized = match serde_json::to_string(&with_option_none) {
        Ok(s) => s,
        Err(e) => panic!("Error serializing: {}", e),
    };

    println!("serialized = {}", serialized);

    let deserialized: WithOption<Person> = match serde_json::from_str(&serialized) {
        Ok(s) => s,
        Err(e) => panic!("Error deserializing: {}", e),
    };

    println!("deserialized = {:?}", deserialized);
}
