use zoker_prover::ikos::{IKosResult, IKosVariable4P, IKosVariable4V};
use zoker_prover::zkboo::{ProvingProof, VerifyingProof, ZkBoo};

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
    let out = input[0].clone().bit_and(&input[1].clone());
    vec![out]
}

fn circuit_verifying(
    input: &Vec<IKosVariable4V>,
    input_pub: &Vec<u32>,
) -> IKosResult<Vec<IKosVariable4V>> {
    let out = input[0].clone().bit_and(&input[1].clone())?;
    Ok(vec![out])
}

#[test]
fn test_prove() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![97, 107];
    let in_pub = vec![15];
    let out = vec![97 + 107 + 15];
    let res = zk_boo.prove(ProvingProof::new(input, in_pub, out, circuit));
    assert!(res.is_ok())
}

#[test]
fn test_proving_challenge() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![97, 107];
    let in_pub = vec![15];
    let out = vec![97 + 107 + 15];
    let res = zk_boo.prove(ProvingProof::new(input, in_pub, out, circuit));
    let challenge = ZkBoo::query_random_oracle(&res.unwrap());
    assert_eq!(
        challenge,
        vec![
            48, 51, 98, 101, 55, 54, 98, 98, 57, 102, 97, 50, 51, 98, 55, 51, 53, 53, 49, 57, 101,
            48, 57, 53, 48, 48, 56, 100, 48, 99, 100, 53, 49, 102, 56, 56, 49, 51, 54, 53, 101, 52,
            50, 56, 100, 102, 50, 102, 52, 50, 51, 48, 101, 97, 101, 57, 97, 52, 55, 56, 51, 54,
            99, 56
        ]
    );
}

#[test]
fn test_proving_build() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![1, 3];
    let in_pub = vec![5];
    let out = vec![1];
    let mut res = zk_boo
        .prove(ProvingProof::new(input, in_pub, out, circuit))
        .unwrap();
    let challenge = ZkBoo::query_random_oracle(&res);
    let response = zk_boo.build_response(&res.views, &challenge);
    assert!(response.is_ok());
    let optimized = zk_boo.rebuild_proof(&mut res, &challenge);
    assert_eq!(
        optimized,
        vec![
            54, 57, 50, 99, 98, 99, 53, 100, 102, 101, 51, 102, 101, 49, 50, 48, 53, 49, 55, 100,
            100, 101, 54, 52, 98, 100, 48, 55, 99, 51, 102, 52, 55, 55, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        ]
    );
}

#[test]
fn test_proving_verifying() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![97, 107];
    let in_pub = vec![15];
    let out = vec![97 + 107 + 15];
    let mut res = zk_boo
        .prove(ProvingProof::new(
            input.clone(),
            in_pub.clone(),
            out.clone(),
            circuit,
        ))
        .unwrap();
    let challenge = ZkBoo::query_random_oracle(&res);
    let response = zk_boo.build_response(&res.views, &challenge).unwrap();
    zk_boo.rebuild_proof(&mut res, &challenge);
    let res = zk_boo.verify(VerifyingProof::new(
        input.len(),
        in_pub,
        out,
        challenge,
        response,
        circuit_verifying,
    ));

    println!("{:#?}", res.unwrap());
}
