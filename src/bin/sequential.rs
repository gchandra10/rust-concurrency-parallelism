use std::time::Instant;

fn main() {
    let start = Instant::now();

    // Calculate Fibonacci numbers sequentially
    let result1 = fib(30);
    let result2 = fib(30);
    let result3 = fib(30);

    let duration = start.elapsed();
    println!("\n\n Sequential execution took: {:?}", duration);
    println!("\n Results: {}, {}, {}\n", result1, result2, result3);
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
