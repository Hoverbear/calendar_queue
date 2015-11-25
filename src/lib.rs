#![crate_name = "calendar_queue"]
#![crate_type="lib"]
#![doc(html_logo_url = "")]
#![doc(html_root_url = "https://hoverbear.github.io/calendar_queue/calendar_queue/")]

//! This crate implements the idea of a "Calendar Queue Scheduler" data structure.
//!
//! The Calendar Queue Sheduler is an effective way to fairly receive
//! from a number of `mpsc::Receiver`s. This implementation is intentionally very generi, practical
//! and simple.
//!
//! ```text
//! TODO ideas for PR's or future fun:
//! [ ] Handle overflows (easy first PR!)
//! [ ] Implement a variety of fake interfaces for simulations? (Read? TcpSocket?)
//! [ ] Optimize `next()` for production so it don't just call `tick()` repeatedly.
//! ```
//!
//! You can imagine it like so:
//!
//! ```text
//!     (The Sorter)              (The flows)
//!         +
//!          |
//!          |
//!        +-v-+
//!        |   |
//!        | ∅ |
//!        |   |
//!        +-+-+
//!          |
//! +---+  +-+-+
//! |   |  |   |
//! | 2 +--+ 3 |
//! |   |  |   |
//! +---+  +-+-+        |              |              |
//!          |          |              |              |
//!        +-+-+  +-----v-----+  +-----v-----+  +-----v-----+
//!        |   |  |           |  |           |  |           |
//!        | 1 |  | Channel 1 |  | Channel 2 |  | Channel 3 |
//!        |   |  |           |  |           |  |           |
//!        +-+-+  +-----------+  +-----------+  +-----------+
//!          |
//!          |
//!          v
//! ```
//!
//! The sorter, on the left, maintains a list of keys matching to channels. At each 'tick' of the
//! calendar it may or may not have one or many channels scheduled. Using the data structure this
//! way allows for simulations, since one can effectively simulate something similar to a router.
//!
//! The queue also implements the `Iterator` trait, and can be used as a generic iterator
//! throughout your code however you see fit. An iterator over the queue will simple repeatedly
//! call tick until it has carried out an entire "cycle" and (still fairly) given every channel a
//! chance to send, **only then** will the iterator finally exhaust.
//!
//! ### An Example
//!
//! ```rust
//! use calendar_queue::CalendarQueue;
//!
//! // `id`s and `value`s are generic. Think of this as a hashmap of channels.
//! let mut queue = CalendarQueue::<u64, String>::new();
//!
//! // Each `Reciever` is one channel to the queue.
//! // Here we give our reciever to the queue, with a cycle of 3 ticks.
//! let (sender_1, receiver_1) = std::sync::mpsc::channel();
//! queue.add_channel(receiver_1, 1, 3)
//!     .unwrap();
//! // We can also just pull one out.
//! let sender_2 = queue.create_channel(2, 5)
//!     .unwrap();
//!
//! // The `Sender`s behave as you might expect.
//! for _ in 0..3 {
//!     sender_1.send("Foo".into())
//!         .unwrap();
//! }
//!
//! for _ in 0..5 {
//!     sender_2.send("Bar".into())
//!         .unwrap();
//! }
//!
//! // The zero-th clock tick has two items!
//! assert_eq!(queue.tick(), Some("Foo".into()));
//! assert_eq!(queue.tick(), Some("Bar".into()));
//! // First tick has none!
//! assert_eq!(queue.tick(), None);
//! // Second tick has none either!
//! assert_eq!(queue.tick(), None);
//! // The third tick will be "Foo" because of it's cycle time.
//! assert_eq!(queue.tick(), Some("Foo".into()));
//! // We can use `.next()` (and other iterator goodies) to fast forward over empty gaps.
//! // This will tick over 4 and move on to 5.
//! assert_eq!(queue.next(), Some("Bar".into()));
//! // Tick 6 now.
//! assert_eq!(queue.tick(), Some("Foo".into()));
//! ```

mod calendar_queue;

pub use calendar_queue::CalendarQueue;

pub type ConformanceTicks = u64;
pub type ClockTick = u64;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DuplicateFlowId,
}
