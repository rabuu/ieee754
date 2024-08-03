use crate::Ieee754;

impl<const R: usize, const P: usize> Ieee754<R, P> {
    /// Parse a float number from String
    pub fn parse(input: impl AsRef<str>) -> Result<Self, String> {
        let mut chars = input.as_ref().chars().filter(|c| *c != ' ');

        let input_count = chars.clone().count();

        if input_count == 0 {
            return Err("Empty string cannot be parsed".to_string());
        }

        if input_count != Self::DIGITS {
            return Err(format!(
                "Number must have {} digits but has {input_count}",
                Self::DIGITS
            ));
        }

        let sign = chars.next().unwrap() == '1';

        let mut exp = [false; R];
        for bit in exp.iter_mut() {
            *bit = match chars.next() {
                Some('0') => false,
                Some('1') => true,
                other => return Err(format!("Neither 0 nor 1: {other:?}")),
            };
        }

        let mut mantissa = [false; P];
        for bit in mantissa.iter_mut() {
            *bit = match chars.next() {
                Some('0') => false,
                Some('1') => true,
                other => return Err(format!("Neither 0 nor 1: {other:?}")),
            };
        }

        if chars.count() != 0 {
            return Err("Too many bits".to_string());
        }

        Ok(Self {
            sign,
            exp,
            mantissa,
        })
    }
}
