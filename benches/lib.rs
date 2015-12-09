#![feature(test)]

extern crate test;
extern crate calendar_queue;
extern crate rand;

use rand::distributions::{IndependentSample, Range};
use calendar_queue::{CalendarQueue};
use test::Bencher;

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
fn colliding_dual_channel(b: &mut Bencher) {
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
fn noncolliding_dual_channel(b: &mut Bencher) {
    let mut queue = CalendarQueue::new();
    let sender_1 = queue.create_channel(1, 2)
        .unwrap();
    let _ = queue.tick(); // Cause an offset.
    let sender_2 = queue.create_channel(2, 2)
        .unwrap();
    sender_1.send("Foo".to_string())
        .unwrap();
    sender_2.send("Bar".into())
        .unwrap();

    b.iter(|| {
        assert_eq!(queue.tick(), Some("Bar".into()));
        // Replenish it.
        sender_2.send("Bar".into())
            .unwrap();
        assert_eq!(queue.tick(), Some("Foo".into()));
        // Replenish it.
        sender_1.send("Foo".into())
            .unwrap();
    });
}

fn multi_rand_channels(channels: usize, between: Range<usize>, b: &mut Bencher) {
    let mut queue = CalendarQueue::new();

    let mut rng = rand::thread_rng();

    let channels = (0..channels).map(|i| {
        let choice = between.ind_sample(&mut rng);
        let sender = queue.create_channel(i, choice as u64).unwrap();
        sender.send(i).unwrap();
        sender
    }).collect::<Vec<_>>();


    b.iter(|| {
        let i = queue.next().unwrap();
        channels[i as usize].send(i).unwrap()
    });
}

#[bench]
fn multi_10_000_channels(b: &mut Bencher) {
    let channels = 10_000;
    multi_rand_channels(channels, Range::new(10, channels/2), b);
}

#[bench]
fn multi_5_000_channels(b: &mut Bencher) {
    let channels = 5_000;
    multi_rand_channels(channels, Range::new(10, channels/2), b);
}

#[bench]
fn multi_1_000_channels(b: &mut Bencher) {
    let channels = 1_000;
    multi_rand_channels(channels, Range::new(10, channels/2), b);
}

#[bench]
fn multi_500_channels(b: &mut Bencher) {
    let channels = 500;
    multi_rand_channels(channels, Range::new(10, channels/2), b);
}

#[bench]
fn multi_100_channels(b: &mut Bencher) {
    let channels = 100;
    multi_rand_channels(channels, Range::new(10, channels/2), b);
}

#[bench]
fn multi_50_channels(b: &mut Bencher) {
    let channels = 50;
    multi_rand_channels(channels, Range::new(10, channels/2), b);
}
