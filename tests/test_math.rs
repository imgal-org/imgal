use imgal::math::sum;

#[test]
fn test_sum_integers() {
    let data: [i32; 4] = [2, 5, 10, 23];
    assert_eq!(sum(&data), 40);
}
