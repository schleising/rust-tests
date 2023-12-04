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
    pub name: Option<String>,
    #[serde(rename = "JsonAge", skip_serializing_if = "Option::is_none")]
    pub age: Option<i64>,
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
        name: Some("John Doe".to_string()),
        age: Some(43),
        phones: phones.clone(),
    };

    let person_no_age = Person {
        name: None,
        age: None,
        phones,
    };

    // Serialise Person to JSON
    let serialised = match serde_json::to_string(&person) {
        Ok(s) => s,
        Err(e) => panic!("Error serialising: {}", e),
    };

    println!("Serialised: {}", serialised);

    // Deserialise JSON to Person
    let deserialised: Person = match serde_json::from_str(&serialised) {
        Ok(p) => p,
        Err(e) => panic!("Error deserialising: {}", e),
    };

    println!("Deserialised: {:?}", deserialised);

    // Serialise Person with no age to JSON
    let serialised_no_age = match serde_json::to_string(&person_no_age) {
        Ok(s) => s,
        Err(e) => panic!("Error serialising: {}", e),
    };

    println!("Serialised No Age: {}", serialised_no_age);

    // Deserialise JSON to Person with no age
    let deserialised_no_age: Person = match serde_json::from_str(&serialised_no_age) {
        Ok(p) => p,
        Err(e) => panic!("Error deserialising: {}", e),
    };

    println!("Deserialised No Age: {:?}", deserialised_no_age);

}
