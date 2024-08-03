pub fn pow2(n: usize) -> usize {
    let n: u32 = n.try_into().expect("Cannot cast usize to u32");
    usize::pow(2, n)
}

pub fn bits_to_int<const N: usize>(bits: &[bool; N]) -> usize {
    bits.iter()
        .rev()
        .enumerate()
        .map(|(i, b)| if *b { pow2(i) } else { 0 })
        .sum()
}

pub fn int_to_bits<const N: usize>(int: usize) -> [bool; N] {
    let mut bits = [false; N];
    bits.iter_mut()
        .rev()
        .enumerate()
        .for_each(|(i, b)| *b = (int >> i) & 1 == 1);
    bits
}
