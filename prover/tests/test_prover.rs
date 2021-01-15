use zoker_prover::ikos::IKosVariable4P;
use zoker_prover::zkboo::{prove, ProvingProof};

#[test]
fn test_ikos_for_prover() {
    let a = IKosVariable4P::new_value(1);
    let b = IKosVariable4P::new_value(2);
    println!("{:#?}", a);
    println!("{:#?}", a.clone().bit_and(&b).bit_and(&a));
    println!("{:#?}", a);

    println!("{:#?}", a.clone().negate());
    println!("{:#?}", a);
}

fn circuit(input: &Vec<IKosVariable4P>, input_pub: &Vec<u32>) -> Vec<IKosVariable4P> {
    let out = input[0]
        .clone()
        .add(&input[1].clone())
        .add(&IKosVariable4P::new_value(input_pub[0]));
    vec![out]
}

#[test]
fn test_prove() {
    let input = vec![97, 107];
    let in_pub = vec![15];
    let out = vec![97 + 107 + 15];
    let res = prove(ProvingProof::new(input, in_pub, out, circuit));
    assert!(res.is_ok())
}
