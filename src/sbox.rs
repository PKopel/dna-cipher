use crate::dna::{self, binary_to_DNA, DNA};

#[derive(Copy, Clone, Debug)]
pub struct SBox {
    sbox: [[DNA; 4]; 256],
}

#[inline]
fn rotl8(x: u8, shift: u8) -> u8 {
    return ((x) << (shift)) | ((x) >> (8 - (shift)));
}

impl SBox {
    pub fn new() -> SBox {
        let mut p: u8 = 1;
        let mut q: u8 = 1;
        let mut sbox = [[DNA::A; 4]; 256];

        /* loop invariant: p * q == 1 in the Galois field */
        loop {
            /* multiply p by 3 */
            p = p ^ (p << 1) ^ (if p & 0x80 != 0 { 0x1B } else { 0 });

            /* divide q by 3 (equals multiplication by 0xf6) */
            q ^= q << 1;
            q ^= q << 2;
            q ^= q << 4;
            q ^= if q & 0x80 != 0 { 0x09 } else { 0 };

            /* compute the affine transformation */
            let xformed: u8 = q ^ rotl8(q, 1) ^ rotl8(q, 2) ^ rotl8(q, 3) ^ rotl8(q, 4) ^ 0x63;

            sbox[usize::from(p)] = binary_to_DNA(&xformed);
            if p == 1 {
                break;
            }
        }

        /* 0 is a special case since it has no inverse */
        sbox[0] = binary_to_DNA(&0x63);
        return SBox { sbox };
    }

    pub fn get(&self, dna: &[DNA; 4]) -> [DNA; 4] {
        let index = dna::DNA_to_binary(&dna);
        return self.sbox[usize::from(index)];
    }
}
