use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rayon::ThreadPoolBuilder;
use std::env;
use std::time::Instant;

fn rand_num_array(size: usize) -> Vec<i32> {
    let seed: u64 = 40; // Fixed seed
    let mut rng = StdRng::seed_from_u64(seed);
    (0..size).map(|_| rng.gen_range(0..1_000_000)).collect()
}

fn merge(left: &[i32], right: &[i32], buffer: &mut [i32]) {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    let mut left_peek = left_iter.next();
    let mut right_peek = right_iter.next();
    let mut i = 0;

    while let (Some(&l_val), Some(&r_val)) = (left_peek, right_peek) {
        if l_val <= r_val {
            buffer[i] = l_val;
            left_peek = left_iter.next();
        } else {
            buffer[i] = r_val;
            right_peek = right_iter.next();
        }
        i += 1;
    }

    while let Some(&l_val) = left_peek {
        buffer[i] = l_val;
        left_peek = left_iter.next();
        i += 1;
    }

    while let Some(&r_val) = right_peek {
        buffer[i] = r_val;
        right_peek = right_iter.next();
        i += 1;
    }
}

fn merge_sort_parallel(arr: &mut [i32], buffer: &mut [i32], depth: usize, max_depth: usize) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    let (left, right) = arr.split_at_mut(mid);
    let (left_buffer, right_buffer) = buffer.split_at_mut(mid);

    if depth < max_depth {
        // Use rayon::join to parallelize recursive calls
        rayon::join(
            || merge_sort_parallel(left, left_buffer, depth + 1, max_depth),
            || merge_sort_parallel(right, right_buffer, depth + 1, max_depth),
        );
    } else {
        // Sequential
        merge_sort_parallel(left, left_buffer, depth + 1, max_depth);
        merge_sort_parallel(right, right_buffer, depth + 1, max_depth);
    }

    // Merge the sorted halves
    merge(left, right, buffer);
    arr.copy_from_slice(buffer);
}

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} <number_of_elements> <number_of_threads>",
            args[0]
        );
        std::process::exit(1);
    }

    let size: usize = args[1]
        .parse()
        .expect("Invalid number of elements");
    let num_threads: usize = args[2]
        .parse()
        .expect("Invalid number of threads");

    // Generate random array
    let mut array = rand_num_array(size);
    let mut buffer = vec![0; size];

    let pool = ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .expect("Failed to create thread pool");

    let start_time = Instant::now();

    // Execute parallel merge sort
    pool.install(|| {
        let max_depth = 3;
        merge_sort_parallel(&mut array, &mut buffer, 0, max_depth);
    });

    let elapsed = start_time.elapsed();
    eprintln!("Elapsed time: {:.6} seconds", elapsed.as_secs_f64());

    // Verify that the array is sorted
    /*
    for i in 1..array.len() {
         if array[i - 1] > array[i] {
             eprintln!("Array is not sorted at position {}", i);
             break;
         }
     }*/
}
