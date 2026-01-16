pub(crate) fn b64_encode_48(input: &[u8; 6]) -> String {
    let mut output = [0 as char; 8];
    for (byte_chunk, output_slice) in input.chunks_exact(3).zip(output.chunks_exact_mut(4)) {
        let byte1 = byte_chunk[0];
        let byte2 = byte_chunk[1];
        let byte3 = byte_chunk[2];

        output_slice[0] = lookup((byte1 & 0b1111_1100) >> 2);
        output_slice[1] = lookup((byte1 & 0b0000_0011) << 4 | (byte2 & 0b1111_0000) >> 4);
        output_slice[2] = lookup((byte2 & 0b0000_1111) << 2 | (byte3 & 0b1100_0000) >> 6);
        output_slice[3] = lookup(byte3 & 0b0011_1111);
    }

    output.iter().collect()
}

pub(crate) fn b64_decode_48(input: &[u8; 8], output: &mut [u8; 6]) {
    for (char_chunk, output_slice) in input.chunks_exact(4).zip(output.chunks_exact_mut(3)) {
        let char_1 = reverse_lookup(char_chunk[0] as char);
        let char_2 = reverse_lookup(char_chunk[1] as char);
        let char_3 = reverse_lookup(char_chunk[2] as char);
        let char_4 = reverse_lookup(char_chunk[3] as char);

        output_slice[0] = (char_1 << 2) | (char_2 >> 4);
        output_slice[1] = (char_2 << 4) | (char_3 >> 2);
        output_slice[2] = (char_3 << 6) | char_4;
    }
}

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

fn lookup(idx: u8) -> char {
    ALPHABET
        .chars()
        .nth(idx as usize)
        .unwrap()
}

fn reverse_lookup(c: char) -> u8 {
    ALPHABET
        .chars()
        .position(|x| x == c)
        .unwrap() as u8
}
