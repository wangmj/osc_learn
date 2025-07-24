use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let delay_time=Duration::from_secs(10);
    let start_time = Instant::now();
    while (Instant::now() - start_time).lt(&delay_time) {
        thread::sleep(Duration::from_secs(1));
    }
    println!("Hello, world!");
}
