use std::fmt;
use std::fmt::Write;

use util::*;
pub use value::Value;

mod parse;
mod util;
mod value;

/// A custom 7-bit float type
pub type SmallFloat = Ieee754<3, 3>;

/// A single-precision float type
pub type SingleFloat = Ieee754<8, 23>;

/// A double-precision float type
pub type DoubleFloat = Ieee754<11, 52>;

/// A quadruple-precision float type
pub type QuadrupleFloat = Ieee754<15, 112>;

/// A IEEE-754 floating point number, generic over R exponent bits and P mantissa bits
#[derive(Debug)]
pub struct Ieee754<const R: usize, const P: usize> {
    sign: bool,
    exp: [bool; R],
    mantissa: [bool; P],
}

impl<const R: usize, const P: usize> Ieee754<R, P> {
    /// The number of digits the float format has
    pub const DIGITS: usize = 1 + R + P;

    /// Construct a float number
    pub fn new(sign: bool, exp: [bool; R], mantissa: [bool; P]) -> Self {
        Self {
            sign,
            exp,
            mantissa,
        }
    }

    /// Generate all possible SmallFloats
    pub fn generate_all() -> Vec<Self> {
        let mut floats = Vec::with_capacity(pow2(Self::DIGITS));

        for sign in [false, true] {
            for exp in 0..pow2(R) {
                let exp_bits = int_to_bits(exp);
                for mantissa in 0..pow2(P) {
                    let mantissa_bits = int_to_bits(mantissa);
                    floats.push(Self::new(sign, exp_bits, mantissa_bits));
                }
            }
        }

        floats
    }

    /// Evaluate a IEEE-754 float into a [Value]
    pub fn evaluate(&self) -> Value {
        if self.exp() == 0 && self.mantissa() == 0 {
            match self.sign() {
                true => Value::NegativeZero,
                false => Value::PositiveZero,
            }
        } else if self.exp() == Ieee754::<R, P>::max_exp() && self.mantissa() == 0 {
            match self.sign() {
                true => Value::NegativeInfinity,
                false => Value::PositiveInfinity,
            }
        } else if self.exp() == Ieee754::<R, P>::max_exp() {
            Value::NaN
        } else if self.exp() == 0 {
            Value::Number(self.denormalized_value(), true)
        } else {
            Value::Number(self.raw_value(), false)
        }
    }

    /// Get the raw value of a IEEE-754 float without checking for special cases
    pub fn raw_value(&self) -> f32 {
        let s = self.sign_f32();
        let f = 1.0 + (self.mantissa() as f32 / pow2(P) as f32);
        let e = self.exp() as f32 - Ieee754::<R, P>::bias() as f32;

        s * f * 2.0_f32.powf(e)
    }

    /// Get the denormalized value of a IEEE-754 float
    pub fn denormalized_value(&self) -> f32 {
        self.sign_f32()
            * (self.mantissa() as f32 / pow2(P) as f32)
            * 2.0_f32.powf(1.0 - Ieee754::<R, P>::bias() as f32)
    }

    /// Get the exponential offset of a Ieee754<R,P>
    fn bias() -> usize {
        pow2(R - 1) - 1
    }

    /// Get the maximum exponent value of a Ieee754<R,P>
    fn max_exp() -> usize {
        pow2(R) - 1
    }

    /// Whether the sign bit is 1 or zero
    fn sign(&self) -> bool {
        self.sign
    }

    /// Get the sign as numerical value: negative -> -1, positive -> 1
    fn sign_f32(&self) -> f32 {
        match self.sign {
            true => -1.0,
            false => 1.0,
        }
    }

    /// Get the exponent as integer
    fn exp(&self) -> usize {
        bits_to_int(&self.exp)
    }

    /// Get the mantissa as integer
    fn mantissa(&self) -> usize {
        bits_to_int(&self.mantissa)
    }
}

impl<const R: usize, const P: usize> fmt::Display for Ieee754<R, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(if self.sign { '1' } else { '0' })?;

        for i in 0..R {
            f.write_char(if self.exp[i] { '1' } else { '0' })?;
        }

        for i in 0..P {
            f.write_char(if self.mantissa[i] { '1' } else { '0' })?;
        }

        Ok(())
    }
}
