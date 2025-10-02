use imgal::kernel::neighborhood;

// kernel parameters
const RADIUS: usize = 5;
const FALLOFF_RADIUS: f64 = 7.0;

#[test]
fn neighborhood_circle() {
    // create a circle neighborhood kernel
    let k = neighborhood::circle(RADIUS).unwrap();

    assert_eq!(k.shape(), [11, 11]);
    assert_eq!(k[[RADIUS, RADIUS]], true);
    assert_eq!(k[[8, 1]], true);
    assert_eq!(k[[2, 0]], false);
}

#[test]
fn neighborhood_sphere() {
    // create a sphere neighborhood kernel
    let k = neighborhood::sphere(RADIUS).unwrap();

    assert_eq!(k.shape(), [11, 11, 11]);
    assert_eq!(k[[RADIUS, RADIUS, RADIUS]], true);
    assert_eq!(k[[2, 5, 1]], true);
    assert_eq!(k[[8, 9, 10]], false);
}

#[test]
fn neighborhood_weighted_circle() {
    // create a weighted circle neighborhood kernel
    let k = neighborhood::weighted_circle(RADIUS, FALLOFF_RADIUS, None).unwrap();

    assert_eq!(k.shape(), [11, 11]);
    assert_eq!(k[[RADIUS, RADIUS]], 1.0);
    assert_eq!(k[[8, 1]], 0.2857142857142857);
    assert_eq!(k[[2, 0]], 0.0);
}

#[test]
fn neighborhood_weighted_sphere() {
    // create a weighted sphere neighborhood kernel
    let k = neighborhood::weighted_sphere(RADIUS, FALLOFF_RADIUS, None).unwrap();

    assert_eq!(k.shape(), [11, 11, 11]);
    assert_eq!(k[[RADIUS, RADIUS, RADIUS]], 1.0);
    assert_eq!(k[[2, 5, 1]], 0.2857142857142857);
    assert_eq!(k[[8, 9, 10]], 0.0);
}
