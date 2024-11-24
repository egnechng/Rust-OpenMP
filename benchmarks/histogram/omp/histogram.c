#include <stdio.h>
#include <stdlib.h>
#include <omp.h>

#define NUM_BINS 256  // Number of bins for the histogram

int* generate_random_data(int n, unsigned int seed) {
    int* data = (int*)malloc(n * sizeof(int));
    if (data == NULL) {
        fprintf(stderr, "Error: Memory allocation failed for data array.\n");
        exit(EXIT_FAILURE);
    }
    srand(seed);
    for (int i = 0; i < n; i++) {
        data[i] = rand() % 1000;
    }
    return data;
}

void parallel_histogram(int *data, int n, int *histogram, int num_threads) {
    // Allocate memory for per-thread local histograms
    int **local_hists = (int **)malloc(num_threads * sizeof(int *));
    if (local_hists == NULL) {
        fprintf(stderr, "Error: Memory allocation failed for local histograms.\n");
        exit(EXIT_FAILURE);
    }
    for (int t = 0; t < num_threads; t++) {
        local_hists[t] = (int *)calloc(NUM_BINS, sizeof(int));
        if (local_hists[t] == NULL) {
            fprintf(stderr, "Error: Memory allocation failed for local histogram %d.\n", t);
            exit(EXIT_FAILURE);
        }
    }

    #pragma omp parallel
    {
        int tid = omp_get_thread_num();
        // Each thread updates its own local histogram
        #pragma omp for nowait
        for (int i = 0; i < n; i++) {
            int bin = data[i] % NUM_BINS;
            local_hists[tid][bin]++;
        }
    }

    // Merge local histograms into the global histogram
    for (int t = 0; t < num_threads; t++) {
        for (int i = 0; i < NUM_BINS; i++) {
            histogram[i] += local_hists[t][i];
        }
    }

    // Free memory
    for (int t = 0; t < num_threads; t++) {
        free(local_hists[t]);
    }
    free(local_hists);
}

int main(int argc, char *argv[]) {
    if (argc != 3) {
        fprintf(stderr, "Usage: %s <number of threads t> <problem size N>\n", argv[0]);
        return EXIT_FAILURE;
    }

    // Parse command-line arguments
    int num_threads = atoi(argv[1]);
    int n = atoi(argv[2]); 

    omp_set_num_threads(num_threads);
    int seed = 40; // Fixed seed
    int *data = generate_random_data(n, seed);

    // Initialize global histogram to zero
    int *histogram = (int *)calloc(NUM_BINS, sizeof(int));
    if (histogram == NULL) {
        fprintf(stderr, "Error: Memory allocation failed for histogram.\n");
        free(data);
        exit(EXIT_FAILURE);
    }

    double start_time = omp_get_wtime();
    parallel_histogram(data, n, histogram, num_threads);
    double end_time = omp_get_wtime();

    fprintf(stderr,"Time for actual program:(%.12f)s\n", end_time - start_time);

    free(data);
    free(histogram);

    return 0;
}
