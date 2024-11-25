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

# Creating Graphs
Within the `/graphing` folder, there is bash script (use `chmod +x graphing_env.sh` to make it executable).<br>
Run this to create the venv that will allow you to use the graphing modules seaborn/matplotlib/etc.<br>
Then, after running the script you can run `juypter lab` or in vscode just make sure to use the `graphing` kernel.<br>
You can then feel free to run the cells to recreate/modify the graphs. As said above the data will be drawn
from the file `saved_suite_history.json`.
