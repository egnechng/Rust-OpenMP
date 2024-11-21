# Rust vs. OpenMP 
A project comparing Rust and OpenMP via various benchmarks for CSCI-GA 3033 (Multicore Processors & Architecture)

## Compiling C Code (OpenMP)
``` bash
gcc -Wall -std=c99 -fopenmp -o <executable> <file_name>
```
Running individual program
```bash
./<executable> <problem size N> <number of threads t>
```

Example:

```bash
./histogram 10000000 4  # N = 10M, t = 4 threads

```

## Compiling Rust code

```bash 
cargo build --release
```

Running individual bin file
```bash
cargo run --bin <file_name> <problem size N> <number of threads t>
```

Example:

```bash
cargo run --bin histogram 10000000 4  # N = 10M, t = 4 threads
```

## Running Benchmarks

Make the script executable
```bash 
chmod +x build.sh
```

Run the script:
```bash
./run.sh
```