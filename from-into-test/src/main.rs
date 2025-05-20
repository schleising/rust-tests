struct Person {
    name: String,
}

impl Person {
    fn new(name: impl Into<String>) -> Self {
        Person { name: name.into() }
    }
}

struct JobRole {
    person: Person,
    role: String,
}

impl JobRole {
    fn new(person: impl Into<Person>, role: impl Into<String>) -> Self {
        JobRole {
            person: person.into(),
            role: role.into(),
        }
    }
}

impl From<&str> for Person {
    fn from(name: &str) -> Self {
        Person { name: name.to_string() }
    }
}

impl From<JobRole> for Person {
    fn from(job_role: JobRole) -> Self {
        job_role.person
    }
}

fn main() {
    let person = Person::new("Alice");
    println!("String Slice: {}", person.name);

    let person = Person::new("Bob".to_string());
    println!("To String: {}", person.name);

    let person = Person::new("Charlie".to_owned());
    println!("To Owned: {}", person.name);

    let person = Person::new(String::from("David"));
    println!("String::From: {}", person.name);

    let person = Person::new("Eve".to_string());
    println!("Format: {}", person.name);

    let job_role = JobRole::new("Frank", "Manager");
    println!("Into: {}", job_role.role);
    let person: Person = job_role.into();
    println!("Into: {}", person.name);

    // let job: JobRole = person.into();
}
