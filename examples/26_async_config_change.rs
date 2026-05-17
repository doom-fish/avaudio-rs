#[path = "support/mod.rs"]
mod support;

use avaudio::async_api::ConfigChangeStream;
use avaudio::prelude::*;
use support::print_skip;

fn main() {
    let engine = match AudioEngine::new() {
        Ok(engine) => engine,
        Err(error) => {
            print_skip(&format!("engine unavailable: {error}"));
            return;
        }
    };

    let stream = ConfigChangeStream::subscribe(&engine, 8);
    println!("buffered before event: {}", stream.buffered_count());
    println!("buffered after subscribe: {}", stream.buffered_count());
    drop(stream);
    println!("✅ async config change stream OK");
}
