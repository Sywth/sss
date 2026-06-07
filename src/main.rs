use clap::Parser;
use sss::{
    args::{Cli, SolverAction, SolverExitType},
    solver, CliError,
};
use std::process::ExitCode;
use tracing::Level;

fn run(cli: Cli) -> Result<SolverExitType, CliError> {
    // TODO
    // This needs to be nested I think, also how do we handle
    // other stuff like --max-timeout?
    match cli.solver_action {
        SolverAction::Sat { file: fp } => Ok(solver::sat(fp)),
        SolverAction::Normalize { file: fp } => Ok(solver::normalize(fp)),
        SolverAction::Cast { file: fp } => Ok(solver::cast(fp)),
    }
}

fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_max_level(Level::ERROR)
        .with_ansi(true)
        .init();

    let cli = Cli::parse();
    match run(cli) {
        Ok(solver_exit) => solver_exit.into(),
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(1)
        }
    }
}
