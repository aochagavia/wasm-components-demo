use std::cell::RefCell;

use crate::bindings::exports::ochagavia::test::guest_types::{Guest, GuestGreeter};
use crate::bindings::ochagavia::test::host_types::Rng;

mod bindings {
    include!("my_world.rs");

    use super::GuestTypes;
    export!(GuestTypes);
}

struct GuestTypes;
struct Greeter {
    rng: RefCell<Rng>,
}

impl Guest for GuestTypes {
    type Greeter = Greeter;
}

impl GuestGreeter for Greeter {
    fn new(r: Rng) -> Self {
        Self { rng: r.into() }
    }

    fn greet(&self) -> String {
        let rng = self.rng.borrow_mut();
        if rng.next_u32(1) == 0 {
            "hello from rust".to_string()
        } else {
            "bye bye from rust".to_string()
        }
    }
}
