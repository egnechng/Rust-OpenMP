import json
import os
import re
import subprocess


benchmark_settings = {
    "histogram":{
        "sizes":["10000","50000","1000000","2500000","5000000","7500000","10000000"]
    },
    "merge_sort":{
        "sizes":["10000","50000","1000000","2500000","5000000","7500000","10000000"],        
    },
    "lu_decomposition":{
        "cpp":True,
        "sizes":["50", "100", "200", "400", "800"],
    },
    "quadrature":{
        "cpp":True,
        "sizes":["1000", "10000", "100000", "1000000"],   
    },
    "monte_carlo":{
        "sizes":["10000","50000","1000000","2500000","5000000","7500000","10000000"],   
    },
    "needleman_wunsch":{
        "math_lib":True,
        "needs_input_files": True,
        "extra_inputs": ["false"],
        "sizes":["500", "1000", "2500", "5000", "7500", "10000"],
    },
}
thread_counts = ["1","2","4","8","16","32"]
benchmarks = {
    "histogram":{},
    "merge_sort":{},
    "lu_decomposition":{},
    "quadrature":{}, 
    "monte_carlo":{},
    "needleman_wunsch":{}
}

if os.path.isfile("saved_suite_history.json"):
    with open("saved_suite_history.json","r") as f:
        try:
            benchmarks = json.load(f)
        except ValueError as e:
            pass



def extract_elapsed(stderr):
    elapsed_string =  [x for x in stderr.split(" ") if "elapsed" in x][0].replace("elapsed","")
    return float(elapsed_string.split(":")[0]) * 60 + float(elapsed_string.split(":")[1])

def extract_perf_stats(serr,res):
    total_wall_time = extract_elapsed(serr)
    res.setdefault("runtime_overhead",[]).append(max(total_wall_time-res["wall_time"][-1],0))

    s_line = [x for x in serr.split("\n") if "cycles:u" in x][0]
    if s_line.split(",")[-1] == 'GHz':
        res.setdefault("cycles_ghz",[]).append(float(s_line.split(",")[-2]))

    s_line = [x for x in serr.split("\n") if "task-clock:u" in x][0]
    res.setdefault("cpus_used",[]).append(float(s_line.split(",")[-2]))

    s_line = [x for x in serr.split("\n") if "LLC-loads:u" in x][0]
    if s_line.split(",")[-1] == 'K/sec':
        res.setdefault("last_l_load_per_sec",[]).append(float(s_line.split(",")[-2])*1000)
    if s_line.split(",")[-1] == 'M/sec':
        res.setdefault("last_l_load_per_sec",[]).append(float(s_line.split(",")[-2])*1_000_000)

    s_line = [x for x in serr.split("\n") if "L1-dcache-loads:u" in x][0]
    if s_line.split(",")[-1] == 'K/sec':
        res.setdefault("l1_load_per_sec",[]).append(float(s_line.split(",")[-2])*1000)
    if s_line.split(",")[-1] == 'M/sec':
        res.setdefault("l1_load_per_sec",[]).append(float(s_line.split(",")[-2])*1_000_000)
        

    s_line = [x for x in serr.split("\n") if "LLC-load-misses:u" in x][0]
    if s_line.split(",")[-1] == 'of all LL-cache accesses':
        res.setdefault("ll_miss_percent",[]).append(float(s_line.split(",")[-2]))
    # s_line = [x for x in result.stderr.split("\n") if "LLC-load-misses:u" in x][0]
    # res["cpus_used"] = s_line.split(",")[-2]
    # cycles ghz only
    # task-clock (cpus utilized)
    # LLC-load-misses


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

    compile_command.extend(['-o',b_name, f'{b_name}{extension}'])
    
    result = subprocess.run(compile_command,
                            stderr=subprocess.PIPE, text = True,
                            cwd=os.path.join(BASE_DIR,b_name,"omp"))
    # print(result.stderr)

    attributes.setdefault("omp_compiletime",[]).append(extract_elapsed(result.stderr))

    ## rust compile
    subprocess.run(['cargo', 'clean', '--release'],
                            stderr=subprocess.PIPE, text = True,
                            cwd=os.path.join(BASE_DIR,b_name,"rust"))
    result  = subprocess.run(['time', 'cargo', 'build', '--release'],
                            stderr=subprocess.PIPE, text = True,
                            cwd=os.path.join(BASE_DIR,b_name,"rust"))
    
    attributes.setdefault("rust_compiletime",[]).append(extract_elapsed(result.stderr))

    for tc in thread_counts:
        for ps in benchmark_settings[b_name]["sizes"]:
            
            ### omp run
            run_command = ["perf", "stat","-d","-d","--no-big-num","-x",",","time",f"./{b_name}",str(tc)]
            if "needs_input_files" in benchmark_settings[b_name]:
                run_command.extend([f"../{ps}input1.txt",f"../{ps}input2.txt"])
            else:
                run_command.append(str(ps))
            
            if "extra_inputs" in benchmark_settings[b_name]:
                run_command.extend(benchmark_settings[b_name]["extra_inputs"])
            print(run_command)
            result  = subprocess.run(run_command,
                            stderr=subprocess.PIPE, text = True,
                            cwd=os.path.join(BASE_DIR,b_name,"omp"))
            print(result.stderr)
            elapsed_string = [x for x in result.stderr.split("\n") if "Time for actual program:" in x][0]
            elapsed = float(re.search('\(([^)]+)',elapsed_string).group(1))
            attributes.setdefault("omp_run_stats",{}
                                  ).setdefault(tc,{}
                                ).setdefault(ps,{}).setdefault("wall_time",[]).append(elapsed)
            extract_perf_stats(result.stderr,attributes["omp_run_stats"][tc][ps])

            ### rust run
            run_command = ["perf", "stat","-d","-d","--no-big-num","-x",",","time","cargo","run","--release",str(tc)]
            if "needs_input_files" in benchmark_settings[b_name]:
                run_command.extend([f"../{ps}input1.txt",f"../{ps}input2.txt"])
            else:
                run_command.append(str(ps))
            
            if "extra_inputs" in benchmark_settings[b_name]:
                run_command.extend(benchmark_settings[b_name]["extra_inputs"])
            print(run_command)
            result  = subprocess.run(run_command,
                            stderr=subprocess.PIPE, text = True,
                            cwd=os.path.join(BASE_DIR,b_name,"rust"))
            print(result.stderr)
            elapsed_string = [x for x in result.stderr.split("\n") if "Time for actual program:" in x][0]
            elapsed = float(re.search('\(([^)]+)',elapsed_string).group(1))
            attributes.setdefault("rust_run_stats",{}
                                  ).setdefault(tc,{}
                                ).setdefault(ps,{}).setdefault("wall_time",[]).append(elapsed)
            extract_perf_stats(result.stderr,attributes["rust_run_stats"][tc][ps])
print(benchmarks)

with open("saved_suite_history.json","w") as f:
    json.dump(benchmarks,f, indent=2)

