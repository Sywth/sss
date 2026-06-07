use std::{path::PathBuf, process::ExitCode};

use clap::{self, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sss", about = "Sywth's Sat Solver")]
pub struct Cli {
    #[command(subcommand)]
    pub solver_action: SolverAction,
}

#[derive(Subcommand)]
pub enum SolverAction {
    #[command(name = "sat")]
    Sat { file: PathBuf },

    #[command(name = "norm")]
    Normalize { file: PathBuf },

    #[command(name = "cast")]
    Cast { file: PathBuf },
}

pub enum SolverExitType {
    Ok,
    Sat,
    Unsat,
}

// Map from our exit code to DIMACs exit code
impl From<SolverExitType> for ExitCode {
    fn from(e: SolverExitType) -> ExitCode {
        match e {
            SolverExitType::Ok => ExitCode::from(0),
            SolverExitType::Sat => ExitCode::from(10),
            SolverExitType::Unsat => ExitCode::from(20),
        }
    }
}
