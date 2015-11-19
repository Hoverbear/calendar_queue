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
    /// ```rust
    /// use calendar_queue::CalendarQueue;
    /// let queue = CalendarQueue::<u64, String>::new();
    /// ```
    pub fn new() -> Self {
        CalendarQueue {
            sorter: LinkedList::new(),
            flows: HashMap::new(),
            conformance_times: HashMap::new(),
        }
    }

    /// ## Errors
    ///
    /// Will return `DuplicateFlowId` if a duplicate key is found.
    ///
    /// ```rust
    /// use calendar_queue::CalendarQueue;
    /// let mut queue = CalendarQueue::<u64, String>::new();
    /// let flow = queue.create_channel(1, 1)
    ///     .unwrap();
    /// flow.send("Foo".into())
    ///     .unwrap();
    /// ```
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

    /// ## Errors
    ///
    /// Will return `DuplicateFlowId` if a duplicate key is found.
    ///
    /// ```rust
    /// use calendar_queue::CalendarQueue;
    /// let mut queue = CalendarQueue::<u64, String>::new();
    /// let (sender, receiver) = std::sync::mpsc::channel();
    /// queue.add_channel(receiver, 1, 1)
    ///     .unwrap();
    /// sender.send("Foo".into())
    ///     .unwrap();
    /// ```
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
