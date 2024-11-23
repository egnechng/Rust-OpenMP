#include <stdio.h>
#include <stdlib.h>
#include <omp.h>

int* randNumArray(const int size, const int seed) {
    srand(seed);
    int* array = (int*)malloc(size * sizeof(int));
    if (array == NULL) {
        fprintf(stderr, "Memory allocation failed.\n");
        exit(-1);
    }
    for (int i = 0; i < size; i++) {
        array[i] = rand() % 1000000;
    }
    return array;
}

void merge(int arr[], int l, int m, int r) {
    int i, j, k;
    int n1 = m - l + 1;
    int n2 = r - m;

    // Create temporary arrays
    int* L = (int*)malloc(n1 * sizeof(int));
    int* R = (int*)malloc(n2 * sizeof(int));
    if (L == NULL || R == NULL) {
        fprintf(stderr, "Memory allocation failed.\n");
        exit(-1);
    }

    // Copy data to temporary arrays L[] and R[]
    for (i = 0; i < n1; i++)
        L[i] = arr[l + i];
    for (j = 0; j < n2; j++)
        R[j] = arr[m + 1 + j];

    // Merge the temporary arrays back into arr[l..r]
    i = 0; j = 0; k = l;
    while (i < n1 && j < n2) {
        if (L[i] <= R[j]) {
            arr[k++] = L[i++];
        } else {
            arr[k++] = R[j++];
        }
    }

    // Copy the remaining elements of L[]
    while (i < n1) {
        arr[k++] = L[i++];
    }

    // Copy the remaining elements of R[]
    while (j < n2) {
        arr[k++] = R[j++];
    }

    free(L);
    free(R);
}

void mergeSortParallel(int arr[], int l, int r, int depth) {
    if (l < r) {
        int m = l + (r - l) / 2;

        // Create parallel omp TASKS
        #pragma omp task shared(arr) if(depth < 3)
        mergeSortParallel(arr, l, m, depth + 1);

        #pragma omp task shared(arr) if(depth < 3)
        mergeSortParallel(arr, m + 1, r, depth + 1);

        #pragma omp taskwait
        merge(arr, l, m, r);
    }
}

int main(int argc, char** argv) {
    int* array;
    int size, seed, numThreads;

    if (argc < 3) {
        fprintf(stderr, "Usage: %s [number of elements] [number of threads]\n", argv[0]);
        exit(-1);
    }

    size = atoi(argv[1]);
    numThreads = atoi(argv[2]);
    seed = 40;

    array = randNumArray(size, seed);

    omp_set_num_threads(numThreads);

    double start_time = omp_get_wtime();

    // Parallel Region
    #pragma omp parallel
    {
        // Only one thread initiates the merge sort
        #pragma omp single nowait
        mergeSortParallel(array, 0, size - 1, 0);
    }

    double end_time = omp_get_wtime();

    fprintf(stderr, "Elapsed time: %f seconds\n", end_time - start_time);
    free(array);

    return 0;
}
