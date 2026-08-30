#![allow(unused)]
use logic::symbol::Form;
use std::{env, fs, path};
use std::{process::ExitCode, str};

const LOG_LEVEL: tracing::Level = tracing::Level::DEBUG;

// TODO should this be a trait?
#[derive(Debug)]
enum DecisionProblemType {
    Sat,
    Tqbf,
    Stcon,
}

impl DecisionProblemType {
    pub fn decide(self, format: DecisionProblemFormat, input: Form) {
        tracing::debug!(?format, ?input, "decide called");
    }
}

impl str::FromStr for DecisionProblemType {
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
enum DecisionProblemFormat {
    Sfol,
    Dimacs,
}

impl str::FromStr for DecisionProblemFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sfol" => Ok(Self::Sfol),
            "dimacs" => Ok(Self::Dimacs),
            _ => Err(format!("unknown format : {s}")),
        }
    }
}

#[derive(Debug)]
enum SssCommand {
    Decide {
        problem_type: DecisionProblemType,
        fp: path::PathBuf,
    },

    Cast {
        fmt_from: DecisionProblemFormat,
        fp_from: path::PathBuf,
        fmt_to: DecisionProblemFormat,
        fp_to: path::PathBuf,
    },
}

impl SssCommand {
    fn execute(self) -> Result<(), String> {
        match self {
            SssCommand::Decide { problem_type, fp } => {
                let file_content = fs::read_to_string(&fp).map_err(|e| {
                    format!("could not read {}: {}", fp.display(), e)
                })?;

                let res = parser::parse_sfol(file_content.as_str())?;

                // TODO: Call decide

                Ok(())
            }

            SssCommand::Cast {
                fmt_from,
                fp_from,
                fmt_to,
                fp_to,
            } => {
                tracing::debug!(?fmt_from, ?fp_from, ?fmt_to, ?fp_to);
                todo!("cast will be implemented after decide")
            }
        }
    }
}

fn parse_args(
    mut args: impl Iterator<Item = String>,
) -> Result<SssCommand, String> {
    let Some(command) = args.next() else {
        return Err("no arguments provided".to_string());
    };

    let command: SssCommand = match command.as_str() {
        "--decide" | "-d" => {
            let err_msg = concat!(
                "used decide wrong, ",
                "usage: --decide ",
                "[problem type] [file path]",
            )
            .to_string();

            let problem_type = args.next().ok_or(err_msg.as_str())?.parse()?;
            let fp = args
                .next()
                .map(path::PathBuf::from)
                .ok_or(err_msg.as_str())?;

            SssCommand::Decide { fp, problem_type }
        }

        "--cast" | "-c" => {
            tracing::error!("cast not implemented yet!");

            let err_msg = concat!(
                "used cast wrong, ",
                "usage: --cast ",
                "[fmt from] [fp from] [fmt to] [fp to]",
            )
            .to_string();

            let fmt_from = args.next().ok_or(err_msg.as_str())?.parse()?;
            let fp_from = args
                .next()
                .map(path::PathBuf::from)
                .ok_or(err_msg.as_str())?;

            let fmt_to = args.next().ok_or(err_msg.as_str())?.parse()?;
            let fp_to = args
                .next()
                .map(path::PathBuf::from)
                .ok_or(err_msg.as_str())?;

            SssCommand::Cast {
                fmt_from,
                fmt_to,
                fp_from,
                fp_to,
            }
        }

        _ => {
            let err_msg = format!("unknown command {}", command);
            return Err(err_msg);
        }
    };

    if let Some(arg) = args.next() {
        let err_msg = format!("unexpected argument {}", arg);
        return Err(err_msg);
    }

    Ok(command)
}

pub fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_max_level(LOG_LEVEL)
        .with_ansi(true)
        .with_line_number(true)
        .init();

    let command = match parse_args(env::args().skip(1)) {
        Ok(command) => command,
        Err(err) => {
            tracing::error!("{err}");
            return ExitCode::FAILURE;
        }
    };

    match command.execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            tracing::error!("{err}");
            ExitCode::FAILURE
        }
    }
}
