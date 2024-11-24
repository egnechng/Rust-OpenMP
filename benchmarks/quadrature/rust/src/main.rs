use rayon::prelude::*;
use std::f64::consts::PI;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

// Function for the given mathematical function f(x)
fn f(x: f64) -> f64 {
    50.0 / (PI * (2500.0 * x * x + 1.0))
}

// Function to print the timestamp
fn timestamp() {
    let now = SystemTime::now();
    let datetime = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let secs = datetime % 60;
    let mins = (datetime / 60) % 60;
    let hours = (datetime / 3600) % 24;
    println!("{:02}:{:02}:{:02}", hours, mins, secs);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let threads = &args[1].to_string();
    let num_threads = threads.parse::<i32>().unwrap() as usize;

    let n_amount_string = &args[2].to_string();
    let n = n_amount_string.parse::<i32>().unwrap() as usize;

    rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();
    
    let a = 0.0;
    let b = 10.0;
    let exact = 0.49936338107645674464;
    // let n = 10_000_000;
    
    println!("\nEstimate the integral of f(x) from A to B.");
    println!("f(x) = 50 / (π * (2500 * x² + 1)).\n");
    println!("A = {}", a);
    println!("B = {}", b);
    println!("N = {}", n);
    println!("Exact = {:.16}", exact);

    let start_time = Instant::now();

    // Parallel iteration using Rayon
    let total: f64 = (0..n).into_par_iter().map(|i| {
        let x = ((n - i - 1) as f64 * a + (i as f64) * b) / (n as f64 - 1.0);
        f(x)
    }).sum();

    let elapsed_time = start_time.elapsed().as_secs_f64();

    // Compute the integral estimate
    let total = (b - a) * total / (n as f64);
    let error = (total - exact).abs();

    println!("\nEstimate = {:.16}", total);
    println!("Error = {}", error);
    eprintln!("Time for actual program:({:.12?})s", elapsed_time);

    println!("\nQUAD_OPENMP:");
    println!("Normal end of execution.\n");

    // Print the timestamp
    timestamp();
}
