const DEBUG: bool = false;

// FSM for a string containing ab
#[derive(Debug)]
enum State {
    A,
    B,
    C,
    Error(String),
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Event {
    a,
    b,
}

impl State {
    fn init() -> State {
        State::A
    }
    fn next(self, event: &Event) -> State {
        match (self, event) {
            (State::A, Event::a) => State::B,
            (State::A, Event::b) => State::A,
            (State::B, Event::a) => State::C,
            (State::B, Event::b) => State::A,
            (State::C, Event::a) => State::C,
            (State::C, Event::b) => State::C,
            // (state, event) => panic!(format!("Error in state: {:#?} for event: {:#?}", state, event)),
            (state, event) => State::Error(
                format!("No transistion for the state: {:#?} for event: {:#?}. Are you sure this is an FSM?", state, event).to_string(),
            ),
        }
    }
}

pub fn fsm() {
    loop {
        let mut state = State::init();
        let mut events = vec![];
        let mut string = String::new();
        println!("Enter the string: ");
        std::io::stdin().read_line(&mut string).unwrap();
        for character in string.trim().chars() {
            let event;
            if character == 'a' {
                event = Event::a;
            } else if character == 'b' {
                event = Event::b;
            } else {
                println!("{}", character);
                panic!("Input not in a, b");
            }
            events.push(event)
        }
        for event in events.iter() {
            if DEBUG {
                print!("-- Got Event {:#?}, Transitioned from {:#?}", event, state)
            };
            state = state.next(&event);
            if DEBUG {
                println!(" to {:#?}", state);
            }

            if let State::Error(ref string) = state {
                println!("{}", string);
                break;
            }
        }
        if let State::C = state {
            println!("This string is accepted!")
        } else {
            println!("This string is not accepted!")
        }
    }
}
