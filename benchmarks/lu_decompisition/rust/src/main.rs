use rand::Rng;
use std::time::Instant;
use rayon::prelude::*;

// Define a constant size for the matrices (this must be known at compile-time)
const SIZE: usize = 200;

// Function to perform LU decomposition
fn l_u_d(a: &mut [[f32; SIZE]; SIZE], l: &mut [[f32; SIZE]; SIZE], u: &mut [[f32; SIZE]; SIZE], size: usize) {
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
fn initialize_matrices() -> ([[f32; SIZE]; SIZE], [[f32; SIZE]; SIZE], [[f32; SIZE]; SIZE]) {
    // Initialize matrices with 0s
    let a = [[0.0; SIZE]; SIZE];
    let l = [[0.0; SIZE]; SIZE];
    let u = [[0.0; SIZE]; SIZE];
    (a, l, u)
}

// Function to fill matrix with random values and ensure diagonal dominance
fn random_fill(matrix: &mut [[f32; SIZE]; SIZE], size: usize) {
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
fn print_matrix(matrix: &[[f32; SIZE]; SIZE], size: usize) {
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

    rayon::ThreadPoolBuilder::new().num_threads(num_threads).build_global().unwrap();

    // Initialize matrices
    let (mut a, mut l, mut u) = initialize_matrices();

    // Seed random number generator
    rand::thread_rng().gen::<u32>(); // To ensure random state initialization

    // Fill matrix A with random values
    println!("Generating random values for A matrix...");
    random_fill(&mut a, SIZE);

    // Print original matrix A
    println!("A Matrix:");
    print_matrix(&a, SIZE);

    // Perform LU Decomposition
    let start_time = Instant::now();
    l_u_d(&mut a, &mut l, &mut u, SIZE);
    let duration = start_time.elapsed();

    // Print L and U matrices
    println!("L Matrix:");
    print_matrix(&l, SIZE);
    println!("U Matrix:");
    print_matrix(&u, SIZE);

    // Print runtime
    println!("LU Decomposition Time: {:.6} seconds", duration.as_secs_f64());
}
