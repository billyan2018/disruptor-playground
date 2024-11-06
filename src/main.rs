use disruptor::*;
use std::thread;

struct Event {
    price: f64
}

fn main() {

    let factory = || { Event { price: 0.0 }};
    let h1 = |e: &Event, sequence: Sequence, end_of_batch: bool| {
        println!("h1: {}", e.price);
    };
    let h2 = |e: &Event, sequence: Sequence, end_of_batch: bool| {
        println!("h2: {}", e.price);
    };
    let h3 = |e: &Event, sequence: Sequence, end_of_batch: bool| {
        println!("h3: {}", e.price);
    };

    let mut producer1 = disruptor::build_multi_producer(64, factory, BusySpin)
        .pin_at_core(1).handle_events_with(h1)
        .pin_at_core(2).handle_events_with(h2)
        .and_then()
        .pin_at_core(3).handle_events_with(h3)
        .build();


    let mut producer2 = producer1.clone();

  
    thread::scope(|s| {
        s.spawn(move || {
            for i in 0..10 {
                producer1.publish(|e| {
                    e.price = i as f64;
                });
            }
        });
        s.spawn(move || {
            for i in 10..20 {
                producer2.publish(|e| {
                    e.price = i as f64;
                });
             }
        });
    });
}