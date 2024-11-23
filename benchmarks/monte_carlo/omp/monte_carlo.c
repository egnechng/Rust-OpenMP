#include <stdio.h>
#include <stdlib.h>
#include <omp.h>

//gcc -Wall -std=c99 -fopenmp -o monte monte-carlo.c; ./monte 128 100000000

int main(int argc, char* argv[]){

    double t_start = 0.0, ttaken;

    long long trial_count;
    int t_count;


    // Does a parameter check to make sure we have the right inputs.
    if(argc != 3){
        printf("You need to enter the thread count and trial count for simulation.");
        return 1;
    }

    // Reads in the arguments
    t_count = atoi(argv[1]);
    trial_count = atoll(argv[2]);

    double RADIUS = 1.000;
    double RADIUS_SQUARED = RADIUS * RADIUS;
    omp_set_num_threads(t_count);

    t_start = omp_get_wtime(); 

    // We parallelize with chunk size of 256, while I have theoretical guesses for why this works better
    // (cache lines on cores, etc.) I converged upon it through emperical trial-and-error.
    long long circle_count = 0;
    #pragma omp parallel 
    {
        unsigned int seed = omp_get_thread_num() + 3;

        #pragma omp for schedule(static) reduction(+:circle_count)
        for (long long i = 0; i < trial_count; i++ ){
            
            double coord_1 = (double)rand_r(&seed)/(double)RAND_MAX;
            double coord_2 = (double)rand_r(&seed)/(double)RAND_MAX;

            double landed = coord_1 * coord_1;
            landed += coord_2 * coord_2;
            if (landed <= RADIUS_SQUARED){
                circle_count++;
            }
        }
    }
    double pi_estimate = 4.0 * ((double)circle_count/(double)trial_count);
    ttaken = omp_get_wtime() - t_start;
    
    printf("\n %lld trials, pi is %lf \n",trial_count, pi_estimate);
    printf("Time take for the main part: %f\n", ttaken);



    return 0;
}

