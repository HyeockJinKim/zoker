pragma solidity ^0.7.4;
pragma experimental ABIEncoderV2;

contract ZKBoo {
    uint32 constant OZKB_HASH_BYTES = 32;
    uint32 constant OZKB_NUMBER_OF_ROUNDS = 2;
    uint32 constant OZKB_PUBLIC_BRANCHES = 2;
    uint32 constant OZKB_COMMITMENT_VIEW_LENGTH = 32;
    uint32 constant OZKB_RND_TAPE_SEED_LEN = 16;
    uint32 constant AES_KEY_LENGTH = 16;
    uint32 constant OZKB_TOTAL_BRANCHES = 3;

    struct Proof {
        bytes challenge;
        uint inputLen;
        bytes inPub;
        uint inPubLen;
        bytes output;
        uint outputLen;
        uint randTapeLen;
        string proof;
        string[] response;
    }

    struct _IkosView {
        uint8[OZKB_RND_TAPE_SEED_LEN] _rndTapeSeed;
        uint8[]    _inData;
        uint32[] _outData32;
        uint64[] _outData64;
    }

    struct _IkosProof {
        _IkosView[OZKB_PUBLIC_BRANCHES] _2Outof3Views;
    }

    struct _IkosContext {
        _IkosView _ikosView;
        uint32[] _randomness;
        uint32 _usedRandCtr;
        uint32 _outViewCtr32;
        uint32 _outViewCtr64;
        bool _isVerifyMode;
    }

    struct _ikosVariable {
        uint8[OZKB_PUBLIC_BRANCHES] value__;
        _IkosContext[OZKB_PUBLIC_BRANCHES] ikosCtx__;
    }

    struct _3DVector {
        uint32[] _data;
        uint64 row__;
        uint64 col__;
        uint64 area__;
        uint64 depth__;
    }

    function _getIndex(_3DVector memory self, uint32 d, uint32 r, uint32 c) internal pure returns (uint64) {
        return d * self.area__ + self.col__ * r + c;
    }

    function _getNextRandomFromContext(_IkosContext memory ikosCtx) internal pure returns (uint32) {
        uint32 rand = 0;
        rand = ikosCtx._randomness[ikosCtx._usedRandCtr];
        ikosCtx._usedRandCtr++;

        return rand;
    }

    function _generateAllRandomness(bytes memory key, uint randLenBits) pure internal returns (uint32[] memory) {
        uint len = randLenBits/32;
        uint32[] memory randomness = new uint32[](len);
        bytes memory r;

        for (uint i = 0; i < randLenBits / 128; i++) {
            r = toBytes(keccak256(concat(r, key)));
            randomness[i*4] = toUint32(r, 0);
            randomness[i*4+1] = toUint32(r, 8);
            randomness[i*4+2] = toUint32(r, 16);
            randomness[i*4+3] = toUint32(r, 24);
        }
        return randomness;
    }

    function _initIkosContext(bytes memory key, uint randTapeLen, bool mode) pure internal returns (_IkosContext memory) {
        _IkosContext memory ikosCtx;

        for (uint i = 0; i < AES_KEY_LENGTH; ++i) {
            ikosCtx._ikosView._rndTapeSeed[i] = toUint8(key, i*8);
        }
        ikosCtx._randomness = _generateAllRandomness(key, randTapeLen * 8);
        ikosCtx._usedRandCtr = 0;
        ikosCtx._outViewCtr32 = 0;
        ikosCtx._outViewCtr64 = 0;
        ikosCtx._isVerifyMode = mode;

        return ikosCtx;
    }

    function _chooseIndexFromChallenges(bytes memory hash) pure internal returns (uint[OZKB_NUMBER_OF_ROUNDS] memory) {
        uint[OZKB_NUMBER_OF_ROUNDS] memory idxVec;
        uint numOfBlocks = 8;
        uint numOfRounds = 18;
        uint rep = 0;

        for (uint i = 0; i < numOfBlocks; i++) {
            uint32 val = toUint32(hash, i);
            for (int j = 0; i < numOfRounds; j++) {
                idxVec[rep++] = val % 3;
                val /= 3;
                if (OZKB_NUMBER_OF_ROUNDS <= rep) {
                    return idxVec;
                }
            }
        }
        return idxVec;
    }

    function _str2ikosview(string memory viewPart1, string memory viewPart2) pure internal returns (_IkosView memory) {
        _IkosView memory ikosview;
        {
            bytes memory viewStr = bytes(viewPart1);
            uint32 offset = 0;
            uint32 rndSize = OZKB_RND_TAPE_SEED_LEN*8;
            uint32 inDataSize = 0;

            for (uint i = 0; i < OZKB_RND_TAPE_SEED_LEN; ++i) {
                ikosview._rndTapeSeed[i] = toUint8(viewStr, offset+i*OZKB_RND_TAPE_SEED_LEN);
            }
            offset += rndSize;
            inDataSize = toUint32(viewStr, offset);
            offset += 4;
            uint8[] memory inData = new uint8[](inDataSize);
            for (uint i = 0; i < inDataSize; ++i) {
                inData[i] = toUint8(viewStr, offset+i*8);
            }
            ikosview._inData = inData;
        }

        if (bytes(viewPart2).length > 0) {
            bytes memory viewStr = bytes(viewPart2);
            uint32 offset = 0;
            uint32 outData64Size;
            uint32 outData32Size;

            outData32Size = toUint32(viewStr, offset);
            offset += 4;
            outData64Size = toUint32(viewStr, offset);
            offset += 4;
            uint32[] memory outData32 = new uint32[](outData32Size);
            for (uint i = 0; i < outData32Size; ++i) {
                outData32[i] = toUint32(viewStr, offset+i*8);
            }
            ikosview._outData32 = outData32;
            offset += 4 * outData32Size;

            uint64[] memory outData64 = new uint64[](outData64Size);
            for (uint i = 0; i < outData32Size; ++i) {
                outData64[i] = toUint32(viewStr, offset+i*8);
            }
            ikosview._outData64 = outData64;
        }

        return ikosview;
    }

    function str2proof__(
        string[3] memory proofStr,
        string[] memory responseStr,
        _IkosProof[] memory proof
    ) pure internal returns (byte[OZKB_NUMBER_OF_ROUNDS][OZKB_PUBLIC_BRANCHES][OZKB_COMMITMENT_VIEW_LENGTH] memory) {
        byte[OZKB_NUMBER_OF_ROUNDS][OZKB_PUBLIC_BRANCHES][OZKB_COMMITMENT_VIEW_LENGTH] memory twoViews;
        bytes memory views = bytes(proofStr[2]);
        for (uint i = 0; i < OZKB_NUMBER_OF_ROUNDS; ++i) {
            for (uint j = 0; j < OZKB_PUBLIC_BRANCHES; ++j) {
                for (uint k = 0; k < OZKB_COMMITMENT_VIEW_LENGTH; ++k) {
                    twoViews[i][j][k] = views[i*OZKB_NUMBER_OF_ROUNDS+j*OZKB_PUBLIC_BRANCHES+k];
                }
            }
        }

        uint viewIdx = 0;
        for (uint i = 0; i < OZKB_NUMBER_OF_ROUNDS; ++i) {
            proof[i]._2Outof3Views[0] = _str2ikosview(responseStr[viewIdx + 0], responseStr[viewIdx + 1]);
            proof[i]._2Outof3Views[1] = _str2ikosview(responseStr[viewIdx + 2], responseStr[viewIdx + 3]);
            viewIdx += 4;
        }

        return twoViews;
    }

    function _str2vecstr(string memory str) pure internal returns (string[3] memory) {
        string[3] memory vecStr;
        bytes memory strBytes = bytes(str);
        uint32 offset = 0;
        for (uint i = 0; i < 3; ++i) {
            uint32 stringSize = toUint32(strBytes, offset);
            string memory s = substring(str, offset+4, offset+4+stringSize);
            offset += 4 + stringSize;
            vecStr[i] = s;
        }

        return vecStr;
    }

    function _inDataFromNextRandom(
        _IkosContext[OZKB_NUMBER_OF_ROUNDS][OZKB_PUBLIC_BRANCHES] memory ikosCtx,
        _IkosProof[] memory ikosProof,
        uint inputLen,
        uint round,
        uint party
    ) internal pure {
        if (0 == ikosProof[round]._2Outof3Views[party]._inData.length) {
            uint8[] memory inData = new uint8[](inputLen);
            for (uint ilen = 0; ilen < inputLen; ilen++) {
                inData[ilen] = uint8(0xFF & _getNextRandomFromContext(ikosCtx[round][party]));   ///< _inData: a vector of uint8_t
            }
            ikosCtx[round][party]._ikosView._inData = inData;
        }
    }

    function _initContext(
        _IkosContext[OZKB_NUMBER_OF_ROUNDS][OZKB_PUBLIC_BRANCHES] memory ikosCtx,
        _IkosProof[] memory ikosProof,
        uint inputLen,
        uint randTapeLen
    ) internal pure {
        for (uint round = 0; round < OZKB_NUMBER_OF_ROUNDS; round++) {
            for (uint party = 0; party < OZKB_PUBLIC_BRANCHES; party++) {
                ikosCtx[round][party] = _initIkosContext(arrToBytes(ikosProof[round]._2Outof3Views[party]._rndTapeSeed), randTapeLen, true);  ///< in verify mode
                ikosCtx[round][party]._ikosView = ikosProof[round]._2Outof3Views[party];
                ikosCtx[round][party]._outViewCtr32 = 0;
                ikosCtx[round][party]._outViewCtr64 = 0;
                ///< reconstruct the last share

                _inDataFromNextRandom(ikosCtx, ikosProof, inputLen, round, party);
            }
        }
    }

    function _requireReconstruct(_IkosContext[OZKB_PUBLIC_BRANCHES] memory ikosCtx) internal pure returns (bool) {
        return (ikosCtx[0]._ikosView._outData32.length != ikosCtx[1]._ikosView._outData32.length) ||
        (ikosCtx[0]._ikosView._outData64.length != ikosCtx[1]._ikosView._outData64.length);
    }

    function _commitIkosContext(_IkosContext memory ikosCtx) internal pure returns (bytes32) {
        return sha256(abi.encodePacked(ikosCtx._ikosView._rndTapeSeed, ikosCtx._ikosView._outData32[0]));
    }

    function first_round(
        _IkosContext[OZKB_NUMBER_OF_ROUNDS][OZKB_PUBLIC_BRANCHES] memory ikosCtx,
        _ikosVariable[] memory ikosOutput,
        uint ikosOutputLen,
        uint round,
        bool required
    ) internal pure {
        for (uint branch = 0; branch < OZKB_PUBLIC_BRANCHES; branch++) {
            uint32[] memory outData32 = new uint32[](ikosOutputLen);
            for (uint olen = 0; olen < ikosOutputLen; olen++) {
                if (!required || 0 != branch) {
                    assert(ikosOutput[olen].value__[branch] == ikosCtx[round][branch]._ikosView._outData32[ikosCtx[round][branch]._outViewCtr32]);
                } else {
                    outData32[olen] = ikosOutput[olen].value__[branch];
                }
                ikosCtx[round][branch]._outViewCtr32++;
            }
            ikosCtx[round][branch]._ikosView._outData32 = outData32;
        }
    }

    function second_round(
        _IkosContext[OZKB_NUMBER_OF_ROUNDS][OZKB_PUBLIC_BRANCHES] memory ikosCtx,
        byte[OZKB_NUMBER_OF_ROUNDS][OZKB_PUBLIC_BRANCHES][OZKB_COMMITMENT_VIEW_LENGTH] memory twoViews,
        uint[OZKB_NUMBER_OF_ROUNDS] memory idxVec,
        uint round
    ) internal pure {
        bytes32 cmt = _commitIkosContext(ikosCtx[round][0]);
        byte[OZKB_NUMBER_OF_ROUNDS * OZKB_TOTAL_BRANCHES * OZKB_COMMITMENT_VIEW_LENGTH] memory threeViews;
        uint offset3Views = 0;
        uint offset2Views = 0;
        uint offset2Views2 = 0;
        if (0 == idxVec[round]) {
            for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; i++) {
                threeViews[offset3Views+i] = cmt[i];
            }
            offset3Views += OZKB_COMMITMENT_VIEW_LENGTH;

            for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; i++) {
                threeViews[offset3Views+i] = twoViews[offset2Views2][offset2Views][i];
            }
            offset3Views += OZKB_COMMITMENT_VIEW_LENGTH;
            offset2Views += 1;
            if (offset2Views == OZKB_PUBLIC_BRANCHES) {
                offset2Views = 0;
                offset2Views2 += 1;
            }

            for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; i++) {
                threeViews[offset3Views+i] = twoViews[offset2Views2][offset2Views][i];
            }
            offset3Views += OZKB_COMMITMENT_VIEW_LENGTH;
            offset2Views += 1;
            if (offset2Views == OZKB_PUBLIC_BRANCHES) {
                offset2Views = 0;
                offset2Views2 += 1;
            }
        }
        else if (1 == idxVec[round]) {
            for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; i++) {
                threeViews[offset3Views+i] = twoViews[offset2Views2][offset2Views][i];
            }
            offset3Views += OZKB_COMMITMENT_VIEW_LENGTH;
            offset2Views += 1;
            if (offset2Views == OZKB_PUBLIC_BRANCHES) {
                offset2Views = 0;
                offset2Views2 += 1;
            }

            for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; i++) {
                threeViews[offset3Views+i] = cmt[i];
            }
            offset3Views += OZKB_COMMITMENT_VIEW_LENGTH;

            for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; i++) {
                threeViews[offset3Views+i] = twoViews[offset2Views2][offset2Views][i];
            }
            offset3Views += OZKB_COMMITMENT_VIEW_LENGTH;
            offset2Views += 1;
            if (offset2Views == OZKB_PUBLIC_BRANCHES) {
                offset2Views = 0;
                offset2Views2 += 1;
            }
        }
        else if (2 == idxVec[round]) {
            for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; i++) {
                threeViews[offset3Views+i] = twoViews[offset2Views2][offset2Views][i];
            }
            offset3Views += OZKB_COMMITMENT_VIEW_LENGTH;
            offset2Views += 1;
            if (offset2Views == OZKB_PUBLIC_BRANCHES) {
                offset2Views = 0;
                offset2Views2 += 1;
            }
            for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; i++) {
                threeViews[offset3Views+i] = twoViews[offset2Views2][offset2Views][i];
            }
            offset3Views += OZKB_COMMITMENT_VIEW_LENGTH;
            offset2Views += 1;
            if (offset2Views == OZKB_PUBLIC_BRANCHES) {
                offset2Views = 0;
                offset2Views2 += 1;
            }

            for (uint i = 0; i < OZKB_COMMITMENT_VIEW_LENGTH; i++) {
                threeViews[offset3Views+i] = cmt[i];
            }
            offset3Views += OZKB_COMMITMENT_VIEW_LENGTH;
        }
        else {
            assert(false);
        }

    }

    function third_round(
        _3DVector memory vecView,
        uint[OZKB_NUMBER_OF_ROUNDS] memory idxVec,
        _ikosVariable[] memory ikosOutput,
        bytes memory output,
        uint ikosOutputLen,
        uint round
    ) internal pure {
        for (uint olen = 0; olen < ikosOutputLen; olen++) {
            uint64 pos = _getIndex(vecView, uint32(olen), uint32(round), 0);

            if (0 == idxVec[round]) {
                vecView._data[pos + 0] = ikosOutput[olen].value__[0];
                vecView._data[pos + 1] = ikosOutput[olen].value__[1];
                vecView._data[pos + 2] = toUint32(output, olen);
                // vecView._data[pos + 2] = output[olen] ^ vecView._data[pos + 0] ^ vecView._data[pos + 1];
            }
            else if (1 == idxVec[round]) {
                vecView._data[pos + 1] = ikosOutput[olen].value__[0];
                vecView._data[pos + 2] = ikosOutput[olen].value__[1];
                vecView._data[pos + 0] = toUint32(output, olen);
                // vecView._data[pos + 0] = output[olen] ^ vecView._data[pos + 1] ^ vecView._data[pos + 2];
            }
            else if (2 == idxVec[round]) {
                vecView._data[pos + 2] = ikosOutput[olen].value__[0];
                vecView._data[pos + 0] = ikosOutput[olen].value__[1];
                vecView._data[pos + 1] = toUint32(output, olen);
                // vecView._data[pos + 1] = output[olen] ^ vecView._data[pos + 2] ^ vecView._data[pos + 0];
            }
            else {
                assert(false);
            }
        }
    }

    function run_round(
        _IkosProof[] memory ikosProof,
        byte[OZKB_NUMBER_OF_ROUNDS][OZKB_PUBLIC_BRANCHES][OZKB_COMMITMENT_VIEW_LENGTH] memory twoViews,
        uint[OZKB_NUMBER_OF_ROUNDS] memory idxVec,
        bytes memory output,
        _3DVector memory vecView,
        uint inputLen,
        uint randTapeLen,
        uint ikosOutputLen
    ) internal pure {
        _IkosContext[OZKB_NUMBER_OF_ROUNDS][OZKB_PUBLIC_BRANCHES] memory ikosCtx;

        for (uint round = 0; round < OZKB_NUMBER_OF_ROUNDS; round++) {
            _initContext(ikosCtx, ikosProof, inputLen, randTapeLen);
            _ikosVariable[] memory ikosInput = new _ikosVariable[](inputLen);
            _ikosVariable[] memory ikosOutput = new _ikosVariable[](ikosOutputLen);

            for (uint i = 0; i < inputLen; i++) {
                uint8[OZKB_PUBLIC_BRANCHES] memory shares;

                for (uint party = 0; party < OZKB_PUBLIC_BRANCHES; party++) {
                    shares[party] = ikosCtx[round][party]._ikosView._inData[i];
                }
                ikosInput[i] = _ikosVariable(shares, ikosCtx[round]);
            }

            // run circuit with callback?
            // (*targetCircuit4V)(ikosInput, inputLen, (uint8_t *)inPub, inPubLen, ikosOutput, ikosOutputLen);

            bool required = _requireReconstruct(ikosCtx[round]);
            first_round(ikosCtx, ikosOutput, ikosOutputLen, round, required);
            second_round(ikosCtx, twoViews, idxVec, round);
            third_round(vecView, idxVec, ikosOutput, output, ikosOutputLen, round);
        }
    }

    function _verify(Proof memory inputProof) pure public {
        _IkosProof[] memory ikosProof = new _IkosProof[](OZKB_NUMBER_OF_ROUNDS);
        string[3] memory vecProof = _str2vecstr(inputProof.proof);
        uint ikosOutputLen = (inputProof.outputLen) / 4;
        _3DVector memory vecView = _3DVector(
            new uint32[](OZKB_NUMBER_OF_ROUNDS*OZKB_TOTAL_BRANCHES*ikosOutputLen),
            OZKB_NUMBER_OF_ROUNDS, OZKB_TOTAL_BRANCHES, OZKB_NUMBER_OF_ROUNDS*OZKB_TOTAL_BRANCHES, uint64(ikosOutputLen));
        byte[OZKB_NUMBER_OF_ROUNDS][OZKB_PUBLIC_BRANCHES][OZKB_COMMITMENT_VIEW_LENGTH] memory twoViews = str2proof__(vecProof, inputProof.response, ikosProof);
        uint[OZKB_NUMBER_OF_ROUNDS] memory idxVec = _chooseIndexFromChallenges(inputProof.challenge);
        run_round(ikosProof, twoViews, idxVec, inputProof.output, vecView, inputProof.inputLen, inputProof.randTapeLen, ikosOutputLen);
    }

    function concat(bytes memory a, bytes memory b) public pure returns (bytes memory c) {
        // Store the length of the first array
        uint alen = a.length;
        // Store the length of BOTH arrays
        uint totallen = alen + b.length;
        // Count the loops required for array a (sets of 32 bytes)
        uint loopsa = (a.length + 31) / 32;
        // Count the loops required for array b (sets of 32 bytes)
        uint loopsb = (b.length + 31) / 32;
        assembly {
            let m := mload(0x40)
        // Load the length of both arrays to the head of the new bytes array
            mstore(m, totallen)
        // Add the contents of a to the array
            for {  let i := 0 } lt(i, loopsa) { i := add(1, i) } { mstore(add(m, mul(32, add(1, i))), mload(add(a, mul(32, add(1, i))))) }
        // Add the contents of b to the array
            for {  let i := 0 } lt(i, loopsb) { i := add(1, i) } { mstore(add(m, add(mul(32, add(1, i)), alen)), mload(add(b, mul(32, add(1, i))))) }
            mstore(0x40, add(m, add(32, totallen)))
            c := m
        }
    }

    function toUint8(bytes memory _bytes, uint256 _start) internal pure returns (uint8) {
        require(_start + 1 >= _start, "toUint8_overflow");
        require(_bytes.length >= _start + 1 , "toUint8_outOfBounds");
        uint8 tempUint;

        assembly {
            tempUint := mload(add(add(_bytes, 0x1), _start))
        }

        return tempUint;
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

    function toBytes(bytes32 self) internal pure returns (bytes memory bts) {
        bts = new bytes(32);
        assembly {
            mstore(add(bts, /*BYTES_HEADER_SIZE*/32), self)
        }
    }

    function arrToBytes(uint8[16] memory arr) internal pure returns (bytes memory) {
        bytes memory b = new bytes(arr.length);
        for (uint i = 0; i < arr.length; ++i) {
            b[i] = byte(arr[i]);
        }
        return b;
    }

    function substring(string memory str, uint startIndex, uint endIndex) internal pure returns (string memory) {
        bytes memory strBytes = bytes(str);
        bytes memory result = new bytes(endIndex-startIndex);
        for(uint i = startIndex; i < endIndex; i++) {
            result[i-startIndex] = strBytes[i];
        }
        return string(result);
    }
}
