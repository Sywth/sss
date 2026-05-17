#[derive(Clone, Copy, Debug)]
pub enum AnsiColor {
    Red,
    Green,
    Yellow,
    Blue,
    Grey,
    Reset,
}

impl AnsiColor {
    fn code(self) -> &'static str {
        match self {
            AnsiColor::Red => "\x1b[31m",
            AnsiColor::Green => "\x1b[32m",
            AnsiColor::Yellow => "\x1b[33m",
            AnsiColor::Blue => "\x1b[34m",
            AnsiColor::Grey => "\x1b[37m",
            AnsiColor::Reset => "\x1b[0m",
        }
    }
}

pub fn as_colored<S: AsRef<str>>(text: S, color: AnsiColor) -> String {
    let text = text.as_ref();
    format!("{}{}{}", color.code(), text, AnsiColor::Reset.code())
}
