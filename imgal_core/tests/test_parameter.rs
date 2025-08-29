use imgal_core::parameter;

#[test]
fn parameter_abbe_diffraction_limit() {
    let l = parameter::abbe_diffraction_limit(570, 1.45);
    assert_eq!(l, 196.55172413793105)
}

#[test]
fn parameter_omega() {
    // test with 12.5 nanoseconds
    let w = parameter::omega(12.5);
    assert_eq!(w, 0.5026548245743669)
}
