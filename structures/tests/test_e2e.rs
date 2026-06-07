use structures::primitives::Atom;

#[test]
fn assert_trivial() {
    let s = Atom::from(0);
    assert!(s < 1.into());
}
