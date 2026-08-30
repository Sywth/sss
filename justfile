_default:
    @just --list

run *args:
  cargo r -p cli {{args}}

test *args:
  cargo t -p cli {{args}}
