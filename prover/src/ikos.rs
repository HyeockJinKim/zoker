use crate::utils::convert_vec_to_u8;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

macro_rules! get_bit {
    ($x: expr, $i: expr) => {{
        ($x >> $i) & 0x01
    }};
}

macro_rules! set_bit {
    ($x: expr, $i: expr, $b: expr) => {{
        $x = if $b & 1 != 0 {
            $x | (1 << $i)
        } else {
            $x & (!(1 << $i))
        };
    }};
}

pub type IKosResult<T> = Result<T, IKosError>;

#[derive(Debug, PartialEq)]
pub struct IKosError {
    pub error: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IKosView {
    rand_tape_seed: Vec<u8>,
    pub in_data: Vec<u32>,
    pub out_data: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IKosContext {
    pub ikos_view: IKosView,
    randomness: Vec<u32>,
    used_rand_ctr: usize,
    pub out_view_ctr: usize,
    is_verify_mode: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IKosVariable4P {
    pub value: Vec<u32>,
    ctx: Vec<IKosContext>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IKosVariable4V {
    pub value: Vec<u32>,
    pub ctx: Vec<IKosContext>,
    inst_random: Vec<u32>,
}

fn generate_random(num: usize) -> Vec<u8> {
    // TODO: 랜덤을 사용해야 함
    vec![0; num]
}

fn generate_all_randomness(rand_len: usize) -> Vec<u32> {
    // TODO: 랜덤을 사용해야 함
    let len = rand_len / 32;
    vec![0; len]
}

pub fn get_next_random_from_context(ctx: &mut IKosContext) -> IKosResult<u32> {
    if ctx.randomness.len() <= ctx.used_rand_ctr {
        return Err(IKosError {
            error: String::from("All pre-generated randomness are exhausted!"),
        });
    }
    let rand = ctx.randomness[ctx.used_rand_ctr as usize];
    ctx.used_rand_ctr += 1;
    Ok(rand)
}

impl IKosView {
    pub fn new() -> Self {
        let ozkb_rand_tape_seed_len = 16;
        IKosView {
            rand_tape_seed: generate_random(ozkb_rand_tape_seed_len),
            in_data: vec![],
            out_data: vec![],
        }
    }
}

impl IKosContext {
    pub fn new(rand_tape_len: usize, is_verify_mode: bool) -> Self {
        IKosContext {
            ikos_view: IKosView::new(),
            randomness: generate_all_randomness(rand_tape_len * 8),
            used_rand_ctr: 0,
            out_view_ctr: 0,
            is_verify_mode,
        }
    }

    pub fn new_views(ikos_view: IKosView, rand_tape_len: usize, is_verify_mode: bool) -> Self {
        IKosContext {
            ikos_view,
            randomness: generate_all_randomness(rand_tape_len * 8),
            used_rand_ctr: 0,
            out_view_ctr: 0,
            is_verify_mode,
        }
    }

    pub fn commit_ikos_context(&mut self) -> Vec<u8> {
        let mut sha = Sha256::new();
        sha.input(&self.ikos_view.rand_tape_seed);
        if !self.ikos_view.out_data.is_empty() {
            sha.input(convert_vec_to_u8::<u32>(&self.ikos_view.out_data).as_ref());
        }
        sha.result_str().as_bytes().to_vec()
    }
}

impl IKosVariable4P {
    pub fn new() -> Self {
        IKosVariable4P {
            value: vec![0; 3],
            ctx: vec![],
        }
    }

    pub fn new_value(value: u32) -> Self {
        IKosVariable4P {
            value: vec![value; 3],
            ctx: vec![],
        }
    }

    pub fn new_share(value: Vec<u32>, ctx: Vec<IKosContext>) -> Self {
        IKosVariable4P { value, ctx }
    }

    fn is_empty_context(&self) -> bool {
        self.ctx.is_empty()
    }

    fn copy_context(&mut self, rhs_ctx: Vec<IKosContext>) {
        self.ctx = rhs_ctx;
    }

    pub fn negate(mut self) -> Self {
        for i in 0..3 {
            self.value[i] = !self.value[i];
        }
        self
    }

    pub fn or(mut self, rhs: &IKosVariable4P) -> Self {
        if self.is_empty_context() {
            self.copy_context(rhs.ctx.clone());
        }
        for i in 0..3 {
            self.value[i] ^= rhs.value[i];
        }
        self
    }

    pub fn rshift(mut self, n: u32) -> Self {
        for i in 0..3 {
            self.value[i] >>= n;
        }
        self
    }

    pub fn lshift(mut self, n: u32) -> Self {
        for i in 0..3 {
            self.value[i] <<= n;
        }
        self
    }

    pub fn bit_and(mut self, rhs: &IKosVariable4P) -> Self {
        let mut rand = vec![0; 3];
        let mut out = vec![0; 3];

        if self.is_empty_context() && rhs.is_empty_context() {
            for i in 0..3 {
                self.value[i] &= rhs.value[i];
            }
            return self;
        }

        if self.is_empty_context() {
            self.copy_context(rhs.ctx.clone());
        }

        for i in 0..3 {
            rand[i] = get_next_random_from_context(&mut self.ctx[i]).unwrap();
        }
        for i in 0..3 {
            out[i] = (self.value[i] & rhs.value[(i + 1) % 3])
                ^ (self.value[(i + 1) % 3] & rhs.value[i])
                ^ (self.value[i] & rhs.value[i])
                ^ rand[i]
                ^ rand[(i + 1) % 3];
        }
        for i in 0..3 {
            self.value[i] = out[i];
            self.ctx[i].ikos_view.out_data.push(self.value[i]);
        }
        self
    }

    pub fn bit_or(mut self, rhs: &IKosVariable4P) -> Self {
        if self.is_empty_context() {
            self.copy_context(rhs.ctx.clone());
        }
        for i in 0..3 {
            self.value[i] |= rhs.value[i];
        }
        self
    }

    pub fn add(mut self, rhs: &IKosVariable4P) -> Self {
        let mut a = vec![0; 3];
        let mut b = vec![0; 3];
        let mut c = 0;
        let mut rand = vec![0; 3];
        let mut out = vec![0; 3];

        if self.is_empty_context() && rhs.is_empty_context() {
            for i in 0..3 {
                self.value[i] += rhs.value[i];
            }
            return self;
        }

        if self.is_empty_context() {
            self.copy_context(rhs.ctx.clone());
        }
        for i in 0..3 {
            rand[i] |= get_next_random_from_context(&mut self.ctx[i]).unwrap();
        }

        for i in 0..31 {
            for j in 0..3 {
                a[j] = get_bit!(self.value[j] ^ out[j], i);
                b[j] = get_bit!(rhs.value[j] ^ out[j], i);
            }
            for j in 0..3 {
                c = (a[j] & b[(j + 1) % 3])
                    ^ (a[(j + 1) % 3] & b[j])
                    ^ get_bit!(rand[(j + 1) % 3], i);
                set_bit!(
                    out[j],
                    i + 1,
                    (c ^ (a[j] & b[j]) ^ (get_bit!(out[j], i)) ^ (get_bit!(rand[j], i)))
                );
            }
        }

        for i in 0..3 {
            self.value[i] = self.value[i] ^ rhs.value[i] ^ out[i];
            self.ctx[i].ikos_view.out_data.push(out[i]);
        }
        self
    }
}

impl IKosVariable4V {
    pub fn new() -> Self {
        IKosVariable4V {
            value: vec![0; 2],
            ctx: vec![],
            inst_random: vec![],
        }
    }

    pub fn new_value(value: u32) -> Self {
        IKosVariable4V {
            value: vec![value; 2],
            ctx: vec![],
            inst_random: vec![],
        }
    }

    pub fn new_share(value: Vec<u32>, ctx: Vec<IKosContext>) -> Self {
        IKosVariable4V {
            value,
            ctx,
            inst_random: vec![],
        }
    }

    pub fn new_share_with_random(
        value: Vec<u32>,
        ctx: Vec<IKosContext>,
        inst_random: Vec<u32>,
    ) -> Self {
        IKosVariable4V {
            value,
            ctx,
            inst_random,
        }
    }

    // TODO: 이 함수들은 P 에서도 사용함
    fn is_empty_context(&self) -> bool {
        self.ctx.is_empty()
    }

    // TODO: 이 함수들은 P 에서도 사용함
    fn copy_context(&mut self, rhs_ctx: Vec<IKosContext>) {
        self.ctx = rhs_ctx;
    }

    pub fn negate(mut self) -> Self {
        for i in 0..2 {
            self.value[i] = !self.value[i];
        }
        self
    }

    pub fn or(mut self, rhs: &IKosVariable4V) -> Self {
        if self.is_empty_context() {
            self.copy_context(rhs.ctx.clone());
        }
        for i in 0..2 {
            self.value[i] ^= rhs.value[i];
        }
        self
    }

    pub fn rshift(mut self, n: u32) -> Self {
        for i in 0..2 {
            self.value[i] >>= n;
        }
        self
    }

    pub fn lshift(mut self, n: u32) -> Self {
        for i in 0..2 {
            self.value[i] <<= n;
        }
        self
    }

    pub fn require_reconstruct(ctx: &Vec<IKosContext>) -> bool {
        ctx[0].ikos_view.out_data.len() != ctx[1].ikos_view.out_data.len()
    }

    fn get_next_random(&mut self, i: usize) -> IKosResult<u32> {
        if i < 2 {
            get_next_random_from_context(&mut self.ctx[i])
        } else {
            Ok(0)
        }
    }

    pub fn bit_and(mut self, rhs: &IKosVariable4V) -> IKosResult<Self> {
        let mut rand = vec![0; 2];
        let mut out = 0;

        if self.is_empty_context() && rhs.is_empty_context() {
            for i in 0..2 {
                self.value[i] &= rhs.value[i];
            }
            return Ok(self);
        }

        if self.is_empty_context() {
            self.copy_context(rhs.ctx.clone());
        }

        for i in 0..2 {
            rand[i] = get_next_random_from_context(&mut self.ctx[i]).unwrap();
        }

        out = (self.value[0] & rhs.value[1])
            ^ (self.value[1] & rhs.value[0])
            ^ (self.value[0] & rhs.value[0])
            ^ rand[0]
            ^ rand[1];

        if self.ctx[0].is_verify_mode {
            // verify mode
            if !IKosVariable4V::require_reconstruct(&self.ctx) {
                println!(
                    "{:?} {:?} {}",
                    self.ctx.len(),
                    self.ctx[0].ikos_view.out_data,
                    self.ctx[0].out_view_ctr
                );
                if out != self.ctx[0].ikos_view.out_data[self.ctx[0].out_view_ctr] {
                    return Err(IKosError {
                        error: String::from("_IkosVariable4V & operation fail."),
                    });
                }
            } else {
                self.ctx[0].ikos_view.out_data.push(out);
            }
            self.value[0] = out;
            self.value[1] = self.ctx[1].ikos_view.out_data[self.ctx[1].out_view_ctr];
            for i in 0..2 {
                self.ctx[i].out_view_ctr += 1;
            }
        } else {
            // Non verify mode
            self.value[0] = out;
            self.value[1] = self.inst_random[self.inst_random[0] as usize];
            self.inst_random[0] += 1;

            for i in 0..2 {
                self.ctx[i].ikos_view.out_data.push(self.value[i]);
            }
        }

        Ok(self)
    }

    pub fn bit_or(mut self, rhs: &IKosVariable4V) -> Self {
        if self.is_empty_context() {
            self.copy_context(rhs.ctx.clone());
        }
        for i in 0..2 {
            self.value[i] |= rhs.value[i];
        }
        self
    }

    pub fn add(mut self, rhs: &IKosVariable4V) -> IKosResult<Self> {
        let mut a = vec![0; 2];
        let mut b = vec![0; 2];
        let mut rand = vec![0; 2];
        let mut out = vec![0; 2];
        if self.is_empty_context() && rhs.is_empty_context() {
            for i in 0..2 {
                self.value[i] += rhs.value[i];
            }
            return Ok(self);
        }

        if self.is_empty_context() {
            self.copy_context(rhs.ctx.clone());
        }
        for i in 0..2 {
            rand[i] = self.get_next_random(i)?;
        }

        if self.ctx[0].is_verify_mode {
            // verify mode
            let required = IKosVariable4V::require_reconstruct(&self.ctx);
            for i in 0..2 {
                if !required || i != 0 {
                    out[i] = self.ctx[i].ikos_view.out_data[self.ctx[i].out_view_ctr];
                }
                self.ctx[i].out_view_ctr += 1;
            }
            for i in 0..31 {
                for j in 0..2 {
                    a[j] = get_bit!(self.value[j] ^ out[j], i);
                    b[j] = get_bit!(rhs.value[j] ^ out[j], i);
                }
                let c = (a[0] & b[1]) ^ (a[1] & b[0]) ^ (get_bit!(rand[1], i));
                println!("{:?} {:?} {:?} {:?} {:?}", i, a, b, c, out);
                println!(
                    "{:?} {:?}",
                    c ^ (a[0] & b[0]) ^ (get_bit!(out[0], i)) ^ (get_bit!(rand[0], i)),
                    get_bit!(out[0], i + 1)
                );
                if !required {
                    if c ^ (a[0] & b[0]) ^ (get_bit!(out[0], i)) ^ (get_bit!(rand[0], i))
                        != get_bit!(out[0], i + 1)
                    {
                        return Err(IKosError {
                            error: String::from("IkosVariable4V + operation fail."),
                        });
                    }
                } else {
                    set_bit!(
                        out[0],
                        i + 1,
                        c ^ (a[0] & b[0]) ^ (get_bit!(out[0], i)) ^ (get_bit!(rand[0], i))
                    );
                }
            }
            if required {
                self.ctx[0].ikos_view.out_data.push(out[0]);
            }
            for i in 0..2 {
                self.value[i] = self.value[i] ^ rhs.value[i] ^ out[i];
            }
        } else {
            // Non verify mode
            out[1] = self.inst_random[self.inst_random[0] as usize];
            self.inst_random[0] += 1;
            set_bit!(out[1], 0, 0);

            for i in 0..31 {
                for j in 0..2 {
                    a[j] = get_bit!(self.value[j] ^ out[j], i);
                    b[j] = get_bit!(rhs.value[j] ^ out[j], i);
                }
                let c = (a[0] & b[1]) ^ (a[1] & b[0]) ^ (get_bit!(rand[1], i));
                set_bit!(
                    out[0],
                    i + 1,
                    c ^ (a[0] & b[0]) ^ (get_bit!(out[0], i)) ^ (get_bit!(rand[0], i))
                );
            }

            for i in 0..2 {
                self.value[i] = self.value[i] ^ rhs.value[i] ^ out[i];
                self.ctx[i].ikos_view.out_data.push(out[i]);
            }
        }
        Ok(self)
    }
}
