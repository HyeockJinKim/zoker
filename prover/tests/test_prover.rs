use zoker_prover::ikos::IKosVariable4P;

#[test]
fn test_prover() {
    let a = IKosVariable4P::new();
    let b = IKosVariable4P::new();
    a.clone().bit_and(&b).bit_and(&a);
}
