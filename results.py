import json
import os
import subprocess


benchmark_settings = {
    "histogram":{
        "sizes":[10000,50000,1000000,2500000,5000000,7500000,10000000]
    },
    "merge_sort":{
        "sizes":[10000,50000,1000000,2500000,5000000,7500000,10000000],        
    },
    "lu_decomposition":{
        "cpp":True,
        "sizes":[50, 100, 200, 400, 800],
    },
    "quadrature":{
        "cpp":True,
        "sizes":[1000, 10000, 100000, 1000000],   
    },
    "monte_carlo":{
        "sizes":[10000,50000,1000000,2500000,5000000,7500000,10000000],   
    },
    "needleman_wunsch":{
        "math_lib":True,
        "needs_input_files": True,
        "extra_inputs": ["false"],
        "sizes":[500, 1000, 2500, 5000, 7500, 10000],
    },
}
thread_counts = [1,2,4,8,16,32]
benchmarks = {"histogram":{},"merge_sort":{},"lu_decomposition":{},
             "quadrature":{}, "monte_carlo":{},"needleman_wunsch":{}}

if os.path.isfile("saved_suite_history.json"):
    with open("saved_suite_history.json","r") as f:
        benchmarks = json.load(f)

BASE_DIR = "benchmarks"

for b_name, attributes in benchmarks.items():
    ### omp compile
    compile_command = ['time']
    if "cpp" in benchmark_settings[b_name]:
        compile_command.append("g++")
        extension=".cpp"
    else:
        compile_command.extend(["gcc", '-Wall', '-std=c99'])
        extension=".c"

    compile_command.append("-fopenmp")
    if "math_lib" in benchmark_settings[b_name]:
        compile_command.append("-lm")

    compile_command.append(f'-o {b_name}')
    compile_command.append(f'{b_name}{extension}')
    
    result = subprocess.run(compile_command,
                            capture_output = True, text = True,
                            cwd=os.path.join(BASE_DIR,b_name,"omp"))
    # print(result.stderr)

    elapsed_string =  [x for x in result.stderr.split(" ") if "elapsed" in x][0].replace("elapsed","")
    elapsed = float(elapsed_string.split(":")[0]) * 60 + float(elapsed_string.split(":")[1])
    attributes.setdefault("omp_compiletime",[]).append(elapsed)
    # print(result.stderr)

    ### rust compile
    subprocess.run(['cargo', 'clean', '--release'],
                            capture_output = True, text = True,
                            cwd=os.path.join(BASE_DIR,b_name,"rust"))
    result  = subprocess.run(['time', 'cargo', 'build', '--release'],
                            capture_output = True, text = True,
                            cwd=os.path.join(BASE_DIR,b_name,"rust"))
    
    elapsed_string =  [x for x in result.stderr.split(" ") if "elapsed" in x][0].replace("elapsed","")
    elapsed = float(elapsed_string.split(":")[0]) * 60 + float(elapsed_string.split(":")[1])
    attributes.setdefault("rust_compiletime",[]).append(elapsed)

    # ### omp run
    # for tc in thread_counts:
    #     for ps in benchmark_settings[b_name]["sizes"]:
    #         run_command = [f"./{b_name}",str(tc)]
    #         result  = subprocess.run(['time', 'cargo', 'build', '--release'],
    #                         capture_output = True, text = True,
    #                         cwd=os.path.join(BASE_DIR,b_name,"rust"))

print(benchmarks)

with open("saved_suite_history.json","w") as f:
    json.dump(benchmarks,f, indent=2)