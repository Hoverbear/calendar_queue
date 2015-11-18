use std::collections::{LinkedList, HashMap};
use std::sync::mpsc::{Receiver, Sender, channel};

use {FlowId, ConformanceTime};

pub struct CalendarQueue<T> {
    sorter: LinkedList<Option<FlowId>>, // TODO: Collisions?
    flows: HashMap<FlowId, Receiver<T>>,
    conformance_times: HashMap<FlowId, ConformanceTime>,
}

impl<T> CalendarQueue<T> {
    pub fn new() -> Self {
        CalendarQueue {
            sorter: LinkedList::new(),
            flows: HashMap::new(),
            conformance_times: HashMap::new(),
        }
    }
    pub fn add(&mut self, id: FlowId, conformance_time: ConformanceTime) -> Sender<T> {
        let (sender, receiver) = channel();
        self.flows.insert(id, receiver);
        self.conformance_times.insert(id, conformance_time);
        sender
    }
}
