extern crate calendar_queue;

use calendar_queue::{CalendarQueue, Error};

type FlowId = u64;
type Packet = String;

#[test]
fn add_single_flow() {
    let mut queue = CalendarQueue::<FlowId, Packet>::new();
    let (sender, receiver) = std::sync::mpsc::channel();
    queue.add_channel(receiver, 1, 1)
        .unwrap();
    // Ensure we can send.
    sender.send("Foo".into())
        .unwrap();
}

#[test]
fn colliding_flow_is_handled() {
    let mut queue = CalendarQueue::<FlowId, Packet>::new();
    let _ = queue.create_channel(1, 1)
        .unwrap();
    // Same ID! Should fail!
    match queue.create_channel(1, 1) {
        Err(Error::DuplicateFlowId) => return,
        _ => panic!("Did not return an error on colliding FlowId")
    }
}

#[test]
fn multi_flow() {
    let mut queue = CalendarQueue::<FlowId, Packet>::new();
    let flow_1 = queue.create_channel(1, 1)
        .unwrap();
    let flow_2 = queue.create_channel(2, 2)
        .unwrap();
    // Ensure we can send.
    flow_1.send("Foo".into())
        .unwrap();
    flow_2.send("Bar".into())
        .unwrap();
}
