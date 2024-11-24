use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::env;
use std::time::Instant;

const NUM_BINS: usize = 256; // Number of bins for the histogram

fn generate_random_data(n: usize) -> Vec<u32> {
    let seed: u64 = 40; // Fixed seed
    let mut rng = StdRng::seed_from_u64(seed);
    (0..n).map(|_| rng.gen_range(0..1000)).collect()
}

fn parallel_histogram(data: &[u32], num_bins: usize) -> Vec<usize> {
    data.par_iter()
        .fold(
            || vec![0usize; num_bins],
            |mut hist, &value| {
                let bin = (value % num_bins as u32) as usize;
                hist[bin] += 1;
                hist
            },
        )
        .reduce(
            || vec![0usize; num_bins],
            |mut hist1, hist2| {
                for (i, count) in hist2.iter().enumerate() {
                    hist1[i] += count;
                }
                hist1
            },
        )
}

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!(
            "Usage: {}  <number of threads t> <problem size N>",
            args[0]
        );
        std::process::exit(1);
    }

    let num_threads: usize = args[1].parse().expect("t must be a positive integer");
    let n: usize = args[2].parse().expect("N must be a positive integer");

    if n == 0 || num_threads == 0 {
        eprintln!("Error: N and t must be greater than 0.");
        std::process::exit(1);
    }

    // Generate random data
    let data: Vec<u32> = generate_random_data(n);

    // Create a custom Rayon thread pool
    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .expect("Failed to create thread pool");

    // Execute parallel_histogram within the custom thread pool
    let start_time = Instant::now();
    let _histogram = pool.install(|| parallel_histogram(&data, NUM_BINS)); // _ for unused variable
    let duration = start_time.elapsed();

    eprintln!("Time for actual program:({:.12?})s", duration.as_secs_f64());

    /* print the histogram
    for (i, count) in histogram.iter().enumerate() {
        println!("Bin {}: {}", i, count);
    }
    */
}
