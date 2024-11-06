use std::{cell::RefCell, rc::Rc};
use disruptor::*;

// The event on the ring buffer.
struct Event {
    price: f64
}

// Your custom state.
#[derive(Default)]
struct State {
    data: Rc<RefCell<i32>>
}
fn main() {
let factory = || { Event { price: 0.0 }};
let initial_state = || { State::default() };

// Closure for processing events *with* state.
let processor = |s: &mut State, e: &Event, _: Sequence, _: bool| {
    // Mutate your custom state:
    *s.data.borrow_mut() += 1;
};

let size = 64;
let mut producer = disruptor::build_single_producer(size, factory, BusySpin)
    .handle_events_and_state_with(processor, initial_state)
    .build();

// Publish into the Disruptor via the `Producer` handle.
for i in 0..10 {
    producer.publish(|e| {
        e.price = i as f64;
    });
}
}