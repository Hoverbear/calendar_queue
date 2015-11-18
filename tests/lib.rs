extern crate calendar_queue;

use calendar_queue::CalendarQueue;
use std::collections::HashMap;

type Packet = String;

#[test]
fn queue_creation() {
    let queue = CalendarQueue::<Packet>::new();
}

#[test]
fn queue_makes_a_flow() {
    let mut queue = CalendarQueue::<Packet>::new();
    let flow = queue.add(1, 1);
    flow.send("Foo".into());
}
