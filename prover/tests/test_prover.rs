use zoker_prover::ikos::{IKosResult, IKosVariable4P, IKosVariable4V};
use zoker_prover::zkboo::{ProvingProof, VerifyingProof, ZkBoo};

#[test]
fn test_ikos_for_prover() {
    let a = IKosVariable4P::new_value(1);
    let b = IKosVariable4P::new_value(2);
    a.clone().bit_and(&b).bit_and(&a);
}

fn circuit(input: &[IKosVariable4P], input_pub: &[u32]) -> Vec<IKosVariable4P> {
    let mut out = input[0].clone();
    for _ in 0..input_pub[0] {
        out = out.add_op(&input[1].clone());
    }
    vec![out]
}

fn circuit_verifying(
    input: &[IKosVariable4V],
    input_pub: &[u32],
) -> IKosResult<Vec<IKosVariable4V>> {
    let mut out = input[0].clone();
    for _ in 0..input_pub[0] {
        out = out.add_op(&input[1].clone())?;
    }
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
    let input = vec![97, 127];
    let in_pub = vec![80];
    let out = vec![97 + 127 * 80];
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

    print!("{:?},", input.len());
    print!("{:?},", in_pub);
    print!("{:?},\"0x", res.output);
    for &ch in &challenge.clone() {
        print!("{:02x?}", ch);
    }
    print!("\",\"0x");
    for &ch in &two_views.clone() {
        print!("{:02x?}", ch);
    }
    print!("\",[");
    for i in 0..3 {
        print!("[\"0x");
        for &ch in &response[i].rand_tape_seed.clone() {
            print!("{:02x?}", ch);
        }
        print!("\",");
        print!("{:?},{:?}],", response[i].in_data, response[i].out_data);
    }
    print!("[\"0x");
    for &ch in &response[3].rand_tape_seed.clone() {
        print!("{:02x?}", ch);
    }
    print!("\",");
    println!("{:?},{:?}]]", response[3].in_data, response[3].out_data);

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

/*
16
16
2,[1],[224],"0xf061ba143500446b4eefa2d285f77884e122374056110bb77c84c1f94e1b254d","0x29c90e4851e611fed9226683127da136f29e51b28fb0ee316415c0b5ad2e63cbea496f336c596754847e56fafe784f85f4caeb7e7e15e2982c3e32abd97f4d52",[["0xe85f3b868ff5e056655f014caff91674",[], [1101059858, 3331400255]],["0x598aed3df7c2c1462f047b21cb1e230f",[529823820, 121948633], [877898874, 746790383]],["0x24440c1478129fcd2a9ba2404f4f5e80",[], [187600280, 687236365]],["0xebedf05f42c8d67690422954549e369b",[], [2576801620, 2540859047]]]

2
1
[3927064368, 3331400255, 746790383, 687236365, 2540859047, 3213101898]
[41, 201, 14, 72, 81, 230, 17, 254, 217, 34, 102, 131, 18, 125, 161, 54, 242, 158, 81, 178, 143, 176, 238, 49, 100, 21, 192, 181, 173, 46, 99, 203, 161, 107, 248, 127, 79, 161, 117, 116, 11, 236, 208, 134, 92, 193, 35, 72, 103, 57, 211, 90, 74, 243, 109, 212, 218, 156, 133, 14, 58, 128, 14, 24, 188, 146, 8, 125, 95, 206, 83, 6, 194, 109, 206, 116, 77, 54, 162, 156, 234, 22, 202, 73, 80, 216, 17, 237, 220, 177, 5, 150, 236, 108, 139, 23, 223, 172, 158, 75, 65, 111, 59, 250, 177, 137, 10, 88, 107, 56, 120, 69, 168, 97, 160, 127, 89, 127, 203, 192, 39, 255, 63, 216, 123, 113, 103, 194, 117, 123, 63, 165, 143, 229, 123, 128, 101, 47, 56, 181, 254, 237, 42, 107, 50, 235, 64, 10, 197, 59, 118, 4, 63, 105, 64, 142, 220, 170, 132, 34, 234, 73, 111, 51, 108, 89, 103, 84, 132, 126, 86, 250, 254, 120, 79, 133, 244, 202, 235, 126, 126, 21, 226, 152, 44, 62, 50, 171, 217, 127, 77, 82]
[240, 97, 186, 20, 53, 0, 68, 107, 78, 239, 162, 210, 133, 247, 120, 132, 225, 34, 55, 64, 86, 17, 11, 183, 124, 132, 193, 249, 78, 27, 37, 77]
*/
