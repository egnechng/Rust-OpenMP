#include <iostream>
#include <stdio.h>
#include <stdlib.h>
#include <string>
#include <iomanip>
#include <omp.h>
//#include "../Matrix_Formatting/Matrix_Formatting.cpp"
//#include "Sequential.h"
using namespace std;

//do LU decomposition
//a is the matrix that will be split up into l and u
//array size for all is size x size
void l_u_d(float** a, float** l, float** u, int size, int num_threads)
{
	
	//for each column...
	#pragma omp parallel num_threads(num_threads)
	for (int i = 0; i < size; i++)
	{

		//for each row....
		#pragma omp parallel for schedule(static)
		for (int j = 0; j < size; j++)
		{
			//if j is smaller than i, set l[j][i] to 0
			if (j < i)
			{
				l[j][i] = 0;
			}
			else {
				//otherwise, do some math to get the right value
				l[j][i] = a[j][i];
				#pragma omp parallel for schedule(static)
				for (int k = 0; k < i; k++)
				{
					//deduct from the current l cell the value of these 2 values multiplied
					l[j][i] = l[j][i] - l[j][k] * u[k][i];
				}
			}
		}
		//for each row...
		#pragma omp parallel for schedule(static)
		for (int j = 0; j < size; j++)
		{
			//if j is smaller than i, set u's current index to 0
			if (j < i)
			{
				u[i][j] = 0;
			}
			else {
				//if they're equal, set u's current index to 1
				if (j == i)
				{
					u[i][j] = 1;
					continue;
				}
				//otherwise, do some math to get the right value
				u[i][j] = a[i][j] / l[i][i];
				#pragma omp parallel for schedule(static)
				for (int k = 0; k < i; k++)
				{
					u[i][j] = u[i][j] - ((l[i][k] * u[k][j]) / l[i][i]);
				}
			}

		}
	}
}

void print_matrix_2D(float** matrix , int size){
	// Loop through the rows
    for (int i = 0; i < size; i++) {
        // Loop through the columns
        for (int j = 0; j < size; j++) {
            cout << matrix[i][j] << " ";  // Print each element followed by a space
        }
        cout << endl;  // Print a newline after each row
    }
}



//initialize the matrices
void initialize_matrices(float** a, float** l, float** u, int size)
{
	//for each row in the 2d array, initialize the values
	for (int i = 0; i < size; ++i)
	{
		a[i] = new float[size];
		l[i] = new float[size];
		u[i] = new float[size];
	}
}

//fill the array with random values (done for a)
void random_fill(float** matrix, int size)
{
	//fill a with random values
	cout << "Producing random values " << endl;
	for (int i = 0; i < size; i++)
	{
		for (int j = 0; j < size; j++)
		{
			matrix[i][j] = (float) ((rand() % 10) + 1);
		}
	}

	//Ensure the matrix is diagonal dominant to guarantee invertible-ness
	//diagCount well help keep track of which column the diagonal is in
	int diagCount = 0;
	float sum = 0;
	for (int i = 0; i < size; i++)
	{
		for (int j = 0; j < size; j++)
		{
			//Sum all column vaalues
			sum += abs(matrix[i][j]);
		}
		//Remove the diagonal  value from the sum
		sum -= abs(matrix[i][diagCount]);
		//Add a random value to the sum and place in diagonal position
		matrix[i][diagCount] = sum + ((rand() % 5) + 1);
		++diagCount;
		sum = 0;
	}
}

int main(int argc, char** argv)
{	
	// num_threads
	int t = atoi(argv[1]);
	
	//size of matrix
	int size = atoi(argv[2]);

	double runtime;

	//seed rng
	srand(1);

	cout << "Your input matrix size is ";
	cout << size << " x " << size << endl;
	fflush(stdout);

	//initalize matricess
	float** a = new float* [size];
	float** l = new float* [size];
	float** u = new float* [size];
	initialize_matrices(a, l, u, size);
	//fill a with random values
	random_fill(a, size);
	////print A
	//cout << "A Matrix: " << endl;
	//print_matrix_2D(a, size);
	//do LU decomposition
	runtime = omp_get_wtime ( );
	l_u_d(a, l, u, size, t);
	runtime = omp_get_wtime ( ) - runtime;
	//print l and u
	//cout << "L Matrix: " << endl;
	//print_matrix_2D(l, size);
	//cout << "U Matrix:" << endl;
	//print_matrix_2D(u, size);
	cerr << "Time for actual program:(" << runtime << ")s" << endl;;
	return 0;
}