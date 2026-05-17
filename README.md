# SSS

***S***ywth's ***S***at ***S***olver. A SAT solver, currently using [DPLL](https://en.wikipedia.org/wiki/DPLL_algorithm).

![Gif of the sat solver running with pretty print from stderr](./assets/repo-misc/sat-solver-pretty-stderr-print.gif)

## Info

Project is WIP, hence no builds yet. You can run against a file in the [DIMACS](https://people.sc.fsu.edu/~jburkardt/data/cnf/cnf.html) cnf format using cargo like so

```bash
cargo run -- path/to/dimacs/file.cnf
```

### Output 
Outputs either `SAT` or `UNSAT` to `stdout`. Optionally outputs logs, errors and visualization to `stderr`. 

### Tests

```
just test
```
