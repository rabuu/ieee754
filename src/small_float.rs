use crate::util::*;
use crate::Ieee754;

// A small 7-bit float type (3 exponent, 3 mantissa bits)
pub type SmallFloat = Ieee754<3, 3>;

impl SmallFloat {
    /// Generate all possible SmallFloats
    pub fn generate_all() -> Vec<Self> {
        let mut floats = Vec::with_capacity(pow2(7));

        for sign in [false, true] {
            for exp in 0..8 {
                let exp_bits = int_to_bits(exp);
                for mantissa in 0..8 {
                    let mantissa_bits = int_to_bits(mantissa);
                    floats.push(SmallFloat::new(sign, exp_bits, mantissa_bits));
                }
            }
        }

        floats
    }
}
