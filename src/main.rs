#[macro_use]
extern crate derive_new;

fn main() {
    println!("Start");

    let state_a = StateA::new("my_id".to_string(), Stuff{}, Resources{});
    let msg: Option<Message> = Some(Message{});

    // Only way to go from A to B is with a message and consume state_a
    StateB::from((state_a, msg.unwrap()));

    // Not possible!
//    let state_b: StateB = state_a.into();
//    let state_b: StateB = StateB{id: state_a.id.clone(), msg: Message{}};




}

struct SomeHolder {
    cur_state: ThingIn,


}
impl SomeHolder {

    fn do_something(&mut self) -> () {
        if let ThingIn::A(state_a) = &mut self.cur_state {
            let msg: Option<Message> = None; // Network call. Can be some or None
            // If i have to explicitly clone and pass stuff, state_a isn't really fully consumed.
//            self.cur_state = StateB::from((state_a.id, msg.unwrap())).into();
//            state_a.mut_self_fn();
//            self.cur_state = ThingIn::B(StateB::from((state_a, msg.unwrap())));
            self.cur_state = self.cur_state.transition(msg.unwrap()); // This 
            self.cur_state = ThingIn::Dummy;
//            let temp = &state_a.id; // This complains, because state_a has been moved away into the reassigning of self.cur_state

        }



    }
}

//struct StateCollection
enum ThingIn {
    A(Thing<StateA>),
    B(Thing<StateB>),
    C(Thing<StateC>),
    Dummy
}

impl ThingIn {
    pub fn transition(mut self, msg: Message) -> Self {
        match self {
            ThingIn::A(state_a) => ThingIn::B(StateB::from((state_a, msg)).into()),
            _ => self
        }
    }

}
impl From<StateB> for ThingIn {
    fn from(_: StateB) -> Self {
        unimplemented!()
    }
}
struct Thing<S> {
    state: S,
}


#[derive(new)]
struct StateA {
    id: String,
    other_state_a_related_stuff: Stuff,
    resources: Resources,
}
impl StateA {
    fn mut_self_fn(&mut self) {

    }
}
struct Stuff;

/** Important message */
struct Message {}



use restricted_state::StateB;
use std::cell::RefCell;

mod restricted_state {
    use crate::Message;
    use crate::StateA;
    use crate::Thing;

    pub struct StateB {
        id: String,
        msg: Message,
    }

    impl From<(StateA, Message)> for StateB {
        fn from((state, msg): (StateA, Message)) -> Self {
            let state_b = StateB {
                id: state.id,
                msg
            };
            state_b
        }
    }

    impl From<(Thing<StateA>, Message)> for StateB {
        fn from(_: (Thing<StateA>, Message)) -> Self {
            unimplemented!()
        }
    }

    impl From<(String, Message)> for StateB {
        fn from((val, msg): (String, Message)) -> Self {
            let state_b = StateB {
                id: val,
                msg
            };
            state_b
        }
    }
}

trait IntoThing<F, T> {
    fn into_thing(self) -> T;
}

impl<F,T> IntoThing<F,T> for F where T: From<F> {
    fn into_thing(self) -> T {
        T::from(self)
    }
}

impl From<StateB> for Thing<StateB> {
    fn from(_: StateB) -> Self {
        unimplemented!()
    }
}

struct StateC {}

/// Helper types
///
struct Resources {}
