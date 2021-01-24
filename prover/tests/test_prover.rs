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
        .add_op(&IKosVariable4P::new_value(input_pub[0]))
        .add_op(&input[2].clone())
        .add_op(&input[3].clone());
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
        .add_op(&IKosVariable4V::new_value(input_pub[0]))?
        .add_op(&input[2].clone())?
        .add_op(&input[3].clone())?;
    Ok(vec![out])
}

#[test]
fn test_prove() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![97, 107, 10, 2];
    let in_pub = vec![15];
    let res = zk_boo.prove(ProvingProof::new(input, in_pub, 1, circuit));
    assert!(res.is_ok())
}

#[test]
fn test_proving_challenge() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![97, 107, 10, 2];
    let in_pub = vec![15];
    let res = zk_boo
        .prove(ProvingProof::new(input, in_pub, 1, circuit))
        .unwrap();
    let challenge = ZkBoo::query_random_oracle(
        res.input_len,
        res.output_len,
        &res.out_data,
        &res.three_views,
    );
    assert_eq!(challenge.len(), 32);
}

#[test]
fn test_proving_build() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![1, 2, 3, 4];
    let in_pub = vec![5];
    let mut res = zk_boo
        .prove(ProvingProof::new(input, in_pub, 1, circuit))
        .unwrap();
    let challenge = ZkBoo::query_random_oracle(
        res.input_len,
        res.output_len,
        &res.out_data,
        &res.three_views,
    );
    let two_views = zk_boo.rebuild_proof(&mut res, &challenge);
    assert_eq!(two_views.len(), 64);
}

#[test]
fn test_proving_verifying() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![97, 127, 10, 2];
    let in_pub = vec![15];
    let out = vec![97 + 127 + 15 + 10 + 2];
    let mut res = zk_boo
        .prove(ProvingProof::new(
            input.clone(),
            in_pub.clone(),
            out.len(),
            circuit,
        ))
        .unwrap();
    assert_eq!(res.output, out);
    let challenge = ZkBoo::query_random_oracle(
        res.input_len,
        res.output_len,
        &res.out_data,
        &res.three_views,
    );
    let response = zk_boo.build_response(&res.views, &challenge);
    let two_views = zk_boo.rebuild_proof(&mut res, &challenge);
    let res = zk_boo.verify(VerifyingProof::new(
        input.len(),
        in_pub,
        out,
        challenge,
        two_views,
        response,
        circuit_verifying,
    ));
    assert!(res.unwrap());
}

fn circuit_for_loop(input: &[IKosVariable4P], input_pub: &[u32]) -> Vec<IKosVariable4P> {
    let mut out = IKosVariable4P::new_value(0);
    for _ in 0..input_pub[0] {
        out = out.add_op(&input[0].clone());
    }
    vec![out]
}

fn circuit_for_loop_verifying(
    input: &[IKosVariable4V],
    input_pub: &[u32],
) -> IKosResult<Vec<IKosVariable4V>> {
    let mut out = IKosVariable4V::new_value(0);
    for _ in 0..input_pub[0] {
        out = out.add_op(&input[0].clone())?;
    }
    Ok(vec![out])
}

#[test]
fn test_proving_verifying_loop() {
    let zk_boo = ZkBoo::new(2, 3, 2, 32);
    let input = vec![97];
    let in_pub = vec![107];
    let out = vec![97 * 107];
    let mut res = zk_boo
        .prove(ProvingProof::new(
            input.clone(),
            in_pub.clone(),
            out.len(),
            circuit_for_loop,
        ))
        .unwrap();
    assert_eq!(res.output, out);
    let challenge = ZkBoo::query_random_oracle(
        res.input_len,
        res.output_len,
        &res.out_data,
        &res.three_views,
    );
    let response = zk_boo.build_response(&res.views, &challenge);
    let two_views = zk_boo.rebuild_proof(&mut res, &challenge);
    let res = zk_boo.verify(VerifyingProof::new(
        input.len(),
        in_pub,
        out,
        challenge,
        two_views,
        response,
        circuit_for_loop_verifying,
    ));
    assert!(res.unwrap());
}
