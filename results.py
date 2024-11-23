import os
import subprocess


benchmarks = {"histogram":{}, "lu_decomposition":{"cpp":True},"merge_sort":{},
              "monte_carlo":{},"needleman_wunsch":{"math_lib":True},"quadrature":{"cpp":True}}
BASE_DIR = "benchmarks"

for b_name, attributes in benchmarks.items():
    # omp
    # compile
    compile_command = ['time']
    if "cpp" in attributes:
        compile_command.append("g++")
        extension=".cpp"
    else:
        compile_command.extend(["gcc", '-Wall', '-std=c99'])
        extension=".c"

    compile_command.append("-fopenmp")
    if "math_lib" in attributes:
        compile_command.append("-lm")

    compile_command.append(f'-o {b_name}')
    compile_command.append(f'{b_name}{extension}')
    
    result = subprocess.run(compile_command,
                            capture_output = True, text = True,
                            cwd=os.path.join(BASE_DIR,b_name,"omp"))
    print(result.stdout)
    print(result.stderr)