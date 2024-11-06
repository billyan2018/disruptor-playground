use disruptor::*;
use std::sync::Mutex;
use std::thread;
use std::fmt;

struct Event {
    price: f64,
    data: Mutex<Vec<String>>,
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.data.lock().unwrap();
        f.debug_struct("Event")
            .field("price", &self.price)
            .field("data", &*data)
            .finish()
    }
}
impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self.data.lock().unwrap();
        write!(f, "Event {{ price: {}, data: {:?} }}", self.price, *data)
    }
}

fn main() {

    let factory = || { Event { 
        price: 0.0,
        data: Mutex::new(vec![]), 
    }};
    let h1 = |e: &Event, sequence: Sequence, end_of_batch: bool| {
        e.data.lock().unwrap().push("h1".to_string());
        println!("h1: {:?}", e);
    };
    let h2 = |e: &Event, sequence: Sequence, end_of_batch: bool| {
        e.data.lock().unwrap().push("h2".to_string());
        println!("h2: {}", e);
    };
    let h3 = |e: &Event, sequence: Sequence, end_of_batch: bool| {
        e.data.lock().unwrap().push("h3".to_string());
        println!("h3: {}", e);
    };

    let mut producer1 = disruptor::build_multi_producer(64, factory, BusySpin)
        .pin_at_core(1).handle_events_with(h1)
        .pin_at_core(2).handle_events_with(h2)
        .and_then()
        .pin_at_core(3).handle_events_with(h3)
        .build();

    let mut producer2 = producer1.clone();

    let handle1 = thread::spawn(move || {
        for i in 0..10 {
            producer1.publish(|e| {
                e.price = i as f64;
            });
        }
    });

    let handle2 = thread::spawn(move || {
        for i in 10..20 {
            producer2.publish(|e| {
                e.price = i as f64;
            });
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}