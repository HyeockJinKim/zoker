// SPDX-License-Identifier: MIT
pragma solidity ^0.7.4;
pragma experimental ABIEncoderV2;

contract ZKBoo {
    uint32 constant OZKB_NUMBER_OF_ROUNDS = 2;
    uint32 constant OZKB_PUBLIC_BRANCHES = 2;
    uint32 constant OZKB_COMMITMENT_VIEW_LENGTH = 32;
    uint32 constant OZKB_RND_TAPE_SEED_LEN = 16;
    uint32 constant OZKB_TOTAL_BRANCHES = 3;

    // Declaration of structure
    struct _3DVector {
        uint[] data;
        uint row;
        uint col;
        uint area;
        uint depth;
    }

    struct IKosView {
        bytes rand_tape_seed;
        uint32[] in_data;
        uint32[] out_data;
    }

    struct IKosContext {
        IKosView ikos_view;
        uint32[8] randomness;
        uint32 used_rand_ctr;
        uint32 out_view_ctr;
    }

    struct IKosVariable4V {
        uint32[OZKB_PUBLIC_BRANCHES] value;
        IKosContext[OZKB_PUBLIC_BRANCHES] ctx;
        bool is_full;
    }

    struct VerifyingProof {
        uint input_len;
        uint32[] input_pub;
        uint32[] output;
        bytes32 challenge;
        bytes two_views;
        IKosView[] response;
        function(IKosVariable4V[] memory, uint32[] memory) internal pure returns (IKosVariable4V[] memory) circuit;
    }

    // 3D Vector
    function _3DVector_new(uint depth, uint row, uint col) internal pure returns (_3DVector memory) {
        uint area = row * col;
        uint[] memory data = new uint[](depth * area);
        return _3DVector(data, row, col, area, depth);
    }

    function _3DVector_get_index(_3DVector memory self, uint d, uint r, uint c) internal pure returns (uint) {
        return d * self.area + self.col * r + c;
    }

    function generate_randomness(bytes memory key) internal pure returns (uint32[8] memory) {
        uint32[8] memory randomness;
        bytes memory r = toBytes(sha256(key));
        for (uint i = 0; i < 8; i++) {
            randomness[i] = toUint32(r, 4 * i);
        }
        return randomness;
    }


    function get_next_random_from_context(IKosContext memory ctx) internal pure returns (uint32) {
        if (ctx.used_rand_ctr == ctx.randomness.length) {
            bytes memory b = new bytes(32);
            for (uint i = 0; i < 8; i++) {
                b[i * 4] = byte(uint8(ctx.randomness[i] >> 24));
                b[i * 4 + 1] = byte(uint8(ctx.randomness[i] >> 16));
                b[i * 4 + 2] = byte(uint8(ctx.randomness[i] >> 8));
                b[i * 4 + 3] = byte(uint8(ctx.randomness[i]));
            }
            ctx.randomness = generate_randomness(b);
            ctx.used_rand_ctr = 0;
        }
        uint32 rand = ctx.randomness[ctx.used_rand_ctr];
        ctx.used_rand_ctr += 1;
        return rand;
    }

    // Ikos view
    function new_views(IKosView memory ikos_view) internal pure returns (IKosContext memory) {
        uint32[8] memory randomness = generate_randomness(ikos_view.rand_tape_seed);
        return IKosContext(ikos_view, randomness, 0, 0);
    }

    function commit_ikos_context(IKosContext memory ctx) internal pure returns (bytes32) {
        bytes memory b = new bytes(OZKB_RND_TAPE_SEED_LEN + ctx.ikos_view.out_data.length * 4);
        for (uint i = 0; i < OZKB_RND_TAPE_SEED_LEN; ++i) {
            b[i] = ctx.ikos_view.rand_tape_seed[i];
        }
        for (uint i = 0; i < ctx.ikos_view.out_data.length; i++) {
            b[OZKB_RND_TAPE_SEED_LEN + i * 4] = byte(uint8(ctx.ikos_view.out_data[i] >> 24));
            b[OZKB_RND_TAPE_SEED_LEN + i * 4 + 1] = byte(uint8(ctx.ikos_view.out_data[i] >> 16));
            b[OZKB_RND_TAPE_SEED_LEN + i * 4 + 2] = byte(uint8(ctx.ikos_view.out_data[i] >> 8));
            b[OZKB_RND_TAPE_SEED_LEN + i * 4 + 3] = byte(uint8(ctx.ikos_view.out_data[i]));
        }
        bytes32 s = sha256(b);
        return s;
    }

    // IkosVariable for Verifier
    function IKosVariable_new_value(uint32 value) internal pure returns (IKosVariable4V memory) {
        uint32[OZKB_PUBLIC_BRANCHES] memory val;
        val[0] = value;
        val[1] = value;
        IKosContext[OZKB_PUBLIC_BRANCHES] memory ctx;
        return IKosVariable4V(val, ctx, false);
    }

    function IKosVariable_new_share(
        uint32[OZKB_PUBLIC_BRANCHES] memory value,
        IKosContext[OZKB_PUBLIC_BRANCHES] memory ctx
    ) internal pure returns (IKosVariable4V memory) {
        return IKosVariable4V(value, ctx, true);
    }

    function is_empty_context(IKosVariable4V memory self) internal pure returns (bool) {
        return !self.is_full;
    }

    function negate(IKosVariable4V memory self) internal pure returns (IKosVariable4V memory) {
        uint32[OZKB_PUBLIC_BRANCHES] memory val;
        for (uint i = 0; i < OZKB_PUBLIC_BRANCHES; ++i) {
            val[i] = ~self.value[i];
        }
        return IKosVariable4V(val, self.ctx, self.is_full);
    }

    function _xor(IKosVariable4V memory self, IKosVariable4V memory rhs) internal pure returns (IKosVariable4V memory) {
        uint32[OZKB_PUBLIC_BRANCHES] memory val;
        for (uint i = 0; i < OZKB_PUBLIC_BRANCHES; ++i) {
            val[i] = self.value[i] ^ rhs.value[i];
        }
        return IKosVariable4V(val, self.ctx, self.is_full);
    }

    function bit_or(IKosVariable4V memory self, IKosVariable4V memory rhs) internal pure returns (IKosVariable4V memory) {
        uint32[OZKB_PUBLIC_BRANCHES] memory val;
        for (uint i = 0; i < OZKB_PUBLIC_BRANCHES; ++i) {
            val[i] = self.value[i] | rhs.value[i];
        }
        return IKosVariable4V(val, self.ctx, self.is_full);
    }

    function rshift(IKosVariable4V memory self, uint32 n) internal pure returns (IKosVariable4V memory) {
        uint32[OZKB_PUBLIC_BRANCHES] memory val;
        for (uint i = 0; i < OZKB_PUBLIC_BRANCHES; ++i) {
            val[i] = self.value[i] >> n;
        }
        return IKosVariable4V(val, self.ctx, self.is_full);
    }

    function lshift(IKosVariable4V memory self, uint32 n) internal pure returns (IKosVariable4V memory) {
        uint32[OZKB_PUBLIC_BRANCHES] memory val;
        for (uint i = 0; i < OZKB_PUBLIC_BRANCHES; ++i) {
            val[i] = self.value[i] << n;
        }
        return IKosVariable4V(val, self.ctx, self.is_full);
    }

    function bit_and(IKosVariable4V memory self, IKosVariable4V memory rhs) internal pure returns (IKosVariable4V memory) {
        uint32[OZKB_PUBLIC_BRANCHES] memory val;
        if (is_empty_context(self) && is_empty_context(rhs)) {
            for (uint i = 0; i < OZKB_PUBLIC_BRANCHES; ++i) {
                val[i] = self.value[i] & rhs.value[i];
            }
            return IKosVariable4V(val, self.ctx, self.is_full);
        }

        if (is_empty_context(self)) {
            return bit_and(rhs, self);
        }

        uint32[OZKB_PUBLIC_BRANCHES] memory rand;
        for (uint i = 0; i < OZKB_PUBLIC_BRANCHES; ++i) {
            rand[i] = get_next_random_from_context(self.ctx[i]);
        }
        uint32 out = (self.value[0] & rhs.value[1])
        ^ (self.value[1] & rhs.value[0])
        ^ (self.value[0] & rhs.value[0])
        ^ rand[0]
        ^ rand[1];

        if (out != self.ctx[0].ikos_view.out_data[self.ctx[0].out_view_ctr]) {
            revert();
        }
        val[0] = out;
        val[1] = self.ctx[1].ikos_view.out_data[self.ctx[1].out_view_ctr];
        for (uint i = 0; i < OZKB_PUBLIC_BRANCHES; ++i) {
            self.ctx[i].out_view_ctr += 1;
        }

        return IKosVariable4V(val, self.ctx, self.is_full);
    }

    function get_bit(uint32 x, uint i) internal pure returns (uint32) {
        return (x >> i) & 1;
    }

    function add_op(IKosVariable4V memory self, IKosVariable4V memory rhs) internal pure returns (IKosVariable4V memory) {
        uint32[OZKB_PUBLIC_BRANCHES] memory val;
        if (is_empty_context(self) && is_empty_context(rhs)) {
            for (uint i = 0; i < OZKB_PUBLIC_BRANCHES; ++i) {
                val[i] = self.value[i] + rhs.value[i];
            }
            return IKosVariable4V(val, self.ctx, self.is_full);
        }
        if (is_empty_context(self)) {
            return add_op(rhs, self);
        }
        uint32[OZKB_PUBLIC_BRANCHES] memory rand;
        uint32[OZKB_PUBLIC_BRANCHES] memory a;
        uint32[OZKB_PUBLIC_BRANCHES] memory b;
        uint32[OZKB_PUBLIC_BRANCHES] memory out;
        for (uint i = 0; i < OZKB_PUBLIC_BRANCHES; ++i) {
            rand[i] = get_next_random_from_context(self.ctx[i]);
        }
        for (uint i = 0; i < OZKB_PUBLIC_BRANCHES; ++i) {
            out[i] = self.ctx[i].ikos_view.out_data[self.ctx[i].out_view_ctr];
            self.ctx[i].out_view_ctr += 1;
        }
        for (uint i = 0; i < 31; ++i) {
            for (uint j = 0; j < OZKB_PUBLIC_BRANCHES; ++j) {
                a[j] = get_bit((self.value[j] ^ out[j]), i);
                b[j] = get_bit((rhs.value[j] ^ out[j]), i);
            }

            uint32 c = (a[0] & b[1]) ^ (a[1] & b[0]) ^ get_bit(rand[1], i);
            if (c ^ (a[0] & b[0]) ^ (get_bit(out[0], i)) ^ (get_bit(rand[0], i))
                != get_bit(out[0], i + 1)) {
                revert();
            }
        }
        for (uint i = 0; i < OZKB_PUBLIC_BRANCHES; ++i) {
            val[i] = self.value[i] ^ rhs.value[i] ^ out[i];
        }
        return IKosVariable4V(val, self.ctx, self.is_full);
    }
    // ZKBoo+
    function ZKBoo_choose_index_from_challenge(bytes32 commit) internal pure returns (uint[OZKB_NUMBER_OF_ROUNDS] memory) {
        uint[OZKB_NUMBER_OF_ROUNDS] memory res;
        uint val = 0;
        for (uint i = 0; i < 4; ++i) {
            val = val * 16 + uint8(commit[i]);
        }
        for (uint i = 0; i < OZKB_NUMBER_OF_ROUNDS; ++i) {
            res[i] = val % OZKB_TOTAL_BRANCHES;
            val /= OZKB_TOTAL_BRANCHES;
        }
        return res;
    }

    function ZKBoo_verify(VerifyingProof memory proof) internal pure returns (bool) {
        uint[OZKB_NUMBER_OF_ROUNDS] memory index_vec = ZKBoo_choose_index_from_challenge(proof.challenge);
        _3DVector memory vec_view = _3DVector_new(proof.output.length, OZKB_NUMBER_OF_ROUNDS, OZKB_TOTAL_BRANCHES);
        uint8[OZKB_NUMBER_OF_ROUNDS * OZKB_TOTAL_BRANCHES * OZKB_COMMITMENT_VIEW_LENGTH] memory three_views;
        uint three_offset = 0;
        uint two_offset = 0;
        for (uint round = 0; round < OZKB_NUMBER_OF_ROUNDS; round++) {
            IKosContext[OZKB_PUBLIC_BRANCHES] memory ctx;
            for (uint party = 0; party < OZKB_PUBLIC_BRANCHES; ++party) {
                ctx[party] = new_views(proof.response[round * OZKB_PUBLIC_BRANCHES + party]);
                if (ctx[party].ikos_view.in_data.length == 0) {
                    ctx[party].ikos_view.in_data = new uint32[](proof.input_len);
                }
            }
            if (index_vec[round] == 0) {
                for (uint i = 0; i < proof.input_len; ++i) {
                    uint32 data = get_next_random_from_context(ctx[0]);
                    ctx[0].ikos_view.in_data[i] = data;
                }
            } else if (index_vec[round] == 1) {
                for (uint i = 0; i < proof.input_len; ++i) {
                    uint32 data = get_next_random_from_context(ctx[1]);
                    ctx[1].ikos_view.in_data[i] = data;
                }
            } else if (index_vec[round] == 2) {
                for (uint i = 0; i < proof.input_len; ++i) {
                    uint32 data = get_next_random_from_context(ctx[0]);
                    ctx[0].ikos_view.in_data[i] = data;
                    uint32 data2 = get_next_random_from_context(ctx[1]);
                    ctx[1].ikos_view.in_data[i] = data2;
                }
            } else {
                revert();
            }

            IKosVariable4V[] memory ikos_input = new IKosVariable4V[](proof.input_len);
            for (uint i = 0; i < proof.input_len; ++i) {
                uint32[OZKB_PUBLIC_BRANCHES] memory shares;
                for (uint j = 0; j < OZKB_PUBLIC_BRANCHES; ++j) {
                    shares[j] = ctx[j].ikos_view.in_data[i];
                }
                ikos_input[i] = IKosVariable_new_share(shares, ctx);
            }

            IKosVariable4V[] memory ikos_output = proof.circuit(ikos_input, proof.input_pub);
            for (uint branch = 0; branch < OZKB_PUBLIC_BRANCHES; ++branch) {
                for (uint j = 0; j < ikos_output.length; ++j) {
                    if (ikos_output[j].value[branch] != ctx[branch].ikos_view.out_data[ctx[branch].out_view_ctr]) {
                        revert();
                    }
                }
            }

            if (index_vec[round] == 0) {
                for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; ++i) {
                    three_views[three_offset + i] = uint8(proof.two_views[two_offset + i]);
                }
                three_offset += OZKB_COMMITMENT_VIEW_LENGTH;
                two_offset += OZKB_COMMITMENT_VIEW_LENGTH;

                bytes32 commit = commit_ikos_context(ctx[0]);
                for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; ++i) {
                    three_views[three_offset + i] = uint8(commit[i]);
                }
                three_offset += OZKB_COMMITMENT_VIEW_LENGTH;
                commit = commit_ikos_context(ctx[1]);
                for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; ++i) {
                    three_views[three_offset + i] = uint8(commit[i]);
                }
                three_offset += OZKB_COMMITMENT_VIEW_LENGTH;
            } else if (index_vec[round] == 1) {
                bytes32 commit = commit_ikos_context(ctx[1]);
                for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; ++i) {
                    three_views[three_offset + i] = uint8(commit[i]);
                }
                three_offset += OZKB_COMMITMENT_VIEW_LENGTH;
                for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; ++i) {
                    three_views[three_offset + i] = uint8(proof.two_views[two_offset + i]);
                }
                three_offset += OZKB_COMMITMENT_VIEW_LENGTH;
                two_offset += OZKB_COMMITMENT_VIEW_LENGTH;
                commit = commit_ikos_context(ctx[0]);
                for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; ++i) {
                    three_views[three_offset + i] = uint8(commit[i]);
                }
                three_offset += OZKB_COMMITMENT_VIEW_LENGTH;
            } else if (index_vec[round] == 2) {
                bytes32 commit = commit_ikos_context(ctx[0]);
                for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; ++i) {
                    three_views[three_offset + i] = uint8(commit[i]);
                }
                three_offset += OZKB_COMMITMENT_VIEW_LENGTH;
                commit = commit_ikos_context(ctx[1]);
                for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; ++i) {
                    three_views[three_offset + i] = uint8(commit[i]);
                }
                three_offset += OZKB_COMMITMENT_VIEW_LENGTH;
                for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; ++i) {
                    three_views[three_offset + i] = uint8(proof.two_views[two_offset + i]);
                }
                three_offset += OZKB_COMMITMENT_VIEW_LENGTH;
                two_offset += OZKB_COMMITMENT_VIEW_LENGTH;
            } else {
                revert();
            }

            for (uint i = 0; i < ikos_output.length; ++i) {
                uint pos = _3DVector_get_index(vec_view, i, round, 0);
                if (index_vec[round] == 0) {
                    vec_view.data[pos + 1] = ikos_output[i].value[0];
                    vec_view.data[pos + 2] = ikos_output[i].value[1];
                    vec_view.data[pos] = proof.output[i] ^ vec_view.data[pos + 1] ^ vec_view.data[pos + 2];
                } else if (index_vec[round] == 1) {
                    vec_view.data[pos] = ikos_output[i].value[1];
                    vec_view.data[pos + 2] = ikos_output[i].value[0];
                    vec_view.data[pos + 1] = proof.output[i] ^ vec_view.data[pos] ^ vec_view.data[pos + 2];
                } else if (index_vec[round] == 2) {
                    vec_view.data[pos] = ikos_output[i].value[0];
                    vec_view.data[pos + 1] = ikos_output[i].value[1];
                    vec_view.data[pos + 2] = proof.output[i] ^ vec_view.data[pos + 1] ^ vec_view.data[pos];
                } else {
                    revert();
                }
            }
        }
        bytes32 random_oracle = query_random_oracle(proof.input_len, proof.output.length, vec_view.data, three_views);
        for (uint i = 0; i < 32; ++i) {
            if (proof.challenge[i] != random_oracle[i]) {
                return false;
            }
        }
        return true;
    }

    function query_random_oracle(
        uint input_len,
        uint output_len,
        uint[] memory vec_view_data,
        uint8[OZKB_NUMBER_OF_ROUNDS * OZKB_TOTAL_BRANCHES * OZKB_COMMITMENT_VIEW_LENGTH] memory three_views
    ) internal pure returns (bytes32) {
        bytes memory b = new bytes(8 + 4 * vec_view_data.length + three_views.length);
        b[0] = byte(uint8(input_len >> 24));
        b[1] = byte(uint8(input_len >> 16));
        b[2] = byte(uint8(input_len >> 8));
        b[3] = byte(uint8(input_len));
        b[4] = byte(uint8(output_len >> 24));
        b[5] = byte(uint8(output_len >> 16));
        b[6] = byte(uint8(output_len >> 8));
        b[7] = byte(uint8(output_len));
        for (uint i = 0; i < vec_view_data.length; i++) {
            b[8 + i * 4] = byte(uint8(vec_view_data[i] >> 24));
            b[9 + i * 4] = byte(uint8(vec_view_data[i] >> 16));
            b[10 + i * 4] = byte(uint8(vec_view_data[i] >> 8));
            b[11 + i * 4] = byte(uint8(vec_view_data[i]));
        }
        uint sp = 8 + vec_view_data.length * 4;
        for (uint i = 0; i < three_views.length; i++) {
            b[sp + i] = byte(three_views[i]);
        }
        bytes32 s = sha256(b);
        return s;
    }

    // For type
    function toBytes(bytes32 self) internal pure returns (bytes memory bts) {
        bts = new bytes(32);
        assembly {
            mstore(add(bts, /*BYTES_HEADER_SIZE*/32), self)
        }
    }

    function toUint32(bytes memory _bytes, uint256 _start) internal pure returns (uint32) {
        require(_start + 4 >= _start, "toUint32_overflow");
        require(_bytes.length >= _start + 4, "toUint32_outOfBounds");
        uint32 tempUint;

        assembly {
            tempUint := mload(add(add(_bytes, 0x4), _start))
        }

        return tempUint;
    }
}
