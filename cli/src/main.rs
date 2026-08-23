use std::env;
use std::{process::ExitCode, str};

const LOG_LEVEL: tracing::Level = tracing::Level::DEBUG;

#[derive(Debug)]
enum DecisionProblem {
    Sat,
    Tqbf,
    Stcon,
}

impl str::FromStr for DecisionProblem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sat" => Ok(Self::Sat),
            "tqbf" => Ok(Self::Tqbf),
            "stcon" => Ok(Self::Stcon),
            _ => Err(format!("unknown decision problem: {s}")),
        }
    }
}

#[derive(Debug)]
enum SssCommand {
    Decide(DecisionProblem),
}

fn parse_args(
    mut args: impl Iterator<Item = String>,
) -> Result<SssCommand, String> {
    let Some(command) = args.next() else {
        return Err("command not provided".into());
    };

    let command = match command.as_str() {
        "--decide" | "-d" => {
            let problem = args
                .next()
                .ok_or_else(|| "decision problem type not provided".to_owned())?
                .parse::<DecisionProblem>()?;

            SssCommand::Decide(problem)
        }
        _ => return Err(format!("unknown command {command}")),
    };

    if let Some(arg) = args.next() {
        return Err(format!("unexpected argument: {arg}"));
    }

    Ok(command)
}

pub fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_max_level(LOG_LEVEL)
        .with_ansi(true)
        .init();

    let command = match parse_args(env::args().skip(1)) {
        Ok(command) => command,
        Err(err) => {
            tracing::error!("{err}");
            return ExitCode::FAILURE;
        }
    };

    tracing::debug!(?command, "parsed command");
    ExitCode::SUCCESS
}
