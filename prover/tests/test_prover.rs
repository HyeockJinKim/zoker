use zoker_prover::ikos::{IKosResult, IKosVariable4P, IKosVariable4V};
use zoker_prover::zkboo::{ProvingProof, VerifyingProof, ZkBoo};

#[test]
fn test_ikos_for_prover() {
    let a = IKosVariable4P::new_value(1);
    let b = IKosVariable4P::new_value(2);
    a.clone().bit_and(&b).bit_and(&a);
}

fn circuit(input: &[IKosVariable4P], input_pub: &[u32]) -> Vec<IKosVariable4P> {
    let a = input[0].clone();
    let out = a
        .add_op(&input[1].clone())
        .add_op(&IKosVariable4P::new_value(input_pub[0]));
    vec![out]
}

fn circuit_verifying(
    input: &[IKosVariable4V],
    input_pub: &[u32],
) -> IKosResult<Vec<IKosVariable4V>> {
    let a = input[0].clone();
    let out = a
        .clone()
        .add_op(&input[1].clone())?
        .add_op(&IKosVariable4V::new_value(input_pub[0]))?;
    Ok(vec![out])
}

#[test]
fn test_prove() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![97, 107];
    let in_pub = vec![15];
    let out = vec![97 & 107 & 15];
    let res = zk_boo.prove(ProvingProof::new(input, in_pub, out, circuit));
    assert!(res.is_ok())
}

#[test]
fn test_proving_challenge() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![97, 100];
    let in_pub = vec![15];
    let out = vec![97 + 100 + 100];
    let res = zk_boo.prove(ProvingProof::new(input, in_pub, out, circuit));
    let challenge = ZkBoo::query_random_oracle(&res.unwrap());
    assert_eq!(
        challenge,
        vec![
            100, 51, 101, 100, 55, 101, 54, 51, 50, 51, 50, 49, 54, 100, 56, 50, 57, 99, 99, 51,
            100, 100, 55, 52, 52, 100, 99, 49, 52, 56, 51, 102, 54, 51, 55, 98, 101, 53, 57, 57,
            52, 51, 49, 49, 56, 48, 54, 99, 48, 51, 57, 54, 51, 101, 56, 54, 51, 53, 54, 55, 57,
            101, 48, 99
        ]
    );
}

#[test]
fn test_proving_build() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![1, 3];
    let in_pub = vec![5];
    let out = vec![1 + 3 + 5];
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
            48, 99, 100, 99, 56, 50, 98, 51, 49, 51, 50, 100, 101, 54, 50, 98, 52, 50, 48, 102, 99,
            49, 97, 50, 53, 52, 57, 55, 98, 98, 55, 56, 49, 48, 98, 99, 55, 50, 100, 101, 100, 54,
            101, 55, 56, 54, 99, 55, 52, 49, 97, 50, 56, 98, 99, 98, 49, 48, 98, 57, 56, 98, 57,
            101, 102, 98, 99, 100, 98, 52, 100, 102, 49, 101, 52, 54, 57, 99, 53, 102, 50, 101, 49,
            49, 52, 99, 56, 97, 100, 100, 98, 51, 56, 53, 98, 51, 99, 51, 99, 100, 97, 102, 98, 55,
            56, 51, 100, 98, 101, 56, 97, 57, 53, 100, 51, 97, 100, 100, 100, 49, 97, 100, 50, 48,
            97, 100, 53, 51
        ]
    );
}

#[test]
fn test_proving_verifying() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![97, 127];
    let in_pub = vec![15];
    let out = vec![97 + 127 + 13];
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

    assert!(res.unwrap());
}
