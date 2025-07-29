use imgal_core::parameters;

#[test]
fn abbe_diffraction_limit() {
    let l = parameters::abbe_diffraction_limit(570, 1.45);
    assert_eq!(l, 196.55172413793105)
}

#[test]
fn omega() {
    // test with 12.5 nanoseconds
    let w = parameters::omega(1.25e-8);
    assert_eq!(w, 502654824.5743669)
}
