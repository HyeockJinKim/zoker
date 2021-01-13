use crate::ikos::{get_next_random_from_context, IKosContext, IKosResult, IKosVariable4P};

struct ZKBoo {}

fn find_shares(input: u32, mut ctx: Vec<IKosContext>) -> IKosResult<IKosVariable4P> {
    // TODO: 원래는 u8이었음 ;;
    let mut shares = vec![0; 3];
    shares[0] = 0xFFFFFF & get_next_random_from_context(&mut ctx[0])?;
    shares[1] = 0xFFFFFF & get_next_random_from_context(&mut ctx[0])?;
    shares[2] = input ^ shares[0] ^ shares[1];
    Ok(IKosVariable4P::new_share(shares, ctx))
}

type ProveCircuit = fn(Vec<IKosVariable4P>, usize, Vec<u32>, usize, Vec<IKosVariable4P>, usize);
type VerifyingCircuit = fn(Vec<IKosVariable4P>, usize, Vec<u32>, usize, Vec<IKosVariable4P>, usize);

impl ZKBoo {
    pub fn prove(
        full_proof: Vec<String>,
        input: Vec<u32>,
        input_pub: Vec<u32>,
        output: Vec<u32>,
        rand_tape_len: usize,
        circuit: ProveCircuit,
    ) {
    }

    pub fn verify() {
        //
    }
}
