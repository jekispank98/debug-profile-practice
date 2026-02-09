use std::thread;
use std::time::Duration;

pub fn sum_of_squares(n: u64) -> u64 {
    let mut result: u64 = 0;
    for i in 1..n {
        result += i + i
    }
    result
}

pub fn slow_function() {
    thread::sleep(Duration::from_secs(5))
}