// TODO: Implement sha2.
use sha2::{Digest, Sha512};

pub trait AsBytes: Copy {
    fn as_bytes(&self) -> &[u8];
}

impl AsBytes for &str {
    fn as_bytes(&self) -> &[u8] {
        return (*self).as_bytes();
    }
}

impl AsBytes for &[u8] {
    fn as_bytes(&self) -> &[u8] {
        return *self;
    }
}

pub enum Endian {
    Big,
    Little,
}

fn md_padding(block_size: usize, message_length: usize, endian: Endian) -> Vec<u8> {
    // Message length is in bytes.
    let length_in_bits: u64 = (message_length * 8) as u64;
    let length_in_bytes: [u8; 8];
    match endian {
        Endian::Big => length_in_bytes = length_in_bits.to_be_bytes(),
        Endian::Little => length_in_bytes = length_in_bits.to_le_bytes(),
    }

    // Padding in bytes.
    let remainder = (message_length + 1 + 8) % block_size;
    let mut zeros_to_pad = 0;

    // Checks edge case if there are no zeros to pad when the message length and
    // appended bit fill available slots.
    if remainder > 0 {
        zeros_to_pad = block_size - remainder;
    }

    // Append 1 bit and then add the rest of the padding.
    let mut preprocessed_bytes: Vec<u8> = vec![1 << 7];
    preprocessed_bytes.extend(vec![0; zeros_to_pad]);
    preprocessed_bytes.extend(length_in_bytes);
    preprocessed_bytes
}

pub fn md_padding_64(message_length: usize, endian: Endian) -> Vec<u8> {
    md_padding(64, message_length, endian)
}

pub fn md_padding_128(message_length: usize, endian: Endian) -> Vec<u8> {
    md_padding(128, message_length, endian)
}

pub mod md4 {
    use super::*;

    #[allow(non_snake_case)]
    fn F(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | ((!x) & z)
    }

    #[allow(non_snake_case)]
    fn G(x: u32, y: u32, z: u32) -> u32 {
        (x & y) | (x & z) | (y & z)
    }

    #[allow(non_snake_case)]
    fn H(x: u32, y: u32, z: u32) -> u32 {
        x ^ y ^ z
    }

    fn round(
        a: &mut u32,
        b: u32,
        c: u32,
        d: u32,
        func: fn(u32, u32, u32) -> u32,
        word: u32,
        s: u32,
        constant: u32,
    ) {
        *a = ((*a)
            .wrapping_add(func(b, c, d))
            .wrapping_add(word)
            .wrapping_add(constant))
        .rotate_left(s);
    }

    pub fn core<T>(message: T, total_length: usize, iv: (u32, u32, u32, u32)) -> [u8; 16]
    where
        T: AsBytes,
    {
        // Initialize starting variables
        let mut a: u32 = iv.0;
        let mut b: u32 = iv.1;
        let mut c: u32 = iv.2;
        let mut d: u32 = iv.3;

        let bytes = message.as_bytes();
        let processed = [
            bytes,
            md_padding_64(total_length, Endian::Little).as_slice(),
        ]
        .concat();

        for chunk in processed.chunks(64) {
            let mut extended_words: [u32; 16] = [0; 16];
            for (i, ele) in chunk.chunks(4).enumerate() {
                let to_32_bit: [u8; 4] = [ele[0], ele[1], ele[2], ele[3]];
                extended_words[i] = u32::from_le_bytes(to_32_bit);
            }

            let aa = a;
            let bb = b;
            let cc = c;
            let dd = d;

            // Round 1.
            round(&mut a, b, c, d, F, extended_words[0], 3, 0);
            round(&mut d, a, b, c, F, extended_words[1], 7, 0);
            round(&mut c, d, a, b, F, extended_words[2], 11, 0);
            round(&mut b, c, d, a, F, extended_words[3], 19, 0);
            round(&mut a, b, c, d, F, extended_words[4], 3, 0);
            round(&mut d, a, b, c, F, extended_words[5], 7, 0);
            round(&mut c, d, a, b, F, extended_words[6], 11, 0);
            round(&mut b, c, d, a, F, extended_words[7], 19, 0);
            round(&mut a, b, c, d, F, extended_words[8], 3, 0);
            round(&mut d, a, b, c, F, extended_words[9], 7, 0);
            round(&mut c, d, a, b, F, extended_words[10], 11, 0);
            round(&mut b, c, d, a, F, extended_words[11], 19, 0);
            round(&mut a, b, c, d, F, extended_words[12], 3, 0);
            round(&mut d, a, b, c, F, extended_words[13], 7, 0);
            round(&mut c, d, a, b, F, extended_words[14], 11, 0);
            round(&mut b, c, d, a, F, extended_words[15], 19, 0);

            // Round 2.
            round(&mut a, b, c, d, G, extended_words[0], 3, 0x5A827999);
            round(&mut d, a, b, c, G, extended_words[4], 5, 0x5A827999);
            round(&mut c, d, a, b, G, extended_words[8], 9, 0x5A827999);
            round(&mut b, c, d, a, G, extended_words[12], 13, 0x5A827999);
            round(&mut a, b, c, d, G, extended_words[1], 3, 0x5A827999);
            round(&mut d, a, b, c, G, extended_words[5], 5, 0x5A827999);
            round(&mut c, d, a, b, G, extended_words[9], 9, 0x5A827999);
            round(&mut b, c, d, a, G, extended_words[13], 13, 0x5A827999);
            round(&mut a, b, c, d, G, extended_words[2], 3, 0x5A827999);
            round(&mut d, a, b, c, G, extended_words[6], 5, 0x5A827999);
            round(&mut c, d, a, b, G, extended_words[10], 9, 0x5A827999);
            round(&mut b, c, d, a, G, extended_words[14], 13, 0x5A827999);
            round(&mut a, b, c, d, G, extended_words[3], 3, 0x5A827999);
            round(&mut d, a, b, c, G, extended_words[7], 5, 0x5A827999);
            round(&mut c, d, a, b, G, extended_words[11], 9, 0x5A827999);
            round(&mut b, c, d, a, G, extended_words[15], 13, 0x5A827999);

            // Round 3.
            round(&mut a, b, c, d, H, extended_words[0], 3, 0x6ED9EBA1);
            round(&mut d, a, b, c, H, extended_words[8], 9, 0x6ED9EBA1);
            round(&mut c, d, a, b, H, extended_words[4], 11, 0x6ED9EBA1);
            round(&mut b, c, d, a, H, extended_words[12], 15, 0x6ED9EBA1);
            round(&mut a, b, c, d, H, extended_words[2], 3, 0x6ED9EBA1);
            round(&mut d, a, b, c, H, extended_words[10], 9, 0x6ED9EBA1);
            round(&mut c, d, a, b, H, extended_words[6], 11, 0x6ED9EBA1);
            round(&mut b, c, d, a, H, extended_words[14], 15, 0x6ED9EBA1);
            round(&mut a, b, c, d, H, extended_words[1], 3, 0x6ED9EBA1);
            round(&mut d, a, b, c, H, extended_words[9], 9, 0x6ED9EBA1);
            round(&mut c, d, a, b, H, extended_words[5], 11, 0x6ED9EBA1);
            round(&mut b, c, d, a, H, extended_words[13], 15, 0x6ED9EBA1);
            round(&mut a, b, c, d, H, extended_words[3], 3, 0x6ED9EBA1);
            round(&mut d, a, b, c, H, extended_words[11], 9, 0x6ED9EBA1);
            round(&mut c, d, a, b, H, extended_words[7], 11, 0x6ED9EBA1);
            round(&mut b, c, d, a, H, extended_words[15], 15, 0x6ED9EBA1);

            a = a.wrapping_add(aa);
            b = b.wrapping_add(bb);
            c = c.wrapping_add(cc);
            d = d.wrapping_add(dd);
        }

        // Produce the final hash value (big-endian) as a 128-bit number.
        let mut output: [u8; 16] = [0; 16];
        for (i, byte) in a
            .to_le_bytes()
            .iter()
            .chain(b.to_le_bytes().iter())
            .chain(c.to_le_bytes().iter())
            .chain(d.to_le_bytes().iter())
            .enumerate()
        {
            output[i] = *byte;
        }
        output
    }

    pub fn default<T>(message: T) -> [u8; 16]
    where
        T: AsBytes,
    {
        core(
            message,
            message.as_bytes().len(),
            // IV from the spec.
            // https://datatracker.ietf.org/doc/html/rfc1320#section-3-3
            (0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476),
        )
    }
}

pub mod sha1 {
    use super::*;

    pub fn core<T>(message: T, total_length: usize, iv: (u32, u32, u32, u32, u32)) -> [u8; 20]
    where
        T: AsBytes,
    {
        // Initialize starting variables
        let mut h0: u32 = iv.0;
        let mut h1: u32 = iv.1;
        let mut h2: u32 = iv.2;
        let mut h3: u32 = iv.3;
        let mut h4: u32 = iv.4;

        let bytes = message.as_bytes();
        let processed = [bytes, md_padding_64(total_length, Endian::Big).as_slice()].concat();

        for chunk in processed.chunks(64) {
            let mut extended_words: [u32; 80] = [0; 80];
            for (i, ele) in chunk.chunks(4).enumerate() {
                let to_32_bit: [u8; 4] = [ele[0], ele[1], ele[2], ele[3]];
                extended_words[i] = u32::from_be_bytes(to_32_bit);
            }

            for i in 16..80 {
                extended_words[i] = extended_words[i - 3]
                    ^ extended_words[i - 8]
                    ^ extended_words[i - 14]
                    ^ extended_words[i - 16];
                extended_words[i] = extended_words[i].rotate_left(1);
            }

            let mut a = h0;
            let mut b = h1;
            let mut c = h2;
            let mut d = h3;
            let mut e = h4;

            for i in 0..80 {
                let f: u32;
                let k: u32;
                if i <= 19 {
                    f = (b & c) ^ ((!b) & d);
                    k = 0x5A827999
                } else if i <= 39 {
                    f = b ^ c ^ d;
                    k = 0x6ED9EBA1;
                } else if i <= 59 {
                    f = (b & c) ^ (b & d) ^ (c & d);
                    k = 0x8F1BBCDC;
                } else {
                    f = b ^ c ^ d;
                    k = 0xCA62C1D6;
                }

                let temp = a
                    .rotate_left(5)
                    .wrapping_add(f)
                    .wrapping_add(e)
                    .wrapping_add(k)
                    .wrapping_add(extended_words[i]);
                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }

            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
            h4 = h4.wrapping_add(e);
        }

        // Produce the final hash value (big-endian) as a 160-bit number.
        let mut output: [u8; 20] = [0; 20];
        for (i, byte) in h0
            .to_be_bytes()
            .iter()
            .chain(h1.to_be_bytes().iter())
            .chain(h2.to_be_bytes().iter())
            .chain(h3.to_be_bytes().iter())
            .chain(h4.to_be_bytes().iter())
            .enumerate()
        {
            output[i] = *byte;
        }
        output
    }

    pub fn default<T>(message: T) -> [u8; 20]
    where
        T: AsBytes,
    {
        core(
            message,
            message.as_bytes().len(),
            // IV from the spec.
            // https://www.rfc-editor.org/rfc/rfc3174#section-6.1
            (0x67452301, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0),
        )
    }
}

pub fn sha512_n(bytes: &[u8], n: usize) -> Vec<u8> {
    // Enforce constraint without error.
    let mut enn = n;
    if n > 6 {
        enn = 6;
    }

    Sha512::digest(bytes)
        .iter()
        .rev()
        .cloned()
        .take(enn)
        .collect()
}

// Helper function.
pub fn sha1<T>(message: T) -> [u8; 20]
where
    T: AsBytes,
{
    sha1::default(message)
}

pub fn md4<T>(message: T) -> [u8; 16]
where
    T: AsBytes,
{
    md4::default(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1() {
        let empty_output = sha1("");
        let dog_output = sha1("The quick brown fox jumps over the lazy dog");
        let cog_output = sha1("The quick brown fox jumps over the lazy cog");
        let long_output = sha1(
            "abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmn\
            hijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu",
        );
        let edge_case_output = sha1(
            [
                99, 26, 143, 85, 115, 125, 249, 19, 75, 135, 222, 119, 109, 98, 105, 96, 2, 41,
                217, 139, 169, 160, 123, 50, 113, 131, 231, 95, 116, 91, 204, 125, 57, 255, 248,
                23, 109, 181, 125, 51, 155, 185, 229, 133, 69, 150, 225, 19, 208, 36, 183, 214, 2,
                145, 252,
            ]
            .as_slice(),
        );

        let expected_empty_output: [u8; 20] = [
            0xDA, 0x39, 0xA3, 0xEE, 0x5E, 0x6B, 0x4B, 0x0D, 0x32, 0x55, 0xBF, 0xEF, 0x95, 0x60,
            0x18, 0x90, 0xAF, 0xD8, 0x07, 0x09,
        ];
        let expected_dog_output: [u8; 20] = [
            0x2F, 0xD4, 0xE1, 0xC6, 0x7A, 0x2D, 0x28, 0xFC, 0xED, 0x84, 0x9E, 0xE1, 0xBB, 0x76,
            0xE7, 0x39, 0x1B, 0x93, 0xEB, 0x12,
        ];
        let expected_cog_output: [u8; 20] = [
            0xDE, 0x9F, 0x2C, 0x7F, 0xD2, 0x5E, 0x1B, 0x3A, 0xFA, 0xD3, 0xE8, 0x5A, 0x0B, 0xD1,
            0x7D, 0x9B, 0x10, 0x0D, 0xB4, 0xB3,
        ];
        let expected_long_output: [u8; 20] = [
            0xA4, 0x9B, 0x24, 0x46, 0xA0, 0x2C, 0x64, 0x5B, 0xF4, 0x19, 0xF9, 0x95, 0xB6, 0x70,
            0x91, 0x25, 0x3A, 0x04, 0xA2, 0x59,
        ];
        let expected_edge_case_output: [u8; 20] = [
            0x03, 0x17, 0xE3, 0x8D, 0x99, 0xCD, 0xBA, 0x10, 0xF6, 0x05, 0x77, 0x6B, 0xF3, 0xCF,
            0xCD, 0x89, 0xBC, 0xDE, 0x76, 0xBB,
        ];

        assert_eq!(empty_output, expected_empty_output);
        assert_eq!(dog_output, expected_dog_output);
        assert_eq!(cog_output, expected_cog_output);
        assert_eq!(long_output, expected_long_output);
        assert_eq!(edge_case_output, expected_edge_case_output);
    }

    #[test]
    fn test_sha512_n() {
        let test_vec: Vec<u8> = vec![1];
        let hash_of_test_vec = sha512_n(test_vec.as_slice(), 10);
        assert_eq!(hash_of_test_vec.len(), 6);

        let other_hash_of_test_vec = sha512_n(test_vec.as_slice(), 6);
        let another_hash = sha512_n(vec![2].as_slice(), 6);

        assert_eq!(hash_of_test_vec, other_hash_of_test_vec);
        assert_ne!(another_hash, other_hash_of_test_vec);
    }

    #[test]
    fn test_md4() {
        let empty_output = md4("");
        let a_output = md4("a");
        let medium_output = md4("abcdefghijklmnopqrstuvwxyz");
        let long_output = md4("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789");
        let edge_case_output = md4([
            99, 26, 143, 85, 115, 125, 249, 19, 75, 135, 222, 119, 109, 98, 105, 96, 2, 41, 217,
            139, 169, 160, 123, 50, 113, 131, 231, 95, 116, 91, 204, 125, 57, 255, 248, 23, 109,
            181, 125, 51, 155, 185, 229, 133, 69, 150, 225, 19, 208, 36, 183, 214, 2, 145, 252,
        ]
        .as_slice());

        let expected_empty_output: [u8; 16] = [
            0x31, 0xD6, 0xCF, 0xE0, 0xD1, 0x6A, 0xE9, 0x31, 0xB7, 0x3C, 0x59, 0xD7, 0xE0, 0xC0,
            0x89, 0xC0,
        ];
        let expected_a_output: [u8; 16] = [
            0xBD, 0xE5, 0x2C, 0xB3, 0x1D, 0xE3, 0x3E, 0x46, 0x24, 0x5E, 0x05, 0xFB, 0xDB, 0xD6,
            0xFB, 0x24,
        ];
        let expected_medium_output: [u8; 16] = [
            0xD7, 0x9E, 0x1C, 0x30, 0x8A, 0xA5, 0xBB, 0xCD, 0xEE, 0xA8, 0xED, 0x63, 0xDF, 0x41,
            0x2D, 0xA9,
        ];
        let expected_long_output: [u8; 16] = [
            0x04, 0x3F, 0x85, 0x82, 0xF2, 0x41, 0xDB, 0x35, 0x1C, 0xE6, 0x27, 0xE1, 0x53, 0xE7,
            0xF0, 0xE4,
        ];
        let expected_edge_case_output: [u8; 16] = [
            0x67, 0x04, 0xB8, 0x49, 0x3D, 0xDC, 0x39, 0x94, 0x82, 0xF0, 0x48, 0x5F, 0x73, 0x1E,
            0x64, 0x63,
        ];

        assert_eq!(empty_output, expected_empty_output);
        assert_eq!(a_output, expected_a_output);
        assert_eq!(medium_output, expected_medium_output);
        assert_eq!(long_output, expected_long_output);
        assert_eq!(edge_case_output, expected_edge_case_output);
    }
}
