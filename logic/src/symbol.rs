#![allow(unused)]

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct IdTerm(u32);

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct IdForm(u32);

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Sym(u32);

impl Sym {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

#[derive(Debug)]
pub enum Term {
    Var(Sym),
    // Decided to keep all symbols bound
    // by some quantifier or unbound
    // hence no constants
    //Cst(Sym),
    // Removed as not needed
    // and hard to parse applied
    // pred vs applied fn
    //Apl(IdFn, IdArg),
}

#[derive(Debug)]
pub enum Form {
    // All evaluates to Top or Bot when closed
    Top,
    Bot,

    Equ(IdTerm, IdTerm),
    Prd(Sym, Vec<IdTerm>),

    Neg(IdForm),

    Imp(IdForm, IdForm),
    Iff(IdForm, IdForm),
    Con(Vec<IdForm>),
    Dis(Vec<IdForm>),

    Forall(Sym, IdForm),
    Exists(Sym, IdForm),
}

/// Keep all logical objects in arena for cache hit locality
#[derive(Default, Debug)]
pub struct FolArena {
    terms: Vec<Term>,
    forms: Vec<Form>,
}

impl FolArena {
    pub fn new() -> Self {
        Self {
            terms: vec![],
            forms: vec![],
        }
    }

    pub fn new_term(&mut self, t: Term) -> IdTerm {
        let id = IdTerm(self.terms.len() as u32);
        self.terms.push(t);
        id
    }

    pub fn new_form(&mut self, f: Form) -> IdForm {
        let id = IdForm(self.forms.len() as u32);
        self.forms.push(f);
        id
    }

    pub fn get_term(&self, id: IdTerm) -> &Term {
        &self.terms[id.0 as usize]
    }

    pub fn get_form(&self, id: IdForm) -> &Form {
        &self.forms[id.0 as usize]
    }
}

#[test]
fn test_var() {
    let mut arena = FolArena::new();

    let x = arena.new_term(Term::Var(Sym(0)));

    match arena.get_term(x) {
        Term::Var(Sym(id)) => assert_eq!(*id, 0),
        other => panic!("expected Var, got {:?}", other),
    }
}

#[test]
fn test_neg_dis_imp() {
    // ~(A \/ B) -> C
    // where A, B, C are nullary predicates
    let mut arena = FolArena::new();

    let a = arena.new_form(Form::Prd(Sym(0), vec![]));
    let b = arena.new_form(Form::Prd(Sym(1), vec![]));
    let c = arena.new_form(Form::Prd(Sym(2), vec![]));

    let dis = arena.new_form(Form::Dis(vec![a, b]));
    let neg = arena.new_form(Form::Neg(dis));
    let root = arena.new_form(Form::Imp(neg, c));

    match arena.get_form(root) {
        Form::Imp(_, _) => {}
        other => panic!("expected Imp, got {:?}", other),
    }
}

#[test]
fn test_exists_contradiction() {
    // E x1. P(x1) /\ ~P(x1)
    let mut arena = FolArena::new();

    let x1 = arena.new_term(Term::Var(Sym(0)));
    let px = arena.new_form(Form::Prd(Sym(1), vec![x1]));
    let npx = arena.new_form(Form::Neg(px));
    let con = arena.new_form(Form::Con(vec![px, npx]));
    let root = arena.new_form(Form::Exists(Sym(0), con));

    match arena.get_form(root) {
        Form::Exists(bound, _) => assert_eq!(bound.0, 0),
        other => panic!("expected Exists, got {:?}", other),
    }
}
