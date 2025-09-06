use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TestType {
    name: String,
    age: u32,
}

// Implement Display for TestType
impl Display for TestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "(name: {}, age: {})", self.name, self.age)
    }
}

fn print_type_of<T>(_: &T) {
    println!("Type: {}", std::any::type_name::<T>())
}

// Create a new test type and put it in a Box
fn box_test_type() -> Box<TestType> {
    let boxed_test_type = Box::new(TestType {
        name: "Box".to_string(),
        age: 32,
    });

    // Print the test type
    println!("boxed_test_type: {boxed_test_type}");

    // Print the name of the test type
    println!("boxed_test_type.name: {}", boxed_test_type.name);

    // Print the age of the test type
    println!("boxed_test_type.age: {}", boxed_test_type.age);

    // Return the test type
    boxed_test_type
}

// Create a new test type and put it in a Rc
fn rc_test_type() -> Rc<TestType> {
    let rc_test_type = Rc::new(TestType {
        name: "Rc".to_string(),
        age: 32,
    });

    // Print the test type
    println!("rc_test_type: {rc_test_type}");

    // Print the name of the test type
    println!("rc_test_type.name: {}", rc_test_type.name);

    // Print the age of the test type
    println!("rc_test_type.age: {}", rc_test_type.age);

    // Return the test type
    rc_test_type
}

// Create a new test type and put it in RefCell withing a Rc
fn rc_refcell_test_type() -> Rc<RefCell<TestType>> {
    let rc_refcell_test_type = Rc::new(RefCell::new(TestType {
        name: "RcRefCell".to_string(),
        age: 32,
    }));

    // Print the test type
    println!("rc_refcell_test_type.borrow(): {}", rc_refcell_test_type.borrow());

    // Print the name of the test type
    println!("rc_refcell_test_type.borrow().name: {}", rc_refcell_test_type.borrow().name);

    // Print the age of the test type
    println!("rc_refcell_test_type.borrow().age: {}", rc_refcell_test_type.borrow().age);

    // Return the test type
    rc_refcell_test_type
}

fn main() {
    // Create a test type
    let test_type = TestType {
        name: "Test".to_string(),
        age: 32,
    };

    // Print the test type
    println!("test_type: {test_type}");

    // Print the type of the test type
    print_type_of(&test_type);

    // Debug print the test type
    dbg!(test_type);

    // Create a new test type and put it in a Box
    let boxed_test_type = box_test_type();

    // Print the test type
    println!("boxed_test_type: {boxed_test_type}");

    // Print the type of the boxed test type
    print_type_of(&boxed_test_type);

    // Debug print the test type
    dbg!(boxed_test_type);

    // Create a new test type and put it in a Rc
    let rc_test_type = rc_test_type();

    // Print the test type
    println!("rc_test_type: {rc_test_type}");

    // Print the type of the Rc test type
    print_type_of(&rc_test_type);

    // Debug print the test type
    dbg!(rc_test_type);

    // Create a new test type and put it in RefCell withing a Rc
    let rc_refcell_test_type = rc_refcell_test_type();

    // Print the test type
    println!("rc_refcell_test_type.borrow(): {}", rc_refcell_test_type.borrow());

    // Print the type of the Rc RefCell test type
    print_type_of(&rc_refcell_test_type);

    // Debug print the test type
    dbg!(rc_refcell_test_type);
}
