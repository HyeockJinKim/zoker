use crate::ikos::{
    get_next_random_from_context, IKosContext, IKosResult, IKosVariable4P, IKosVariable4V, IKosView,
};
use crate::vector::_3DVector;

fn find_shares(input: u32, ctx: &mut Vec<IKosContext>) -> IKosResult<IKosVariable4P> {
    // TODO: 원래는 input이 u8이었음 ;;
    let mut shares = vec![0; 3];
    shares[0] = 0xFFFFFF & get_next_random_from_context(&mut ctx[0])?;
    shares[1] = 0xFFFFFF & get_next_random_from_context(&mut ctx[0])?;
    shares[2] = input ^ shares[0] ^ shares[1];
    Ok(IKosVariable4P::new_share(shares, ctx.to_owned()))
}

type Circuit4P = fn(&Vec<IKosVariable4P>, &Vec<u32>) -> Vec<IKosVariable4P>;
type Circuit4V = fn(&Vec<IKosVariable4V>, &Vec<u32>) -> Vec<IKosVariable4V>;
type ThreeViews = Vec<Vec<Vec<u8>>>;

pub struct Proof {
    input_len: usize,
    output_len: usize,
    out_data: Vec<u32>,
    three_views: ThreeViews,
    views: Vec<IKosView>,
}

pub struct ProvingProof {
    pub input: Vec<u32>,
    pub input_pub: Vec<u32>,
    pub output: Vec<u32>,
    circuit: Circuit4P,
}

pub struct VerifyingProof {
    input: Vec<u32>,
    input_pub: Vec<u32>,
    output: Vec<u32>,
    challenge: Vec<u8>,
    response: Vec<String>,
    circuit: Circuit4V,
}

fn get_rand_tape_len(input_len: usize) -> usize {
    (input_len + 511) / 512 * 728 * 32
}

pub fn prove(proof: ProvingProof) -> IKosResult<Proof> {
    let mut vec_view = _3DVector::new(proof.output.len(), 2, 3);
    let mut three_views = vec![vec![vec![0; 32]; 3]; 2];
    let mut views = vec![];
    let rand_tape_len = get_rand_tape_len(proof.input.len());

    for round in 0..2 {
        let mut ctx = vec![];
        let mut ikos_input = vec![];

        for _ in 0..3 {
            ctx.push(IKosContext::new(rand_tape_len, false));
        }

        for i in 0..proof.input.len() {
            ikos_input.push(find_shares(proof.input[i], &mut ctx)?);
            ctx[2].ikos_view.in_data.push(ikos_input[i].value[2]);
        }

        // Circuit 실행
        let ikos_output: Vec<IKosVariable4P> = proof.run_circuit(&ikos_input, &proof.input_pub);

        // ikos output 저장
        for party in 0..3 {
            for i in 0..ikos_output.len() {
                let index = vec_view.get_index(i, round, party);
                vec_view.data[index] = ikos_output[i].value[party];
                ctx[party]
                    .ikos_view
                    .out_data32
                    .push(ikos_output[i].value[party]);
            }
        }

        // commitment
        for party in 0..3 {
            three_views[round][party] = ctx[party].commit_ikos_context();
        }
        views.extend(ctx.iter().map(|c| c.ikos_view.clone()));
    }
    // No Check output

    Ok(Proof::new(
        proof.input.len(),
        proof.input_pub.len(),
        vec_view.data,
        three_views,
        views,
    ))
}

pub fn verify(proof: VerifyingProof) {
    //
}

impl Proof {
    fn new(
        input_len: usize,
        output_len: usize,
        out_data: Vec<u32>,
        three_views: ThreeViews,
        views: Vec<IKosView>,
    ) -> Self {
        Proof {
            input_len,
            output_len,
            out_data,
            three_views,
            views,
        }
    }
}

impl ProvingProof {
    pub fn new(input: Vec<u32>, input_pub: Vec<u32>, output: Vec<u32>, circuit: Circuit4P) -> Self {
        ProvingProof {
            input,
            input_pub,
            output,
            circuit,
        }
    }

    fn run_circuit(
        &self,
        ikos_input: &Vec<IKosVariable4P>,
        input_pub: &Vec<u32>,
    ) -> Vec<IKosVariable4P> {
        (self.circuit)(ikos_input, input_pub)
    }
}
