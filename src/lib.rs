mod calendar_queue;

pub use calendar_queue::CalendarQueue;

pub type FlowId = u64;
pub type ConformanceTime = u64;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DuplicateFlowId(FlowId),
}
