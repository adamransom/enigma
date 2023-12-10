pub const ALPHA_LENGTH: usize = 26;

pub fn try_to_alphabet_index(c: u8) -> Result<usize, String> {
    if (65..=90).contains(&c) {
        Ok(to_alphabet_index(c))
    } else {
        Err("not an uppercase alphabet character".to_string())
    }
}

pub const fn to_alphabet_index(c: u8) -> usize {
    (c - 65) as usize
}
