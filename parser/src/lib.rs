use std::collections::HashMap;

use logic::symbol::{FolArena, Form, IdForm, Sym};

pub struct ParsedFolForm {
    pub arena: FolArena,
    pub root: IdForm,
    pub symbol_table: HashMap<String, Sym>,
}

#[derive(Debug)]
enum Token {
    LParen,
    RParen,
    Atom(String),
}

mod ssym {
    pub const COMMENT: &str = ";;";
    pub const LPAREN: char = '(';
    pub const RPAREN: char = ')';

    //const CON: &str = "and";
    //const DIS: &str = "or";
    //const NEG: &str = "not";
    //const IMPLIES: &str = "implies";
    //
    //const EXISTS: &str = "exists";
    //const FORALL: &str = "forall";
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
        if rhs.starts_with(ssym::COMMENT) {
            byte_idx += rhs.find('\n').unwrap_or(input.len());
            continue;
        }

        match curr {
            ssym::LPAREN => {
                byte_idx += ssym::LPAREN.len_utf8();
                tokens.push(Token::LParen);
                continue;
            }
            ssym::RPAREN => {
                byte_idx += ssym::RPAREN.len_utf8();
                tokens.push(Token::RParen);
                continue;
            }
            _ => {}
        }

        let lexeme_start = byte_idx;
        while byte_idx < input.len() {
            let rhs = &input[byte_idx..];
            if rhs.starts_with(ssym::COMMENT) {
                break;
            }

            let c = rhs.chars().next().ok_or_else(|| {
                format!("lexer could not read char at byte index {byte_idx}")
            })?;

            let char_is_not_part_of_a_lexeme: bool =
                c.is_whitespace() || c == ssym::LPAREN || c == ssym::RPAREN;
            if char_is_not_part_of_a_lexeme {
                break;
            }

            byte_idx += 1;
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
    tracing::debug!("\n-----\n{}\n-----\n", input);

    let res = lex(input);
    // TODO: Finish tokezning and ast-ising
    dbg!(res);

    todo!();
}
