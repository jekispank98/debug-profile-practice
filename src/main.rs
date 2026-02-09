use crate::calculation::{slow_function, sum_of_squares};

pub mod calculation;

fn main() {
    println!("Hello");
    let result_for_10 = sum_of_squares(10);
    println!("Hello, result for 10: {result_for_10}");
    let result_for_100 = sum_of_squares(100);
    println!("Hello, result for 100: {result_for_100}");
    slow_function();
    let result_for_1000 = sum_of_squares(1000);
    println!("Hello, result for 1000: {result_for_1000}");
}
