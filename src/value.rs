use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Value {
    NaN,
    NegativeZero,
    PositiveZero,
    Number(f32, bool),
    NegativeInfinity,
    PositiveInfinity,
}

impl Value {
    pub fn is_nan(&self) -> bool {
        *self == Value::NaN
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::NaN => write!(f, "NaN"),
            Value::NegativeZero => write!(f, "-0"),
            Value::PositiveZero => write!(f, "0"),
            Value::Number(num, denorm) => {
                write!(f, "{num}")?;
                if *denorm {
                    write!(f, " (denorm)")?;
                }
                Ok(())
            }
            Value::NegativeInfinity => write!(f, "-Infinity"),
            Value::PositiveInfinity => write!(f, "Infinity"),
        }
    }
}
