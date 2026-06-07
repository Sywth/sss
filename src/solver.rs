use crate::args::SolverExitType;
use std::path::PathBuf;

// TODO: How do we make this work for DIMACS? We dont right?
// Given a fol file, re-write it into a normalized form
pub fn normalize(fp: PathBuf) -> SolverExitType {
    todo!()
}

// TODO: How do i determine the input and output types?
// Given a fol file cast it a output format
pub fn cast(fp: PathBuf) -> SolverExitType {
    todo!()
}

// Given a fol file determine if its sat
pub fn sat(fp: PathBuf) -> SolverExitType {
    todo!()
}
