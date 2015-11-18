extern crate calendar_queue;

use calendar_queue::{CalendarQueue, Error, Result};
use std::collections::HashMap;

type Packet = String;

#[test]
fn queue_creation() {
    let queue = CalendarQueue::<Packet>::new();
}

#[test]
fn single_flow() {
    let mut queue = CalendarQueue::<Packet>::new();
    let flow = queue.add(1, 1).unwrap();
    // Ensure we can send.
    flow.send("Foo".into()).unwrap();
}

#[test]
fn colliding_flow_is_handled() {
    let mut queue = CalendarQueue::<Packet>::new();
    let flow = queue.add(1, 1);
    // Same ID! Should fail!
    match queue.add(1, 1) {
        Err(Error::DuplicateFlowId(1)) => return,
        _ => panic!("Did not return an error on colliding FlowId")
    }
}

#[test]
fn multi_flow() {
    let mut queue = CalendarQueue::<Packet>::new();
    let flow = queue.add(1, 1).unwrap();
    let flow = queue.add(2, 2).unwrap();
    // Ensure we can send.
    flow.send("Foo".into()).unwrap();
}
