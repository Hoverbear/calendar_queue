#![feature(test)]

extern crate test;
extern crate calendar_queue;
extern crate rand;

use rand::distributions::{IndependentSample, Range};
use calendar_queue::{CalendarQueue};
use test::Bencher;

const MANY_CHANNELS: u64 = 1_000;
const SOME_CHANNELS: u64 = MANY_CHANNELS / 2;

#[bench]
fn single_channel(b: &mut Bencher) {
    let mut queue = CalendarQueue::new();
    let sender = queue.create_channel(1, 1)
        .unwrap();
    sender.send("Foo".to_string())
        .unwrap();

    b.iter(|| {
        assert_eq!(queue.tick(), Some("Foo".into()));
        sender.send("Foo".into())
            .unwrap();
    });
}

#[bench]
fn alternating_dual_channel(b: &mut Bencher) {
    let mut queue = CalendarQueue::new();
    let sender_1 = queue.create_channel(1, 1)
        .unwrap();
    let sender_2 = queue.create_channel(2, 1)
        .unwrap();
    sender_1.send("Foo".to_string())
        .unwrap();
    sender_2.send("Bar".into())
        .unwrap();

    b.iter(|| {
        assert_eq!(queue.tick(), Some("Foo".into()));
        // Replenish it.
        sender_1.send("Foo".into())
            .unwrap();
        assert_eq!(queue.tick(), Some("Bar".into()));
        // Replenish it.
        sender_2.send("Bar".into())
            .unwrap();
    });
}

#[bench]
fn many_channels(b: &mut Bencher) {
    let mut queue = CalendarQueue::new();

    let between = Range::new(10, 500);
    let mut rng = rand::thread_rng();

    let channels = (0..MANY_CHANNELS).map(|i| {
        let choice = between.ind_sample(&mut rng);
        let sender = queue.create_channel(i, choice).unwrap();
        sender.send(i).unwrap();
        sender
    }).collect::<Vec<_>>();


    b.iter(|| {
        let i = queue.next().unwrap();
        channels[i as usize].send(i).unwrap()
    });
}

#[bench]
fn some_channels(b: &mut Bencher) {
    let mut queue = CalendarQueue::new();

    let between = Range::new(10, 500);
    let mut rng = rand::thread_rng();

    let channels = (0..SOME_CHANNELS).map(|i| {
        let choice = between.ind_sample(&mut rng);
        let sender = queue.create_channel(i, choice).unwrap();
        sender.send(i).unwrap();
        sender
    }).collect::<Vec<_>>();


    b.iter(|| {
        let i = queue.next().unwrap();
        channels[i as usize].send(i).unwrap()
    });
}
