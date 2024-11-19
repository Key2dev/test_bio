use std::thread;
use std::time;

use rand::Rng;

fn fake_temp_read_range(min_range: f64, max_range: f64) -> f64 {
    // Generating a random value from range min_range..=max_range in .2 (f64!!)
    // Placeholder for temp read.
    let mut rng = rand::thread_rng();
    let random_value = rng.gen_range(min_range..=max_range);

    // Round to 2 decimal places and return
    (random_value * 100.0).round() / 100.0
}

fn pump_on(duration_in_sec: u64) {
    // Mock implementation, TODO
    println!("Pump is on, duration: {duration_in_sec}");
    thread::sleep(time::Duration::from_secs(duration_in_sec));
    println!("Pump is off, duration: {duration_in_sec}");
    todo!();
}

fn main() {
    println!("Hello, world!");
    let min: f64 = 20.0;
    let max: f64 = 30.0;
    for _ in 0..40 {
        let val = fake_temp_read_range(min, max);
        println!("Temperature is: {val}");

        thread::sleep(time::Duration::from_millis(200));
    }
    pump_on(5);
}
