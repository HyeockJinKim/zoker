use zoker_prover::ikos::IKosVariable4P;

#[test]
fn test_prover() {
    let a = IKosVariable4P::new_value(1);
    let b = IKosVariable4P::new_value(2);
    println!("{:#?}", a);
    println!("{:#?}", a.clone().bit_and(&b).bit_and(&a));
    println!("{:#?}", a);

    println!("{:#?}", a.clone().negate());
    println!("{:#?}", a);
}
