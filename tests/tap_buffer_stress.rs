#![cfg(feature = "async")]

use std::thread;
use std::time::{Duration, Instant};

use avaudio::async_api::TapBufferEvent;
use doom_fish_utils::spsc::SpscRing;

const TEST_DURATION: Duration = Duration::from_secs(5);
const CALLBACK_INTERVAL: Duration = Duration::from_millis(10);
const TICKS: u32 = 500;
const EVENTS_PER_TICK: u32 = 10;
const FRAMES_PER_EVENT: u32 = 48;
const SAMPLE_RATE: f64 = 48_000.0;
const RING_CAPACITY: usize = 512;

#[test]
fn tap_buffer_ring_handles_render_rate_without_hanging() {
    let (producer, consumer) = SpscRing::<TapBufferEvent, RING_CAPACITY>::new();
    let producer_thread = thread::spawn(move || {
        let start = Instant::now();
        for tick in 0..TICKS {
            for _ in 0..EVENTS_PER_TICK {
                let _ = producer.push_overwrite(TapBufferEvent {
                    frame_length: FRAMES_PER_EVENT,
                    channel_count: 2,
                    sample_rate: SAMPLE_RATE,
                });
            }

            let next_tick = start
                + CALLBACK_INTERVAL
                    .checked_mul(tick + 1)
                    .expect("tap-buffer test tick should fit in a Duration");
            let now = Instant::now();
            if next_tick > now {
                thread::sleep(next_tick - now);
            }
        }
    });

    let consumer_start = Instant::now();
    let mut received_events = 0_u64;
    let mut received_frames = 0_u64;

    pollster::block_on(async {
        while let Some(event) = consumer.pop_async().await {
            assert_eq!(event.channel_count, 2);
            assert!((event.sample_rate - SAMPLE_RATE).abs() < f64::EPSILON);
            received_events += 1;
            received_frames += u64::from(event.frame_length);
        }
    });

    producer_thread.join().unwrap();

    let expected_events = u64::from(TICKS) * u64::from(EVENTS_PER_TICK);
    let expected_frames = expected_events * u64::from(FRAMES_PER_EVENT);
    assert_eq!(received_events, expected_events);
    assert_eq!(received_frames, expected_frames);
    assert!(
        consumer_start.elapsed() <= TEST_DURATION + Duration::from_secs(2),
        "tap-buffer stress test consumer took too long to drain"
    );
}
