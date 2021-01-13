use zoker_prover::vector::_3DVector;

#[test]
fn test_3d_vector() {
    let a = _3DVector::new(10, 10, 10);
    assert_eq!(a.get_index(0, 2, 4), 24);
}
