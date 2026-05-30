use structures::primitives::SAtom;

#[test]
fn assert_trivial() {
    let s = SAtom::from(0);
    assert!(s < 1.into());
}
