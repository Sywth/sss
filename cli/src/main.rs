#![allow(unused)]
use std::{env, path};
use std::{process::ExitCode, str};

const LOG_LEVEL: tracing::Level = tracing::Level::DEBUG;

#[derive(Debug)]
enum DecisionProblemType {
    Sat,
    Tqbf,
    Stcon,
}

//impl DecisionProblemType {
//    pub fn decide(self, format: DecisionProblemFormat, input: Symbols) {}
//}
//
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
        format_start: DecisionProblemFormat,
        format_end: DecisionProblemFormat,

        fp: path::PathBuf,
    },
}

impl SssCommand {
    fn execute(self) -> Result<(), String> {
        match self {
            SssCommand::Decide { problem_type, fp } => {
                //problem_type::decide(fp)
                todo!()
            }
            SssCommand::Cast {
                format_start,
                format_end,
                fp,
            } => {
                todo!("cast will be implemented after decide")
            }
        }
    }
}

fn parse_args(
    mut args: impl Iterator<Item = String>,
) -> Result<SssCommand, String> {
    let Some(command) = args.next() else {
        return Err("command not provided".to_string());
    };

    let command: SssCommand = match command.as_str() {
        "--decide" | "-d" => {
            let err_msg = concat!(
                "used decide wrong, ",
                "usage: --decide ",
                "[file path] [problem type]",
            )
            .to_string();

            let Some(problem_type) = args.next() else {
                return Err(err_msg);
            };
            let problem_type: DecisionProblemType = problem_type.parse()?;

            let Some(fp) = args.next().map(path::PathBuf::from) else {
                return Err(err_msg);
            };

            SssCommand::Decide { fp, problem_type }
        }

        "--cast" | "-c" => {
            tracing::error!("cast not implemented yet!");

            let err_msg = concat!(
                "used cast wrong, ",
                "usage: --cast ",
                "[file path] [format from] [format to]",
            )
            .to_string();

            let Some(format_start) = args.next() else {
                return Err(err_msg);
            };
            let format_start: DecisionProblemFormat = format_start.parse()?;

            let Some(format_end) = args.next() else {
                return Err(err_msg);
            };
            let format_end: DecisionProblemFormat = format_end.parse()?;

            let Some(fp) = args.next().map(path::PathBuf::from) else {
                return Err(err_msg);
            };

            SssCommand::Cast {
                fp,
                format_start,
                format_end,
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
