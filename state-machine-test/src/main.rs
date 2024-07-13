// A trait to constrain the state types
trait State {}

// Define the states of the state machine
struct State1;
struct State2;
struct State3;
struct State4;

// Implement the State trait for the state types
impl State for State1 {}
impl State for State2 {}
impl State for State3 {}
impl State for State4 {}

// Define the state machine struct, with a phantom type parameter to represent the current state
// The phantom type parameter is constrained to types that implement the State trait so that only valid states can be used
// The state machine struct has a common method that can be called in any state
// The state machine struct has methods to transition between states
// Each transition method takes self by value and returns a new state machine with the new state
// The state machine struct implements the Display trait to print the current state
// The state machine struct is instantiated with the initial state State1
struct StateMachine<StateT: State = State1> {
    state: std::marker::PhantomData<StateT>,
}

// Implement the state machine struct for any state type that implements the State trait
impl<StateT: State> StateMachine<StateT> {
    // Define a common method that can be called in any state
    fn common_method(&self) -> String {
        // Return the current state of the state machine as a string
        format!(
            "Common method called in State: {}",
            std::any::type_name::<StateT>()
        )
    }
}

// Implement the state machine struct new method for the initial state State1
impl StateMachine {
    // Define a new method to create a state machine with the initial state State1
    pub fn new() -> Self {
        StateMachine {
            state: Default::default(),
        }
    }
}

// Implement the state machine struct with methods to transition between states
impl StateMachine<State1> {
    // Define a method to transition from State1 to State2
    fn transition_to_state2(self) -> StateMachine<State2> {
        StateMachine {
            state: std::marker::PhantomData::<State2>,
        }
    }
}

impl StateMachine<State2> {
    // Define a method to transition from State2 to State3
    fn transition_to_state3(self) -> StateMachine<State3> {
        StateMachine {
            state: std::marker::PhantomData::<State3>,
        }
    }
}

impl StateMachine<State3> {
    // Define a method to transition from State3 to State4
    fn transition_to_state4(self) -> StateMachine<State4> {
        StateMachine {
            state: std::marker::PhantomData::<State4>,
        }
    }
}

impl StateMachine<State4> {
    // Define a method to transition from State4 to State1
    fn transition_to_state1(self) -> StateMachine<State1> {
        StateMachine {
            state: std::marker::PhantomData::<State1>,
        }
    }
}

// Implement the Display trait for the StateMachine struct
impl<StateT: State> std::fmt::Display for StateMachine<StateT> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "State: {:?}", std::any::type_name::<StateT>())
    }
}

fn main() {
    // Create a new state machine with the initial state State1
    let state_machine = StateMachine::new();
    println!("StateMachine in State: {}", state_machine);
    println!("{}", state_machine.common_method());

    // Transition to State2, State3, State4, and back to State1
    let state_machine = state_machine.transition_to_state2();
    println!("StateMachine in State: {}", state_machine);
    println!("{}", state_machine.common_method());

    let state_machine = state_machine.transition_to_state3();
    println!("StateMachine in State: {}", state_machine);
    println!("{}", state_machine.common_method());

    let state_machine = state_machine.transition_to_state4();
    println!("StateMachine in State: {}", state_machine);
    println!("{}", state_machine.common_method());

    let state_machine = state_machine.transition_to_state1();
    println!("StateMachine in State: {}", state_machine);
    println!("{}", state_machine.common_method());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_machine() {
        // Create a new state machine with the initial state State1
        let state_machine = StateMachine::new();
        assert_eq!(
            format!("{}", state_machine),
            "State: \"state_machine_test::State1\""
        );

        // Transition to State2, State3, State4, and back to State1
        let state_machine = state_machine.transition_to_state2();
        assert_eq!(
            format!("{}", state_machine),
            "State: \"state_machine_test::State2\""
        );

        let state_machine = state_machine.transition_to_state3();
        assert_eq!(
            format!("{}", state_machine),
            "State: \"state_machine_test::State3\""
        );

        let state_machine = state_machine.transition_to_state4();
        assert_eq!(
            format!("{}", state_machine),
            "State: \"state_machine_test::State4\""
        );

        let state_machine = state_machine.transition_to_state1();
        assert_eq!(
            format!("{}", state_machine),
            "State: \"state_machine_test::State1\""
        );
    }

    // Test the common method
    #[test]
    fn test_common_method() {
        // Create a new state machine with the initial state State1
        let state_machine = StateMachine::new();
        assert_eq!(
            state_machine.common_method(),
            "Common method called in State: state_machine_test::State1"
        );

        // Transition to State2
        let state_machine = state_machine.transition_to_state2();
        assert_eq!(
            state_machine.common_method(),
            "Common method called in State: state_machine_test::State2"
        );

        // Transition to State3
        let state_machine = state_machine.transition_to_state3();
        assert_eq!(
            state_machine.common_method(),
            "Common method called in State: state_machine_test::State3"
        );

        // Transition to State4
        let state_machine = state_machine.transition_to_state4();
        assert_eq!(
            state_machine.common_method(),
            "Common method called in State: state_machine_test::State4"
        );

        // Transition back to State1
        let state_machine = state_machine.transition_to_state1();
        assert_eq!(
            state_machine.common_method(),
            "Common method called in State: state_machine_test::State1"
        );
    }
}
