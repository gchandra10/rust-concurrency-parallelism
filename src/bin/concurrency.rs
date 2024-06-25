use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    // Calculate Fibonacci numbers concurrently
    let task1 = tokio::spawn(async { fib(30) });
    let task2 = tokio::spawn(async { fib(30) });
    let task3 = tokio::spawn(async { fib(30) });

    let result1 = task1.await.unwrap();
    let result2 = task2.await.unwrap();
    let result3 = task3.await.unwrap();

    let duration = start.elapsed();
    println!("\n\n Concurrent execution took: {:?}", duration);
    println!("\n Results: {}, {}, {} \n", result1, result2, result3);
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}
