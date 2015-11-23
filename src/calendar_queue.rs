use std::collections::{VecDeque, HashMap};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::hash::Hash;

use {Result, Error, ClockTick, ConformanceTicks};

pub struct CalendarQueue<I, T>
where I: Hash + Eq + Copy {
    sorter: VecDeque<(ClockTick, VecDeque<I>)>,
    flows: HashMap<I, Receiver<T>>,
    conformance_times: HashMap<I, ConformanceTicks>,
    clock: ClockTick,
}

impl<I, T> CalendarQueue<I, T>
where I: Hash + Eq + Copy {
    /// ```rust
    /// use calendar_queue::CalendarQueue;
    /// let queue = CalendarQueue::<u64, String>::new();
    /// ```
    pub fn new() -> Self {
        CalendarQueue {
            sorter: VecDeque::new(),
            flows: HashMap::new(),
            conformance_times: HashMap::new(),
            clock: 0,
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
    pub fn create_channel(&mut self, id: I, conformance_ticks: ConformanceTicks) -> Result<Sender<T>> {
        if self.flows.contains_key(&id) {
            Err(Error::DuplicateFlowId)
        } else {
            let (sender, receiver) = channel();
            self.flows.insert(id, receiver);
            self.conformance_times.insert(id, conformance_ticks);
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
    pub fn add_channel(&mut self, channel: Receiver<T>, id: I, conformance_ticks: ConformanceTicks) -> Result<()> {
        if self.flows.contains_key(&id) {
            Err(Error::DuplicateFlowId)
        } else {
            self.flows.insert(id, channel);
            self.conformance_times.insert(id, conformance_ticks);
            let clock_time = self.clock;
            self.schedule_flow(id, clock_time);
            Ok(())
        }
    }

    fn schedule_flow(&mut self, id: I, target_tick: ClockTick) {
        // Determine which action to take.
        let action = {
            let current_clock = self.clock;
            // Get into position.
            let mut scanner = self.sorter.iter()
                .enumerate().take_while(|&(_, &(target, _))| target < current_clock);
            match scanner.next() {
                Some((index, &(slot_tick, _))) => {
                    if slot_tick > target_tick {
                        SorterAction::Insert(index)
                    } else if slot_tick == target_tick {
                        SorterAction::Modify(index)
                    } else {
                        unreachable!();
                    }
                },
                None => {
                    SorterAction::Append
                }
            }
        };
        // Take the action.
        match action {
            SorterAction::Insert(index) => {
                let mut slots = VecDeque::new();
                slots.push_back(id);
                self.sorter.insert(index, (target_tick, slots));
            },
            SorterAction::Modify(index) => {
                self.sorter.get_mut(index).unwrap().1.push_back(id);
            },
            SorterAction::Append => {
                let mut slots = VecDeque::new();
                slots.push_back(id);
                self.sorter.push_back((target_tick, slots))
            }
        }
    }

    /// ```rust
    /// use calendar_queue::CalendarQueue;
    /// let mut queue = CalendarQueue::<u64, String>::new();
    /// let (sender, receiver) = std::sync::mpsc::channel();
    /// queue.add_channel(receiver, 1, 1)
    ///     .unwrap();
    /// sender.send("Foo".into())
    ///     .unwrap();
    /// assert_eq!(queue.tick(), Some("Foo".into()));
    /// assert_eq!(queue.tick(), None);
    /// ```
    pub fn tick(&mut self) -> Option<T> {
        let maybe = self.sorter.pop_front();
        match maybe {
            Some((clock, mut slots)) => {
                let id = match slots.pop_front() {
                    Some(id) => id,
                    None => unreachable!(),
                };
                // Re-push the sorter slot if needed.
                if slots.len() > 0 {
                    self.sorter.push_front((clock, slots));
                }
                // Reschedule.
                let next_time = self.conformance_times.get(&id).unwrap().clone();
                self.schedule_flow(id, clock + next_time);
                // Get the next item.
                match self.flows.get(&id) {
                    Some(flow) => flow.try_recv().ok(),
                    None => unreachable!(),
                }
            },
            None => None,
        }
    }
}

impl<I, T> Iterator for CalendarQueue<I, T>
where I: Hash + Eq + Copy {
    type Item = T;

    /// ```rust
    /// use calendar_queue::CalendarQueue;
    /// let mut queue = CalendarQueue::<u64, String>::new();
    /// let (sender, receiver) = std::sync::mpsc::channel();
    /// queue.add_channel(receiver, 1, 1)
    ///     .unwrap();
    /// sender.send("Foo".into())
    ///     .unwrap();
    /// assert_eq!(queue.next(), Some("Foo".into()));
    /// assert_eq!(queue.next(), None);
    /// ```
    fn next(&mut self) -> Option<T> {
        let mut tries = 0;
        let limit = self.flows.len();
        while tries < limit {
            match self.tick() {
                Some(item) => return Some(item),
                None => { tries += 1; continue },
            }
        }
        None
    }
}

enum SorterAction {
    Insert(usize),
    Modify(usize),
    Append,
}
