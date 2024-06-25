use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    // Calculate Fibonacci numbers in parallel
    let results: Vec<u32> = (0..3).into_par_iter().map(|_| fib(30)).collect();

    let duration = start.elapsed();
    println!("\n\n Parallel execution took: {:?}", duration);
    println!("\nResults: {:?}\n", results);
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
