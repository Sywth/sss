_default:
    @just --list

run *args:
  cargo r -p cli {{args}}

test *args:
  cargo t -p cli {{args}}


localadd crate dep:
  cargo add -p {{crate}} {{dep}} --path ./{{dep}}

localremove crate dep:
  cargo remove -p {{crate}} {{dep}}

# Add hoc commands
r1:
  cargo r -p cli -- --decide sat assets/sfol_v2/long_exists.sfol

r2: 
  cargo r -p cli -- --decide sat assets/fol_sat/test_1.fol

t1:
  cargo t -p parser -- --nocapture

