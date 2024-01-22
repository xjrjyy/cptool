# CP Tool

CP Tool is a command line tool for competitive programming.

## Usage

```bash
# generate problem in ./example/a_plus_b
./cptool -w ./example/a_plus_b

# export problem to online judge format
# currently only support syzoj
./cptool -w ./example/a_plus_b --export-oj=syzoj
# export to ./output/syzoj
./cptool -w ./example/a_plus_b -e=syzoj --export-dir=./output

# for more information
./cptool --help
```

`problem.yaml` is the problem description file.

```yaml
name: a_plus_b # problem name
programs:
  gen: # program name
    info: !command
      path: ./gen # command path
      extra_args: [] # extra arguments, optional
    time_limit_secs: 1.0
    memory_limit_mb: 512.0
  std:
    info: !cpp
      path: ./std.cpp
      compile_args: [-O2, -std=c++14] # compile arguments, default to [-O2]
    time_limit_secs: 1.0
    memory_limit_mb: 512.0
  val:
    info: !cpp
      path: ./val.cpp
      compile_args: [-O2, -I../assets/testlib/]
    time_limit_secs: 1.0
    memory_limit_mb: 512.0
  chk:
    info: !cpp
      path: ../assets/testlib/checkers/lcmp.cpp
      compile_args: [-O2, -I../assets/testlib/]
    time_limit_secs: 1.0
    memory_limit_mb: 512.0
solution: std
validator: val # optional
checker: chk # optional
test:
  bundles: # data bundles
    sample: # bundle name
      cases:
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
  - name: sample
    score: 1.0
    type: min
    bundles: [sample] # bundle names
  - name: main
    score: 99.0
    type: sum
    bundles: [main]
    dependencies: [sample] # task names
```

## Notes

+ Syzoj export is not fully supported yet.
+ Generator use multiple threads to generate data, so it may be slower than single thread generator.
