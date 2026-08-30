use logic::symbol::{FolArena, IdForm, Sym};
use std::collections::HashMap;
use util::dbg_boxed;

pub struct ParsedFolForm {
    pub arena: FolArena,
    pub root: IdForm,
    pub symbol_table: HashMap<String, Sym>,
}

#[derive(Debug, PartialEq, Eq)]
enum Token {
    LParen,
    RParen,
    Atom(String),
}

mod ssym {
    // Grammar
    pub const G_COMMENT: &str = ";;";
    pub const G_LPAREN: char = '(';
    pub const G_RPAREN: char = ')';

    // Language Keywords
    pub const CON: &str = "and";
    pub const DIS: &str = "or";
    pub const NEG: &str = "not";
    pub const IMPLIES: &str = "implies";

    pub const EXISTS: &str = "exists";
    pub const FORALL: &str = "forall";
}

/// Assumes input is utf-8
fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    let mut byte_idx = 0;

    while byte_idx < input.len() {
        let curr = input[byte_idx..].chars().next().ok_or_else(|| {
            format!("lexer couldn't read char at index {byte_idx}")
        })?;

        if curr.is_whitespace() {
            byte_idx += curr.len_utf8();
            continue;
        }

        let rhs = &input[byte_idx..];

        // consume to next newline or end of file on comments
        if rhs.starts_with(ssym::G_COMMENT) {
            byte_idx += rhs.find('\n').unwrap_or(rhs.len());
            continue;
        }

        match curr {
            ssym::G_LPAREN => {
                byte_idx += ssym::G_LPAREN.len_utf8();
                tokens.push(Token::LParen);
                continue;
            }
            ssym::G_RPAREN => {
                byte_idx += ssym::G_RPAREN.len_utf8();
                tokens.push(Token::RParen);
                continue;
            }
            _ => {}
        }

        let lexeme_start = byte_idx;
        while byte_idx < input.len() {
            let rhs = &input[byte_idx..];
            if rhs.starts_with(ssym::G_COMMENT) {
                break;
            }

            let curr = rhs.chars().next().ok_or_else(|| {
                format!("lexer could not read char at byte index {byte_idx}")
            })?;

            let char_is_not_part_of_a_lexeme: bool = curr.is_whitespace()
                || curr == ssym::G_LPAREN
                || curr == ssym::G_RPAREN;
            if char_is_not_part_of_a_lexeme {
                break;
            }

            byte_idx += curr.len_utf8();
        }

        if byte_idx == lexeme_start {
            return Err(format!("unexpected char at byte index {byte_idx}"));
        }
        debug_assert!(byte_idx > lexeme_start, "bad logic");

        let lexeme_slice = input[lexeme_start..byte_idx].to_owned();
        tokens.push(Token::Atom(lexeme_slice));
    }

    Ok(tokens)
}

enum Sexp {
    Atom(String),
    List(Vec<Sexp>),
}

fn parse_sexp(tokens: &[Token]) -> Result<Sexp, String> {
    todo!();
}

pub fn parse_sfol(input: &str) -> Result<ParsedFolForm, String> {
    dbg_boxed!("{}", input);
    let res = lex(input);

    todo!();
}

#[cfg(test)]
fn fixture_core_formula() -> String {
    format!(
        "({exists} (x1) ({con} (P x2) x3))",
        exists = ssym::EXISTS,
        con = ssym::CON,
    )
}

#[cfg(test)]
fn fixture_core_formula_expected_tokens() -> Vec<Token> {
    vec![
        Token::LParen,
        Token::Atom(ssym::EXISTS.into()),
        Token::LParen,
        Token::Atom("x1".into()),
        Token::RParen,
        Token::LParen,
        Token::Atom(ssym::CON.into()),
        Token::LParen,
        Token::Atom("P".into()),
        Token::Atom("x2".into()),
        Token::RParen,
        Token::Atom("x3".into()),
        Token::RParen,
        Token::RParen,
    ]
}

#[test]
fn lexer_core_formula() {
    let tokens_produced = lex(&fixture_core_formula()).unwrap();
    assert_eq!(tokens_produced, fixture_core_formula_expected_tokens());
}

#[test]
fn lexer_fol_comments() {
    let input = format!(
        r#"
            ;; E x1. P x1 /\ x2
            ;; There exists x1 such that P x1 and open x2 are true

            {} ;; This comment goes to eol
            "#,
        fixture_core_formula(),
    );

    let tokens_produced = lex(&input).unwrap();
    assert_eq!(tokens_produced, fixture_core_formula_expected_tokens());
}

#[test]
fn lexer_parentheses_without_whitespace() {
    let tokens_produced = lex("(Father(zorlak))").unwrap();

    assert_eq!(
        tokens_produced,
        vec![
            Token::LParen,
            Token::Atom("Father".into()),
            Token::LParen,
            Token::Atom("zorlak".into()),
            Token::RParen,
            Token::RParen,
        ]
    );
}

#[test]
fn lexer_utf8_atoms() {
    let tokens_produced =
        lex("(ßsome ger\r\tman utf--8café λx\t\r 日本語 \r\r)").unwrap();

    assert_eq!(
        tokens_produced,
        vec![
            Token::LParen,
            Token::Atom("ßsome".into()),
            Token::Atom("ger".into()),
            Token::Atom("man".into()),
            Token::Atom("utf--8café".into()),
            Token::Atom("λx".into()),
            Token::Atom("日本語".into()),
            Token::RParen,
        ]
    );
}
