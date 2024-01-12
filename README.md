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
