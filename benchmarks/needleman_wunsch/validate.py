f1 = "input1.txt"
f2 = "input2.txt"

with open(f1, "r") as f:
    input1 = f.read()
with open(f1, "r") as f:
    input2 = f.read()

input2_len = len(input2)
input1_len = len(input1)
matrix = [ [0] * (input2_len + 1) for i in range(input1_len + 1)]
for i in range(input1_len + 1):
    matrix[i][0] = -i
for i in range(input2_len + 1):
    matrix[0][i] = -i

for n in range(input1_len ):
    for m in range(input2_len):
        match = 0
        if(input1[n] == input2[m]):
            match = 1
        else:
            match = -1
        best = matrix[n+1][m] - 1
        if(matrix[n][m] + match > best):
            best = matrix[n][m]+match
        if(matrix[n][m+1] - 1 > best):
            best = matrix[n][m+1] -1
        matrix[n+1][m+1] = best

actual = open("needle_out.txt", 'r')

with open('output_matrix_ref','w') as f:
    for n in range(input1_len + 1):
        for m in range(input2_len + 1):
            f.write(str(matrix[n][m]))
            f.write(" ")
        f.write("\n")

actual_lines = actual.readlines()
expected = open("output_matrix_ref", 'r')
expected_lines = expected.readlines()
for i in range(0, len(actual_lines)): 
    if (actual_lines[i] != expected_lines[i]):
        print("Unmatched row " + str(i))
        print(False)
print(True)