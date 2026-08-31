#![allow(unused)]

use util::dbg_boxed;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Token {
    LParen,
    RParen,
    Atom(String),
}

// Language Syntax
mod sy {
    // Grammar
    pub const G_COMMENT: &str = ";;";
    pub const G_LPAREN: char = '(';
    pub const G_RPAREN: char = ')';

    // Keywords
    pub const CON: &str = "and";
    pub const DIS: &str = "or";
    pub const NEG: &str = "not";
    pub const IMPLIES: &str = "implies";

    pub const EXISTS: &str = "exists";
    pub const FORALL: &str = "forall";

    pub const TOP: &str = "top";
    pub const BOT: &str = "bot";
    pub const EQU: &str = "eq";
}

/// Assumes input is utf-8
fn lex_to_tokens(input: &str) -> Result<Vec<Token>, String> {
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
        if rhs.starts_with(sy::G_COMMENT) {
            byte_idx += rhs.find('\n').unwrap_or(rhs.len());
            continue;
        }

        match curr {
            sy::G_LPAREN => {
                byte_idx += sy::G_LPAREN.len_utf8();
                tokens.push(Token::LParen);
                continue;
            }
            sy::G_RPAREN => {
                byte_idx += sy::G_RPAREN.len_utf8();
                tokens.push(Token::RParen);
                continue;
            }
            _ => {}
        }

        let lexeme_start = byte_idx;
        while byte_idx < input.len() {
            let rhs = &input[byte_idx..];
            if rhs.starts_with(sy::G_COMMENT) {
                break;
            }

            let curr = rhs.chars().next().ok_or_else(|| {
                format!("lexer could not read char at byte index {byte_idx}")
            })?;

            let char_is_not_part_of_a_lexeme: bool = curr.is_whitespace()
                || curr == sy::G_LPAREN
                || curr == sy::G_RPAREN;
            if char_is_not_part_of_a_lexeme {
                break;
            }

            byte_idx += curr.len_utf8();
        }

        // i.e. do not break on the first iteration of the while
        if byte_idx == lexeme_start {
            return Err(format!("unexpected char at byte index {byte_idx}"));
        }
        debug_assert!(
            byte_idx > lexeme_start,
            "lexer must consume at least one byte for valid lexeme"
        );

        let lexeme_slice = input[lexeme_start..byte_idx].to_owned();
        tokens.push(Token::Atom(lexeme_slice));
    }

    Ok(tokens)
}

fn parse_to_sexpr_core(
    tokens: &[Token],
    mut idx: usize,
) -> Result<(SExpr, usize), String> {
    let token = tokens
        .get(idx)
        .ok_or_else(|| format!("could not get token at index {idx}"))?;

    let (sexpr, idx) = match token {
        // On a left paren we consume up to and including the right paren
        Token::LParen => {
            idx += 1;
            let mut items = Vec::new();

            loop {
                match tokens.get(idx) {
                    None => {
                        return Err(format!(
                            "unclosed '{}' at index {idx}",
                            sy::G_LPAREN
                        ));
                    }

                    Some(Token::RParen) => {
                        return Ok((SExpr::List(items), idx + 1));
                    }

                    Some(_) => {
                        let (item, next_idx) =
                            parse_to_sexpr_core(tokens, idx)?;
                        idx = next_idx;
                        items.push(item);
                    }
                }
            }
        }

        Token::Atom(atom) => Ok((SExpr::Atom(atom.clone()), idx + 1)),

        Token::RParen => Err(format!(
            "unexpected opening token '{}' at idx {idx}",
            sy::G_RPAREN
        )),
    }?;

    Ok((sexpr, idx))
}

#[derive(Debug, Clone, PartialEq)]
enum SExpr {
    Atom(String),
    List(Vec<SExpr>),
}

fn parse_to_sexpr(tokens: &[Token]) -> Result<SExpr, String> {
    let (sexpr, idx) = parse_to_sexpr_core(tokens, 0)?;
    if idx != tokens.len() {
        return Err(format!(
            "tokens found after parsing s-expression at index {idx}"
        ));
    }

    Ok(sexpr)
}

// "Symbol" is an overloaded word and refers
// to either the string name x
// or its underlying numerical id
//
// Hence use SymName to refer the lexed name
// of an identifier and
// IdSy to the underlying unique numerical
// representation
type SymName = String;
type IdSy = u32;
type IdTm = u32;
type IdFo = u32;

#[derive(Debug)]
pub enum Term {
    Var(IdSy),
    //Cst(SId),
    //AFn(SId, Vec<TId>),
}

#[derive(Debug)]
pub enum Form {
    Top,
    Bot,

    Prd(IdSy, Vec<IdTm>),
    Equ(IdTm, IdTm),

    Neg(IdFo),
    Con(IdFo, IdFo),
    Dis(IdFo, IdFo),
    Imp(IdFo, IdFo),
    Iff(IdFo, IdFo),

    All(IdSy, IdFo),
    Ext(IdSy, IdFo),
}

#[derive(Debug, Default)]
pub struct FolFormArena {
    terms: Vec<Term>,
    forms: Vec<Form>,
}

impl FolFormArena {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_term(&mut self, t: Term) -> IdTm {
        let id = self.terms.len() as IdTm;
        self.terms.push(t);
        id
    }

    pub fn new_form(&mut self, f: Form) -> IdFo {
        let id = self.forms.len() as IdFo;
        self.forms.push(f);
        id
    }

    pub fn get_term(&self, id: IdTm) -> &Term {
        &self.terms[id as usize]
    }

    pub fn get_form(&self, id: IdFo) -> &Form {
        &self.forms[id as usize]
    }
}

#[derive(Debug)]
pub struct FolForm {
    root: Form,
    arena: FolFormArena,
}

fn lower_to_fol(sexpr: &SExpr) -> Result<FolForm, String> {
    let mut arena = FolFormArena::new();
    // DEBUG Fake root
    let mut root = Form::Top;

    Ok(FolForm { root, arena })
}

pub fn parse_sfol(input: &str) -> Result<FolForm, String> {
    let tokens = lex_to_tokens(input)?;
    let sexpr = parse_to_sexpr(&tokens)?;

    dbg_boxed!("{:?}", input);
    dbg_boxed!("{:?}", tokens);
    dbg_boxed!("{:?}", sexpr);

    let form = lower_to_fol(&sexpr)?;
    dbg_boxed!("{:?}", form);

    Ok(form)
}
// ===============================||
// ------------------------ Tests |>
// ===============================||

// Test Lexer
#[cfg(test)]
fn fixture_form_1() -> String {
    format!(
        "({exists} (x1) ({and} (P x2) x3))",
        exists = sy::EXISTS,
        and = sy::CON,
    )
}

#[cfg(test)]
fn fixture_form_1_expected_tokens() -> Vec<Token> {
    vec![
        Token::LParen,
        Token::Atom(sy::EXISTS.into()),
        Token::LParen,
        Token::Atom("x1".into()),
        Token::RParen,
        Token::LParen,
        Token::Atom(sy::CON.into()),
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
    let tokens_produced = lex_to_tokens(&fixture_form_1()).unwrap();
    assert_eq!(tokens_produced, fixture_form_1_expected_tokens());
}

#[test]
fn lexer_fol_comments() {
    let input = format!(
        r#"
            ;; E x1. P x1 /\ x2
            ;; There exists x1 such that P x1 and open x2 are true

            {} ;; This comment goes to eol
            "#,
        fixture_form_1(),
    );

    let tokens_produced = lex_to_tokens(&input).unwrap();
    assert_eq!(tokens_produced, fixture_form_1_expected_tokens());
}

#[test]
fn lexer_parentheses_without_whitespace() {
    let tokens_produced = lex_to_tokens("(Father(zorlak))").unwrap();

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
        lex_to_tokens("(ßsome ger\r\tman utf--8café λx\t\r 日本語 \r\r)")
            .unwrap();

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

// Test Parser
#[cfg(test)]
fn fixture_form_2() -> String {
    format!("({and} (P x) (not (Q y)))", and = sy::CON,)
}

#[cfg(test)]
fn fixture_form_2_expected_sexpr() -> SExpr {
    SExpr::List(vec![
        SExpr::Atom("and".into()),
        SExpr::List(vec![SExpr::Atom("P".into()), SExpr::Atom("x".into())]),
        SExpr::List(vec![
            SExpr::Atom("not".into()),
            SExpr::List(vec![SExpr::Atom("Q".into()), SExpr::Atom("y".into())]),
        ]),
    ])
}

#[test]
fn sexpr_nested_list() {
    let tokens_produced = lex_to_tokens(&fixture_form_2());
    let sexpr_produced = parse_to_sexpr(&tokens_produced.unwrap()).unwrap();
    assert_eq!(sexpr_produced, fixture_form_2_expected_sexpr());
}

#[test]
fn sexpr_rejects_unwrapped_list() {
    let tokens_produced =
        lex_to_tokens(&format!("{and} (P x) (Q y)", and = sy::CON)).unwrap();
    let sexpr = parse_to_sexpr(&tokens_produced);
    assert!(sexpr.is_err());
}

#[test]
fn sexpr_rejects_unclosed_list() {
    let tokens_produced =
        lex_to_tokens(&format!("({or} (P x) (Q y)", or = sy::DIS)).unwrap();
    let sexpr = parse_to_sexpr(&tokens_produced);
    assert!(sexpr.is_err());
}

#[test]
fn sexpr_rejects_extras() {
    let tokens_produced =
        lex_to_tokens(&format!("({and} (P x) (Q y)) hi", and = sy::CON))
            .unwrap();
    let sexpr = parse_to_sexpr(&tokens_produced);
    assert!(sexpr.is_err());
}

#[test]
fn sexpr_accepts_closed_list() {
    let tokens_produced =
        lex_to_tokens(&format!("({and} (P x) (Q y))", and = sy::CON)).unwrap();
    let sexpr = parse_to_sexpr(&tokens_produced);
    assert!(sexpr.is_ok());
}

#[test]
fn sexpr_accepts_nullary_predicate() {
    let tokens_produced = lex_to_tokens("P").unwrap();
    let sexpr = parse_to_sexpr(&tokens_produced);
    assert!(sexpr.is_ok());
}

#[test]
fn sexpr_accepts_nary_operators() {
    let tokens_produced = lex_to_tokens(&format!(
        "({and} (Parent x) (or (Mother x) (Father y) (Widowed x) (Widowed y)) (Child z))",
        and = sy::CON
    ))
    .unwrap();
    let sexpr = parse_to_sexpr(&tokens_produced);
    assert!(sexpr.is_ok());
}
