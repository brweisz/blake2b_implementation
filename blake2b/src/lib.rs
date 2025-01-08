// DATA
// +--------------+------------------+
// |              | BLAKE2b          |
// +--------------+------------------+
// | Bits in word | w  = 64          |
// | Rounds in F  | r  = 12          |
// | Block bytes  | bb = 128         |
// | Hash bytes   | 1 <= nn <= 64    |
// | Key bytes    | 0 <= kk <= 64    |
// | Input bytes  | 0 <= ll < 2**128 |
// +--------------+------------------+
// | G Rotation   | (R1, R2, R3, R4) |
// |  constants   | (32, 24, 16, 63) |
// +--------------+------------------+

// Constants
const BLAKE2B_IV: [u64; 8] = [
    0x6A09E667F3BCC908,
    0xBB67AE8584CAA73B,
    0x3C6EF372FE94F82B,
    0xA54FF53A5F1D36F1,
    0x510E527FADE682D1,
    0x9B05688C2B3E6C1F,
    0x1F83D9ABFB41BD6B,
    0x5BE0CD19137E2179,
];

const SIGMA: [[u8; 16]; 12] = [
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
[14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3],
[11, 8, 12, 0, 5, 2, 15, 13, 10, 14, 3, 6, 7, 1, 9, 4],
[7, 9, 3, 1, 13, 12, 11, 14, 2, 6, 5, 10, 4, 0, 15, 8],
[9, 0, 5, 7, 2, 4, 10, 15, 14, 1, 11, 12, 6, 8, 3, 13],
[2, 12, 6, 10, 0, 11, 8, 3, 4, 13, 7, 5, 15, 14, 1, 9],
[12, 5, 1, 15, 14, 13, 4, 10, 0, 7, 6, 3, 9, 2, 8, 11],
[13, 11, 7, 14, 12, 1, 3, 9, 5, 0, 15, 4, 8, 6, 2, 10],
[6, 15, 14, 9, 11, 3, 0, 8, 12, 2, 13, 7, 1, 4, 10, 5],
[10, 2, 8, 4, 7, 6, 1, 5, 15, 11, 9, 14, 3, 12, 13, 0],
[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
[14, 10, 4, 8, 9, 15, 13, 6, 1, 12, 0, 2, 11, 7, 5, 3]];

// Context

struct Blake2bCtx {
    b: [u8; 128],  // input buffer
    h: [u64; 8],   // chained state, es como el acumulador de la compression function
    t: [u64; 2],   // total number of bytes, low part y high part del número (pq el mensaje puede ser de hasta 2^128 bytes)
    c: usize,      // pointer for b[]
    outlen: usize, // digest size
}

impl Blake2bCtx {
    fn new(key: &mut Vec<u8>, outlen: usize) -> Self {
        let mut h: [u64; 8] = BLAKE2B_IV.clone();
        h[0] = h[0] ^ 0x01010000 ^ (key.len() << 8) as u64 ^ outlen as u64;

        Self { b: [0; 128], h, t: [0; 2], c: 0, outlen }
    }
}

// Hash Function

pub fn blake2b(
    out: &mut Vec<u8>,
    key: &mut Vec<u8>,
    input_message: &mut Vec<u8>,
) -> i32 {
    if (out.len() == 0 || out.len() > 64 || key.len() > 64){
        panic!("Illegal input parameters")
    }
    let mut ctx = Blake2bCtx::new(key, out.len());

    if (key.len() > 0){
        blake2b_update(&mut ctx, key);
        ctx.c = 128;
    }
    blake2b_update(&mut ctx, &mut input_message);

    0
}

fn blake2b_update(ctx: &mut Blake2bCtx, input: &mut Vec<u8>){
    for i in 0..input.len(){
        if (ctx.c == 128){
            ctx.t[0] += ctx.c as u64;
            if (ctx.t[0] < ctx.c as u64){ ctx.t[1] += 1; }
            blake2b_compress(ctx, 0);
            ctx.c = 0;
        }
        ctx.b[ctx.c] = input[i];
        ctx.c += 1;
    }
}

fn blake2b_compress(ctx: &mut Blake2bCtx, last: i32){

}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
