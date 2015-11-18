extern crate calendar_queue;

use calendar_queue::{CalendarQueue, Error};

type FlowId = u64;
type Packet = String;

#[test]
fn queue_creation() {
    let _ = CalendarQueue::<FlowId, Packet>::new();
}

#[test]
fn single_flow() {
    let mut queue = CalendarQueue::<FlowId, Packet>::new();
    let flow = queue.add(1, 1).unwrap();
    // Ensure we can send.
    flow.send("Foo".into()).unwrap();
}

#[test]
fn colliding_flow_is_handled() {
    let mut queue = CalendarQueue::<FlowId, Packet>::new();
    let _ = queue.add(1, 1);
    // Same ID! Should fail!
    match queue.add(1, 1) {
        Err(Error::DuplicateFlowId) => return,
        _ => panic!("Did not return an error on colliding FlowId")
    }
}

#[test]
fn multi_flow() {
    let mut queue = CalendarQueue::<FlowId, Packet>::new();
    let flow_1 = queue.add(1, 1).unwrap();
    let flow_2 = queue.add(2, 2).unwrap();
    // Ensure we can send.
    flow_1.send("Foo".into()).unwrap();
    flow_2.send("Bar".into()).unwrap();
}
