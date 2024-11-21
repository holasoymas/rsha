const H: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn zeros_k(n: u64) -> u64 {
    let hel = n & 511; // same as doing (num % 512), getting the value of modulo only
                       // masking if negaive num exisit
    let neg = 447u64.wrapping_sub(hel); //(447 - neg num)
    neg & 511
}

fn padding_len(msg: &[u8]) -> u64 {
    let msg_len = msg.len() as u64;
    let len_in_bits = msg_len << 3; // multiply by 8
    let total_len = len_in_bits + 1 + zeros_k(len_in_bits) + 64;
    total_len
}

fn create_msg_padding(msg: &[u8]) -> Vec<u8> {
    let total_bits = padding_len(msg);
    let total_bytes = (total_bits >> 3) as usize;
    let mut vec = Vec::with_capacity(total_bytes);

    // copy the original msg
    vec.extend_from_slice(msg);

    // add 1 bit (technically a byte)
    vec.push(0x80);

    // Adding for k's 0
    let padding_bytes = total_bytes - msg.len() - 9;
    vec.extend(vec![0u8; padding_bytes]);

    let big_ind = (msg.len() as u64) << 3;
    vec.extend_from_slice(&big_ind.to_be_bytes());

    vec
}

fn process_block(state: &mut [u32; 8], block: &[u8]) {
    let mut w = [0u32; 64];

    // Prepare the message schedule (first 16 words from the block)
    for i in 0..16 {
        w[i] = u32::from_be_bytes([
            block[i * 4],
            block[i * 4 + 1],
            block[i * 4 + 2],
            block[i * 4 + 3],
        ]);
    }

    // Extend the message schedule
    for i in 16..64 {
        let w15 = w[i - 15];
        let w2 = w[i - 2];
        let s0 = w15.rotate_right(7) ^ w15.rotate_right(18) ^ (w15 >> 3);
        let s1 = w2.rotate_right(17) ^ w2.rotate_right(19) ^ (w2 >> 10);
        w[i] = w[i - 16]
            .wrapping_add(s0)
            .wrapping_add(w[i - 7])
            .wrapping_add(s1);
    }

    // Initialize the working variables with the current hash state
    let mut h = *state;

    // Compression loop (64 rounds)
    for i in 0..64 {
        let s1 = h[4].rotate_right(6) ^ h[4].rotate_right(11) ^ h[4].rotate_right(25);
        let ch = (h[4] & h[5]) ^ (!h[4] & h[6]);
        let temp1 = h[7]
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K[i])
            .wrapping_add(w[i]);

        let s0 = h[0].rotate_right(2) ^ h[0].rotate_right(13) ^ h[0].rotate_right(22);
        let maj = (h[0] & h[1]) ^ (h[0] & h[2]) ^ (h[1] & h[2]);
        let temp2 = s0.wrapping_add(maj);

        h[7] = h[6];
        h[6] = h[5];
        h[5] = h[4];
        h[4] = h[3].wrapping_add(temp1);
        h[3] = h[2];
        h[2] = h[1];
        h[1] = h[0];
        h[0] = temp1.wrapping_add(temp2);
    }

    // Update the current hash state
    // for i in 0..8 {
    //     state[i] = state[i].wrapping_add(h[i]);
    // }
    for (i, v) in state.iter_mut().enumerate() {
        *v = v.wrapping_add(h[i]);
    }
}

/// Return a hash value of a given input as string
///
/// Examples
///
/// ```
///use rsha::sha256;
///
///let inp = "hello world";
///let hash_val = sha256::hash(inp);
///println!("Hash of {inp} -> {hash_val}");
/// ```
pub fn hash(input: &str) -> String {
    let mut state = H;
    let bytes = input.as_bytes();
    let pad_msg = create_msg_padding(bytes);
    for blocks in pad_msg.chunks(64) {
        process_block(&mut state, blocks);
    }

    state.iter().map(|&x| format!("{:08x}", x)).collect()
}

///Return a hash value of a given input as a array of [u32;8]
///
///Examples
///
///```
///use rsha::sha256;
///
///let inp = "hello world";
///let hash_arr = sha256::hash_arr(inp);
///println!("Hash of {} -> {:?}",inp, hash_arr);
///```
pub fn hash_arr(input: &str) -> [u32; 8] {
    let mut state = H; // Initialize hash values
    let padded_msg = create_msg_padding(input.as_bytes());

    // Process each 512-bit block
    for block in padded_msg.chunks(64) {
        process_block(&mut state, block);
    }
    state
}
