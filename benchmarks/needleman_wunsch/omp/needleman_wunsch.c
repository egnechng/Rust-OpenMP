#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <omp.h>
#include <math.h>

// Run: gcc -Wall -std=c99 -fopenmp -lm -o needle needle.c; ./needle 2 ../input1.txt ../input2.txt false -1576

int main(int argc, char **argv) {
    double start_time,end_time;
    int t_count;
    bool print_scores = false;


    if(argc != 5){
        printf("You need to enter the thread count, two input files, and print traceback flag\n");
        return 1;
    }

    t_count = atoi(argv[1]);
    if (strcmp(argv[4], "true") == 0){
        print_scores = true;
    }

    char* input1;
    int input1_len = 0;
    char* input2;
    int input2_len = 0;

    FILE *file1;
    file1 = fopen(argv[2], "r");
    if(file1 != NULL) {
        fseek(file1, 0, SEEK_END);
        input1_len = ftell(file1);
        rewind(file1);
        input1 = (char*) malloc(sizeof(char) * (input1_len + 1) );
        size_t read_size = fread(input1, sizeof(char), input1_len, file1);
        input1[input1_len] = '\0';
        if (input1_len != read_size){
            printf("Error opening file 1");
            return 1;
        }
    } else {
        printf("Error opening file 1");
        return 1;
    }

    fclose(file1);

    FILE *file2;
    file2 = fopen(argv[3], "r");
    if(file2 != NULL) {
        fseek(file2, 0, SEEK_END);
        input2_len = ftell(file2);
        rewind(file2);
        input2 = (char*) malloc(sizeof(char) * (input2_len + 1) );
        size_t read_size = fread(input2, sizeof(char), input2_len, file2);
        input2[input2_len] = '\0';

        if (input2_len != read_size){
            printf("Error opening file 2");
            return 1;
        }
    } else {
        printf("Error opening file 2");
        return 1;
    }

    fclose(file2);

    input1_len++;
    input2_len++;

    omp_set_num_threads(t_count);
    int* scores = (int*)calloc(input1_len * input2_len, sizeof(int));

    for (int i = 0; i < input2_len; i++) {
        scores[i] = -i;
    }
    for (int i = 0; i < input1_len; i++) {
        scores[i * input2_len] = -i;
    }

    start_time = omp_get_wtime();

//https://stackoverflow.com/questions/31321071/openmp-nested-for-loop-becomes-faster-when-having-parallel-before-outer-loop#:~:text=In%20this%20method%2C%20like%20method%201%2C%20each%20thread%20runs%20over%20all%20n%2D1%20iteration%20over%20i.%20However%2C%20this%20method%20has%20an%20implicit%20barrier%20after%20the%20inner%20loop%20which%20causes%20each%20thread%20to%20pause%20until%20all%20threads%20have%20finished%20a%20row%20making%20this%20method%20sequential%20for%20each%20row%20like%20method%202.
    #pragma omp parallel 
    for(int i = 1; i < input2_len + input1_len - 1; i++) {
        int min_value = fmin(input1_len, i+1);

        #pragma omp for schedule(static)
        for(int j = fmax(1, i-input2_len+2); j < min_value; j++) {
            int mat_y = j;
            int mat_x = i - j + 1;
            int is_match = 1;
            if (input2[mat_x - 1] != input1[mat_y - 1]){
                is_match = -1;
            }
            int score_if_match = scores[(mat_y - 1)* input2_len + mat_x - 1] + is_match;
            int dp_left = scores[mat_y * input2_len + mat_x - 1] - 1;
            int dp_top = scores[(mat_y - 1) * input2_len + mat_x] - 1;
            int dp_max = fmax(dp_left, dp_top);
            dp_max = fmax(score_if_match, dp_max);
            scores[mat_y * input2_len + mat_x] = dp_max;
        }
    }

    end_time = omp_get_wtime();

    fprintf(stderr, "Time for actual program:(%.12f)s\n", end_time-start_time);
    int actual_score = scores[(input1_len - 1) * input2_len + (input2_len - 1)];
    printf("Actual best score: %d\n", actual_score);

    free(input1);
    free(input2);

    if (print_scores){
        FILE *output_file = fopen("needle_out.txt", "w");
        if (output_file == NULL){
            printf("Error occured while opening output file\n");
            exit(1);
        }


        for(int i = 0; i < input1_len; i++){
            for(int j = 0; j < input2_len; j++){
                fprintf(output_file, "%d ", scores[i * input2_len + j]);
            }
            fprintf(output_file,"\n");
        }
    }

    free(scores);
}