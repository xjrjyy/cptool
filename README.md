# CP Tool

CP Tool is a command line tool for competitive programming.

## Usage

```bash
# generate problem in ./example/a_plus_b
./cptool -w ./example/a_plus_b

# generate data bundles to subdirectory
./cptool -w ./example/a_plus_b --subdir

# export problem to online judge format
# currently only support syzoj
./cptool -w ./example/a_plus_b --export-oj=syzoj
```

`problem.yaml` is the problem description file.

```yaml
name: a_plus_b # problem name
programs:
  gen: # program name
    info: !command
      path: ./gen # command path
      extra_args: [] # extra arguments, default to []
    time_limit_secs: 1.0
    memory_limit_mb: 512.0
  std:
    info: !cpp
      path: ./std.cpp
      compile_args: [-O2, -std=c++14] # compile arguments, default to [-O2]
    time_limit_secs: 1.0
    memory_limit_mb: 512.0
test:
  bundles: # data bundles
    sample: # bundle name
      cases:
      - generator: $file # built-in generator, $file means read from file
        args: [./input/0.in] # only one argument, path to input file
      - generator: gen # program name
        args: [20] # arguments to program
    main:
      cases:
      - generator: gen
        args: [10]
      - generator: gen
        args: [10000000]
      - generator: gen
        args: [1000000000]
  tasks: # subtasks
    sample: # task name
      score: 1.0
      type: min
      bundles: [sample] # bundle names
    main:
      score: 99.0
      type: sum
      bundles: [sample, main]
validator: val # optional
solution: std
```
