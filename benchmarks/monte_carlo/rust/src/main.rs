use rayon::prelude::*;
use std::env;
use std::time::Instant;
use rand::distributions::Uniform;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    if  args.len() != 3 {
        panic!("You need to enter the thread count and trial count for simulation.");

    }
    let t_count: usize = match &args[1].parse::<usize>() {
        Ok(num) => *num,
        Err(_) => panic!(),
    };
    rayon::ThreadPoolBuilder::new().num_threads(t_count).build_global().unwrap();
    let trial_count: u128 = match &args[2].parse::<u128>() {
        Ok(num) => *num,
        Err(_) => panic!(),
    };

    let radius = 1.000;
    let radius_squared = radius * radius;
    let distribution: Uniform<f64> = Uniform::new(-radius, radius);

    let timing = Instant::now();

    let circle_count: u128 = (0..trial_count).into_par_iter().map_init(rand::thread_rng,|rng, _ | {
        let coord_1 = rng.sample(distribution);
        let coord_2 = rng.sample(distribution);

        let mut landed = coord_1 * coord_1;
        landed += coord_2 * coord_2;
        if landed <= radius_squared {1}else {0}
    }).reduce(|| 0, |a, b| a + b);

    println!("Time for actual program: {:.12?}", timing.elapsed().as_secs_f64());

    let pi_estimate = 4.0 * (circle_count as f64 / trial_count as f64);
    
    println!("\n {} trials, pi is {} \n",trial_count, pi_estimate);
}

