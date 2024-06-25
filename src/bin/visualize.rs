use rayon::prelude::*;
// use plotters::prelude::*;
use std::time::Instant;

fn main() {
    // Measure execution times for the three methods
    let seq_duration = measure_sequential();
    let concurrent_duration = measure_concurrent();
    let par_duration = measure_parallel();

    // Prepare data for plotting
    let times = vec![
        ("Sequential", seq_duration.as_secs_f64()),
        ("Concurrent", concurrent_duration.as_secs_f64()),
        ("Parallel", par_duration.as_secs_f64()),
    ];

    println!("{:?}", times);

    // Create the chart
    // create_chart(&times).expect("Unable to create chart");
}

fn measure_sequential() -> std::time::Duration {
    let start = Instant::now();
    let _ = fib(30);
    let _ = fib(30);
    let _ = fib(30);
    start.elapsed()
}

fn measure_concurrent() -> std::time::Duration {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let start = Instant::now();
    rt.block_on(async {
        let task1 = tokio::spawn(async { fib(30) });
        let task2 = tokio::spawn(async { fib(30) });
        let task3 = tokio::spawn(async { fib(30) });
        let _ = tokio::join!(task1, task2, task3);
    });
    start.elapsed()
}

fn measure_parallel() -> std::time::Duration {
    let start = Instant::now();
    let _: Vec<u32> = (0..3).into_par_iter().map(|_| fib(30)).collect();
    start.elapsed()
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

// fn create_chart(times: &Vec<(&str, f64)>) -> Result<(), Box<dyn std::error::Error>> {
//     let root_area = BitMapBackend::new("execution_times.png", (600, 400)).into_drawing_area();
//     root_area.fill(&WHITE)?;

//     let max_time = times.iter().map(|&(_, time)| time).fold(0.0, f64::max);

//     let mut chart = ChartBuilder::on(&root_area)
//         .caption("Execution Times for Fibonacci Calculation", ("sans-serif", 20))
//         .margin(10)
//         .x_label_area_size(30)
//         .y_label_area_size(30)
//         .build_cartesian_2d(
//             times.iter().map(|&(label, _)| label).collect::<Vec<&str>>().into_segmented(),
//             0.0..max_time,
//         )?;

//     chart.configure_mesh().draw()?;

//     chart.draw_series(times.iter().map(|&(label, time)| {
//         let x = SegmentValue::CenterOf(label);
//         let y0 = 0.0;
//         let y1 = time;
//         let bar = Rectangle::new([(x, y0), (x, y1)], BLUE.filled());
//         bar
//     }))?;

//     root_area.present()?;
//     Ok(())
// }
