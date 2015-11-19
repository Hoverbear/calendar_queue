#![crate_name = "calendar_queue"]
#![crate_type="lib"]
#![doc(html_logo_url = "")]
#![doc(html_root_url = "https://hoverbear.github.io/calendar_queue/calendar_queue/")]

//! This crate implements the idea of a "Calendar Queue Scheduler" data structure.
//!
//! This is developed currently as part of an academic project, so emphasis is placed on getting
//! it working for now. If there is sufficient interest (seriously, email me) I will maintain and
//! extend this.

mod calendar_queue;

pub use calendar_queue::CalendarQueue;

pub type ConformanceTime = u64;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DuplicateFlowId,
}
