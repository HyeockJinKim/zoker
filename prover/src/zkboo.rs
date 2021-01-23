use crate::ikos::{
    get_next_random_from_context, IKosContext, IKosError, IKosResult, IKosVariable4P,
    IKosVariable4V, IKosView,
};
use crate::utils::{convert_usize_to_u8, convert_vec_to_u8};
use crate::vector::_3DVector;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

fn find_shares(input: u32, ctx: Rc<RefCell<Vec<IKosContext>>>) -> IKosResult<IKosVariable4P> {
    // TODO: 원래는 input이 u8이었음
    let shares = {
        let mut contexts = (*ctx).borrow_mut();
        let share0 = get_next_random_from_context(&mut contexts[0])?;
        let share1 = get_next_random_from_context(&mut contexts[1])?;
        vec![share0, share1, input ^ share0 ^ share1]
    };

    Ok(IKosVariable4P::new_share(shares, ctx))
}

type Circuit4P = fn(&[IKosVariable4P], &[u32]) -> Vec<IKosVariable4P>;
type Circuit4V = fn(&[IKosVariable4V], &[u32]) -> IKosResult<Vec<IKosVariable4V>>;

pub struct ZkBoo {
    num_of_round: usize,
    num_of_branch: usize,
    num_of_public_branch: usize,
    commit_length: usize,
}

pub struct Proof {
    pub input_len: usize,
    pub output_len: usize,
    pub output: Vec<u32>,
    pub out_data: Vec<u32>,
    pub three_views: Vec<u8>,
    pub views: Vec<IKosView>,
}

pub struct ProvingProof {
    input: Vec<u32>,
    input_pub: Vec<u32>,
    output_len: usize,
    circuit: Circuit4P,
}

pub struct VerifyingProof {
    input_len: usize,
    input_pub: Vec<u32>,
    output: Vec<u32>,
    challenge: Vec<u8>,
    two_views: Vec<u8>,
    response: Vec<IKosView>,
    circuit: Circuit4V,
}

fn get_rand_tape_len(input_len: usize) -> usize {
    (input_len + 511) / 512 * 728 * 64
}

impl ZkBoo {
    pub fn new(
        num_of_round: usize,
        num_of_branch: usize,
        num_of_public_branch: usize,
        commit_length: usize,
    ) -> Self {
        ZkBoo {
            num_of_round,
            num_of_branch,
            num_of_public_branch,
            commit_length,
        }
    }

    pub fn prove(&self, proof: ProvingProof) -> IKosResult<Proof> {
        let mut vec_view = _3DVector::new(proof.output_len, self.num_of_round, self.num_of_branch);
        let mut three_views = vec![0; self.num_of_round * self.num_of_branch * self.commit_length];
        let mut views = vec![];
        let rand_tape_len = get_rand_tape_len(proof.input.len());
        for round in 0..self.num_of_round {
            let mut ctx = vec![];
            let mut ikos_input = vec![];

            for _ in 0..self.num_of_branch {
                ctx.push(IKosContext::new(rand_tape_len));
            }

            let ctx = Rc::new(RefCell::new(ctx));
            for i in 0..proof.input.len() {
                ikos_input.push(find_shares(proof.input[i], Rc::clone(&ctx))?);
                let mut contexts = (*ctx).borrow_mut();
                contexts[2].ikos_view.in_data.push(ikos_input[i].value[2]);
            }
            // Circuit 실행
            let ikos_output: Vec<IKosVariable4P> = proof.run_circuit(&ikos_input, &proof.input_pub);

            // ikos output 저장
            for (party, context) in (*ctx).borrow_mut().iter_mut().enumerate() {
                for (i, ikos) in ikos_output.iter().enumerate() {
                    let index = vec_view.get_index(i, round, party);
                    vec_view.data[index] = ikos.value[party];
                    context.ikos_view.out_data.push(ikos.value[party]);
                }
            }
            // commitment
            for party in 0..self.num_of_branch {
                let commit = (*ctx).borrow_mut()[party].commit_ikos_context();
                for (i, &cmt) in commit.iter().enumerate() {
                    three_views[round * self.num_of_branch * self.commit_length
                        + party * self.commit_length
                        + i] = cmt;
                }
            }
            views.extend((*ctx).borrow_mut().iter().map(|c| c.ikos_view.clone()));
        }
        // 계산 결과 구하기
        let mut out = vec![];
        for i in 0..proof.output_len {
            out.push(
                vec_view.data[vec_view.get_index(i, 0, 0)]
                    ^ vec_view.data[vec_view.get_index(i, 0, 1)]
                    ^ vec_view.data[vec_view.get_index(i, 0, 2)],
            )
        }
        Ok(Proof::new(
            proof.input.len(),
            proof.input_pub.len(),
            vec_view.data,
            out,
            three_views,
            views,
        ))
    }

    pub fn verify(&self, proof: VerifyingProof) -> IKosResult<bool> {
        let index_vec = self.choose_index_from_challenge(&*proof.challenge);
        let rand_tape_len = get_rand_tape_len(proof.input_len);
        let mut vec_view =
            _3DVector::new(proof.output.len(), self.num_of_round, self.num_of_branch);
        let mut three_views = vec![0; self.num_of_round * self.num_of_branch * self.commit_length];
        let mut three_offset = 0;
        let mut two_offset = 0;
        for (round, &index) in index_vec.iter().enumerate().take(self.num_of_round) {
            let mut ctx = vec![];
            for party in 0..self.num_of_public_branch {
                ctx.push(IKosContext::new_views(
                    proof.response[round * self.num_of_public_branch + party].clone(),
                    rand_tape_len,
                ));
            }
            match index_vec[round] {
                0 => {
                    for _ in 0..proof.input_len {
                        let data = get_next_random_from_context(&mut ctx[0])?;
                        ctx[0].ikos_view.in_data.push(data);
                        ctx[1].ikos_view.in_data.push(data);
                    }
                }
                1 => {
                    for _ in 0..proof.input_len {
                        let data = get_next_random_from_context(&mut ctx[1])?;
                        ctx[0].ikos_view.in_data.push(data);
                        ctx[1].ikos_view.in_data.push(data);
                    }
                }
                2 => {
                    for _ in 0..proof.input_len {
                        let data = get_next_random_from_context(&mut ctx[0])?;
                        ctx[0].ikos_view.in_data.push(data);
                        let data = get_next_random_from_context(&mut ctx[1])?;
                        ctx[1].ikos_view.in_data.push(data);
                    }
                }
                _ => {
                    return Err(IKosError {
                        error: String::from("index error"),
                    });
                }
            }

            let ctx = Rc::new(RefCell::new(ctx));
            // input
            let mut ikos_input = vec![];
            for i in 0..proof.input_len {
                let mut shares = vec![];
                let contexts = (*ctx).borrow();
                for context in contexts.iter().take(self.num_of_public_branch) {
                    shares.push(context.ikos_view.in_data[i]);
                }
                ikos_input.push(IKosVariable4V::new_share(shares, Rc::clone(ctx.borrow())));
            }
            // rut circuit
            let ikos_out = proof.run_circuit(&ikos_input, &proof.input_pub)?;

            let required = IKosVariable4V::require_reconstruct(&**(*ctx).borrow());
            let mut contexts = (*ctx).borrow_mut();
            for (branch, context) in contexts
                .iter_mut()
                .enumerate()
                .take(self.num_of_public_branch)
            {
                for ikos in &ikos_out {
                    if !required || branch != 0 {
                        if ikos.value[branch] != context.ikos_view.out_data[context.out_view_ctr] {
                            return Err(IKosError {
                                error: String::from("verify output value error"),
                            });
                        }
                    } else {
                        context.ikos_view.out_data.push(ikos.value[branch]);
                        context.out_view_ctr += 1;
                    }
                }
            }

            // construct three views
            match index {
                0 => {
                    three_views[three_offset..(three_offset + self.commit_length)]
                        .clone_from_slice(
                            &proof.two_views[two_offset..(two_offset + self.commit_length)],
                        );
                    three_offset += self.commit_length;
                    two_offset += self.commit_length;

                    let commit = contexts[0].commit_ikos_context();
                    three_views[three_offset..(three_offset + self.commit_length)]
                        .clone_from_slice(&commit);
                    three_offset += self.commit_length;

                    let commit = contexts[1].commit_ikos_context();
                    three_views[three_offset..(three_offset + self.commit_length)]
                        .clone_from_slice(&commit);
                    three_offset += self.commit_length;
                }
                1 => {
                    let commit = contexts[1].commit_ikos_context();
                    three_views[three_offset..(three_offset + self.commit_length)]
                        .clone_from_slice(&commit);
                    three_offset += self.commit_length;
                    three_views[three_offset..(three_offset + self.commit_length)]
                        .clone_from_slice(
                            &proof.two_views[two_offset..(two_offset + self.commit_length)],
                        );
                    three_offset += self.commit_length;
                    two_offset += self.commit_length;
                    let commit = contexts[0].commit_ikos_context();
                    three_views[three_offset..(three_offset + self.commit_length)]
                        .clone_from_slice(&commit);
                    three_offset += self.commit_length;
                }
                2 => {
                    let commit = contexts[0].commit_ikos_context();
                    three_views[three_offset..(three_offset + self.commit_length)]
                        .clone_from_slice(&commit);
                    three_offset += self.commit_length;
                    let commit = contexts[1].commit_ikos_context();
                    three_views[three_offset..(three_offset + self.commit_length)]
                        .clone_from_slice(&commit);
                    three_offset += self.commit_length;

                    three_views[three_offset..(three_offset + self.commit_length)]
                        .clone_from_slice(
                            &proof.two_views[two_offset..(two_offset + self.commit_length)],
                        );
                    three_offset += self.commit_length;
                    two_offset += self.commit_length;
                }
                _ => {
                    return Err(IKosError {
                        error: String::from("verify index error"),
                    });
                }
            }

            // rebuild shares
            for (i, ikos) in ikos_out.iter().enumerate() {
                let pos = vec_view.get_index(i, round, 0);
                match index {
                    0 => {
                        vec_view.data[pos + 1] = ikos.value[0];
                        vec_view.data[pos + 2] = ikos.value[1];
                        vec_view.data[pos] =
                            proof.output[i] ^ vec_view.data[pos + 2] ^ vec_view.data[pos + 1];
                    }
                    1 => {
                        vec_view.data[pos] = ikos.value[1];
                        vec_view.data[pos + 2] = ikos.value[0];
                        vec_view.data[pos + 1] =
                            proof.output[i] ^ vec_view.data[pos] ^ vec_view.data[pos + 2];
                    }
                    2 => {
                        vec_view.data[pos] = ikos.value[0];
                        vec_view.data[pos + 1] = ikos.value[1];
                        vec_view.data[pos + 2] =
                            proof.output[i] ^ vec_view.data[pos + 1] ^ vec_view.data[pos];
                    }
                    _ => {
                        return Err(IKosError {
                            error: String::from("verify index error"),
                        });
                    }
                }
            }
        }
        let mut out = vec![];
        for i in 0..proof.output.len() {
            out.push(
                vec_view.data[vec_view.get_index(i, 0, 0)]
                    ^ vec_view.data[vec_view.get_index(i, 0, 1)]
                    ^ vec_view.data[vec_view.get_index(i, 0, 2)],
            )
        }

        for (i, &cmt) in ZkBoo::query_random_oracle(
            proof.input_len,
            proof.output.len(),
            &vec_view.data,
            &three_views,
        )
        .iter()
        .enumerate()
        {
            if proof.challenge[i] != cmt {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn query_random_oracle(
        input_len: usize,
        output_len: usize,
        out_data: &[u32],
        three_views: &[u8],
    ) -> Vec<u8> {
        let mut sha = Sha256::new();
        sha.input(convert_usize_to_u8(input_len).as_ref());
        sha.input(convert_usize_to_u8(output_len).as_ref());
        sha.input(convert_vec_to_u8::<u32>(&out_data).as_ref());
        sha.input(convert_vec_to_u8::<u8>(&three_views).as_ref());
        sha.result_str().as_bytes().to_vec()
    }

    fn choose_index_from_challenge(&self, commit: &[u8]) -> Vec<usize> {
        let mut res = vec![];
        let mut val = 0;
        for &commit in commit.iter().take(4) {
            val = val as usize * 16 + commit as usize;
        }

        for _ in 0..self.num_of_round {
            res.push(val % self.num_of_branch);
            val /= self.num_of_branch;
        }
        res
    }

    fn discard_one_view(&self, three_views: &[u8], index_vec: Vec<usize>) -> Vec<u8> {
        let mut res = vec![];
        for (round, &index) in index_vec.iter().enumerate().take(self.num_of_round) {
            let offset =
                round * self.num_of_branch * self.commit_length + index * self.commit_length;
            for i in 0..self.commit_length {
                res.push(three_views[offset + i]);
            }
        }
        res
    }

    pub fn build_response(&self, views: &[IKosView], challenge: &[u8]) -> Vec<IKosView> {
        let index_vec = self.choose_index_from_challenge(challenge);
        let mut res = vec![];
        for (round, &index) in index_vec.iter().enumerate().take(self.num_of_round) {
            match index {
                0 => {
                    res.push(views[round * self.num_of_branch + 1].clone());
                    res.push(views[round * self.num_of_branch + 2].clone());
                }
                1 => {
                    res.push(views[round * self.num_of_branch + 2].clone());
                    res.push(views[round * self.num_of_branch].clone());
                }
                2 => {
                    res.push(views[round * self.num_of_branch].clone());
                    res.push(views[round * self.num_of_branch + 1].clone());
                }
                _ => {}
            }
        }
        res
    }

    pub fn rebuild_proof(&self, proof: &mut Proof, commit: &[u8]) -> Vec<u8> {
        let index_vec = self.choose_index_from_challenge(commit);
        self.discard_one_view(&proof.three_views, index_vec)
    }
}

impl Proof {
    fn new(
        input_len: usize,
        output_len: usize,
        out_data: Vec<u32>,
        output: Vec<u32>,
        three_views: Vec<u8>,
        views: Vec<IKosView>,
    ) -> Self {
        Proof {
            input_len,
            output_len,
            out_data,
            output,
            three_views,
            views,
        }
    }
}

impl ProvingProof {
    pub fn new(
        input: Vec<u32>,
        input_pub: Vec<u32>,
        output_len: usize,
        circuit: Circuit4P,
    ) -> Self {
        ProvingProof {
            input,
            input_pub,
            output_len,
            circuit,
        }
    }

    fn run_circuit(&self, ikos_input: &[IKosVariable4P], input_pub: &[u32]) -> Vec<IKosVariable4P> {
        (self.circuit)(ikos_input, input_pub)
    }
}

impl VerifyingProof {
    pub fn new(
        input_len: usize,
        input_pub: Vec<u32>,
        output: Vec<u32>,
        challenge: Vec<u8>,
        two_views: Vec<u8>,
        response: Vec<IKosView>,
        circuit: Circuit4V,
    ) -> Self {
        VerifyingProof {
            input_len,
            input_pub,
            output,
            challenge,
            two_views,
            response,
            circuit,
        }
    }

    fn run_circuit(
        &self,
        ikos_input: &[IKosVariable4V],
        input_pub: &[u32],
    ) -> IKosResult<Vec<IKosVariable4V>> {
        (self.circuit)(ikos_input, input_pub)
    }
}
