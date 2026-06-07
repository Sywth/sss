use core::fmt::Display;

#[derive(Debug)]
pub enum CliError {
    NoInputFile,
    UnknownFlag(String),
    ParsingError(String),
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::NoInputFile => write!(f, "no input file provided"),
            CliError::UnknownFlag(s) => write!(f, "unknown flag: {s}"),
            CliError::ParsingError(s) => write!(f, "parsing error: {s}"),
        }
    }
}
