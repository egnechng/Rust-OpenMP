use rand::Rng;
use std::time::Instant;
use rayon::prelude::*;

// Define a constant size for the matrices (this must be known at compile-time)
//c

// Function to perform LU decomposition
fn l_u_d(a: &mut Vec<Vec<f32>>, l: &mut Vec<Vec<f32>>, u: &mut Vec<Vec<f32>>, size: usize) {
    let mut buffer = vec![0f32; size];
    for i in 0..(size-1) {
        (&mut buffer[1..size]).par_iter_mut().enumerate().for_each(|(j, row_val)| {
            if j < i {
                *row_val = 0.0;
            } else {
                let mut sum = 0f32;
                
                for k in 0..i {
                    sum = sum + l[i][k]*u[k][j];
                }
                *row_val = a[j][i] - sum;
            }
        });

        (&mut buffer[1..size]).par_iter_mut().enumerate().for_each(|(j,row_val)| {
            if j < i {
                *row_val = 0.0;
            } else if j == i {
                *row_val = 1.0;
            } else {
                let mut sum = 0f32;

                for k in 0..i {
                    sum = sum+ (l[i][k] * u[k][j]) / l[i][i];
                }

                *row_val = a[i][j] / l[i][i] - sum;
            }
        });
    }
}

// Function to initialize matrices (allocate memory)
fn initialize_matrices(size: usize) -> (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<Vec<f32>>) {
    // Initialize matrices with 0s
    let a = vec![vec![0.0; size]; size];
    let l = vec![vec![0.0; size]; size];
    let u = vec![vec![0.0; size]; size];
    (a, l, u)
}

// Function to fill matrix with random values and ensure diagonal dominance
fn random_fill(matrix: &mut Vec<Vec<f32>>, size: usize) {
    let mut rng = rand::thread_rng();
    for i in 0..size {
        let mut sum = 0.0;
        for j in 0..size {
            matrix[i][j] = rng.gen_range(1.0..10.0);  // Random value between 1 and 10
            sum += matrix[i][j].abs();
        }
        matrix[i][i] = sum + rng.gen_range(1.0..6.0);  // Make the diagonal dominant
    }
}

// Function to print matrix
fn print_matrix(matrix: &Vec<Vec<f32>>, size: usize) {
    for i in 0..size {
        for j in 0..size {
            print!("{:8.2} ", matrix[i][j]);
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let threads = &args[1].to_string();
    let num_threads = threads.parse::<i32>().unwrap() as usize;

    let size = args[2].parse::<usize>().unwrap();

    rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();

    // Initialize matrices
    let (mut a, mut l, mut u) = initialize_matrices(size);

    // Seed random number generator
    rand::thread_rng().gen::<u32>(); // To ensure random state initialization

    // Fill matrix A with random values
    println!("Generating random values for A matrix...");
    random_fill(&mut a, size);

    // Print original matrix A
    println!("A Matrix:");
    // print_matrix(&a, size);

    // Perform LU Decomposition
    let start_time = Instant::now();
    l_u_d(&mut a, &mut l, &mut u, size);
    let duration = start_time.elapsed();

    // Print L and U matrices
    println!("L Matrix:");
    //(&l, size);
    println!("U Matrix:");
    //print_matrix(&u, size);

    // Print runtime
    eprintln!("Time for actual program:({:.12?})s", duration.as_secs_f64());
}
