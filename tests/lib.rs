extern crate calendar_queue;

use calendar_queue::{CalendarQueue, Error};
use std::sync::mpsc::channel;

type FlowId = u64;
type Packet = String;

// Printing the error.
#[test]
fn error_prints() {
    assert_eq!(
        format!("{:?}", Error::DuplicateFlowId),
        "DuplicateFlowId"
    );
}

// Basic functionality code.
#[test]
fn add_single_flow() {
    let mut queue = CalendarQueue::<FlowId, Packet>::new();
    let (sender, receiver) = channel();
    queue.add_channel(receiver, 1, 1)
        .unwrap();
    // Ensure we can send.
    sender.send("Foo".into())
        .unwrap();
    assert_eq!(queue.next(), Some("Foo".into()));
}

// Ensure that a colliding flow gives an error.
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

// This test ensures the sorter supports multiple flows.
#[test]
fn multi_flow() {
    let mut queue = CalendarQueue::<FlowId, Packet>::new();
    let flow_1 = queue.create_channel(1, 10)
        .unwrap();
    let flow_2 = queue.create_channel(2, 10)
        .unwrap();
    // Ensure we can send.
    flow_1.send("Foo".into())
        .unwrap();
    flow_2.send("Bar".into())
        .unwrap();
    assert_eq!(queue.next(), Some("Foo".into()));
    assert_eq!(queue.next(), Some("Bar".into()));
}

// This test ensures that given the correct conformance times that `flow_2`'s message goes after
// two of `flow_1`'s.
#[test]
fn multi_flow_big_priority_diff() {
    let mut queue = CalendarQueue::<FlowId, Packet>::new();
    let flow_1 = queue.create_channel(1, 4)
        .unwrap();
    let flow_2 = queue.create_channel(2, 10)
        .unwrap();
    // Flow 1
    flow_1.send("1 Foo".into())
        .unwrap();
    flow_1.send("1 Bar".into())
        .unwrap();
    flow_1.send("1 Baz".into())
        .unwrap();
    // Flow 2
    flow_2.send("2 Foo".into())
        .unwrap();
    flow_2.send("2 Bar".into())
        .unwrap();
    flow_2.send("2 Baz".into())
        .unwrap();
    assert_eq!(queue.next(), Some("1 Foo".into()));
    assert_eq!(queue.next(), Some("2 Foo".into()));
    assert_eq!(queue.next(), Some("1 Bar".into()));
    assert_eq!(queue.next(), Some("1 Baz".into()));
    assert_eq!(queue.next(), Some("2 Bar".into()));
    // 1 exhausted.
    assert_eq!(queue.next(), Some("2 Baz".into()));
}

// This test ensures that using `next()` will not yield `None` on gaps in the sorter.
#[test]
fn multi_flow_gaps() {
    let mut queue = CalendarQueue::<FlowId, Packet>::new();
    let flow_1 = queue.create_channel(1, 2)
        .unwrap();
    let flow_2 = queue.create_channel(2, 10)
        .unwrap();
    // Ensure we can send.
    flow_1.send("1 Foo".into())
        .unwrap();
    flow_1.send("1 Bar".into())
        .unwrap();
    flow_2.send("2 Foo".into())
        .unwrap();
    flow_2.send("2 Bar".into())
        .unwrap();
    assert_eq!(queue.next(), Some("1 Foo".into()));
    assert_eq!(queue.next(), Some("2 Foo".into()));
    assert_eq!(queue.next(), Some("1 Bar".into()));
    assert_eq!(queue.next(), Some("2 Bar".into()));
}

// No point in making this a doc test, `rustdoc` doesn't show it. Even on the `impl` :(
// Ensure ticks until it finds something.
#[test]
fn next_functions() {
    let mut queue = CalendarQueue::<u64, String>::new();
    let (sender, receiver) = channel();
    queue.add_channel(receiver, 1, 1)
        .unwrap();
    sender.send("Foo".into())
        .unwrap();
    assert_eq!(queue.next(), Some("Foo".into()));
    assert_eq!(queue.next(), None);
}
