use std::collections::{LinkedList, HashMap};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::hash::Hash;

use {ConformanceTime, Result, Error};

pub struct CalendarQueue<I, T>
where I: Hash + Eq + Copy {
    sorter: LinkedList<Option<I>>, // TODO: Collisions?
    flows: HashMap<I, Receiver<T>>,
    conformance_times: HashMap<I, ConformanceTime>,
}

impl<I, T> CalendarQueue<I, T>
where I: Hash + Eq + Copy {
    pub fn new() -> Self {
        CalendarQueue {
            sorter: LinkedList::new(),
            flows: HashMap::new(),
            conformance_times: HashMap::new(),
        }
    }
    pub fn create_channel(&mut self, id: I, conformance_time: ConformanceTime) -> Result<Sender<T>> {
        if self.flows.contains_key(&id) {
            Err(Error::DuplicateFlowId)
        } else {
            let (sender, receiver) = channel();
            self.flows.insert(id, receiver);
            self.conformance_times.insert(id, conformance_time);
            Ok(sender)
        }
    }

    pub fn add_channel(&mut self, channel: Receiver<T>, id: I, conformance_time: ConformanceTime) -> Result<()> {
        if self.flows.contains_key(&id) {
            Err(Error::DuplicateFlowId)
        } else {
            self.flows.insert(id, channel);
            self.conformance_times.insert(id, conformance_time);
            Ok(())
        }
    }
}
