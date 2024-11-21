#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Compile Rust code
echo "Compiling Rust code in the rust directory..."
cd rust
cargo build --release
echo "Rust compilation completed."

# Go back to the root directory
cd ..

# Compile OpenMP C code
echo "Compiling OpenMP C code in the openmp directory..."
cd openmp
make
echo "OpenMP compilation completed."

# Return to the root directory
cd ..

echo "Build process completed successfully."

# Array of executables to test (add more)
EXECUTABLES=("histogram" "merge_sort")

# Array of input sizes to test
SIZES=(100000 50000 1000000 2500000 5000000 7500000 10000000)
# Array of thread counts to test for parallel versions
THREAD_COUNTS=(2 4 8 16 32)

# Run Benchmarks
# Function to run OpenMP benchmarks
run_openmp() {
    echo "Running OpenMP benchmarks..."
    cd openmp
    for exe in "${EXECUTABLES[@]}"; do
        for size in "${SIZES[@]}"; do
            for threads in "${THREAD_COUNTS[@]}"; do
                echo "Running OpenMP program: $exe with size=$size and threads=$threads"
                ./"$exe" "$size" "$threads"
            done
        done
    done
    cd ..
}

# Function to run Rust benchmarks
run_rust() {
    echo "Running Rust benchmarks..."
    cd rust
    for exe in "${EXECUTABLES[@]}"; do
        for size in "${SIZES[@]}"; do
            for threads in "${THREAD_COUNTS[@]}"; do
                echo "Running Rust program: $exe with size=$size and threads=$threads"
                cargo run --release --bin "$exe" "$size" "$threads"
            done
        done
    done
    cd ..
}

# Run both benchmarks
run_openmp
run_rust

echo "All benchmarks completed."

# TODO: Parse times automatically and add them to a results.csv file and graph (use grep?)