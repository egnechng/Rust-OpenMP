use rayon::prelude::*;
use std::cmp;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

unsafe fn set_unsync(first_elem: *mut i32, idx: usize, val: i32) {
    unsafe { *first_elem.add(idx) = val }
}

unsafe fn get_unsync_c(first_elem: *mut char, idx: usize) -> char {
    unsafe { *first_elem.add(idx) }
}

unsafe fn get_unsync(first_elem: *mut i32, idx: usize) -> i32 {
    unsafe { *first_elem.add(idx) }
}

#[derive(Clone, Copy)]
pub struct ThreadedRawPtr<T: ?Sized>(pub *mut T);
unsafe impl<T: ?Sized> Send for ThreadedRawPtr<T> {}
unsafe impl<T: ?Sized> Sync for ThreadedRawPtr<T> {}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        panic!("You need to enter the thread count, two input files, and print traceback flag.");
    }
    let t_count: usize = match &args[1].parse::<usize>() {
        Ok(num) => *num,
        Err(_) => panic!(),
    };
    rayon::ThreadPoolBuilder::new()
        .num_threads(t_count)
        .build_global()
        .unwrap();
    let print_scores = if &args[4] == "true" { true } else { false };

    let mut input1: Vec<char> = fs::read_to_string(&args[2])
        .expect("Error reading file 1")
        .chars()
        .collect();
    let mut input2: Vec<char> = fs::read_to_string(&args[3])
        .expect("Error reading file 1")
        .chars()
        .collect();
    let input1_len = input1.len() + 1;
    let input2_len = input2.len() + 1;

    let mut scores: Vec<i32> = vec![0; input1_len * input2_len];

    for i in 1..input2_len {
        scores[i] = -1 * (i as i32)
    }
    for i in 1..input1_len {
        scores[i * input2_len] = -1 * (i as i32);
    }
    let scores_first_elem = ThreadedRawPtr(scores.as_mut_ptr());
    let input1_first_elem = ThreadedRawPtr(input1.as_mut_ptr());
    let input2_first_elem = ThreadedRawPtr(input2.as_mut_ptr());

    let timing = Instant::now();
    for i in 1..input2_len + input1_len - 1 {
        let j_end = cmp::min(input1_len, i + 1) as usize;
        let j_start = cmp::max(1, (i as i32) - (input2_len as i32) + 2) as usize;

        (j_start..j_end)
            .into_par_iter()
            .enumerate()
            .for_each(|(_, j)| {
                let scores_first_elem = scores_first_elem;
                let input1_first_elem = input1_first_elem;
                let input2_first_elem = input2_first_elem;

                let mat_y = j;
                let mat_x = i - j + 1;

                unsafe {
                    let is_match = if get_unsync_c(input2_first_elem.0, mat_x - 1)
                        == get_unsync_c(input1_first_elem.0, mat_y - 1)
                    {
                        1
                    } else {
                        -1
                    };
                    let score_if_match = get_unsync(
                        scores_first_elem.0,
                        (mat_y - 1) * input2_len + mat_x - 1,
                    ) + is_match;
                    let dp_left =
                        get_unsync(scores_first_elem.0, mat_y * input2_len + mat_x - 1) - 1;
                    let dp_top =
                        get_unsync(scores_first_elem.0, (mat_y - 1) * input2_len + mat_x) - 1;

                    let mut dp_max = cmp::max(dp_left, dp_top);
                    dp_max = cmp::max(score_if_match, dp_max);

                    set_unsync(
                        scores_first_elem.0,
                        mat_y * input2_len + mat_x,
                        dp_max,
                    )
                }
            });
    }
    println!("Time to execute: {:.4?}", timing.elapsed());

    let actual_score = scores[(input1_len - 1) * input2_len + (input2_len - 1)];
    print!("Actual best score: {}\n", actual_score);

    if print_scores {
        let mut output = File::create("needle_out.txt").unwrap();

        for i in 0..input1_len {
            for j in 0..input2_len {
                write!(output, "{} ", scores[i * input2_len + j]).unwrap();
            }
            write!(output, "\n").unwrap();
        }
    }
}
