use std::{ error::Error, ops::{ Add, Div, Mul, MulAssign, Neg, Sub } };

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValueType {
    Float64(f64),
    // Float32(f32),
    Int64(i64),
    // Int32(i32),
    // Int16(i16),
    // Int8(i8),
    // UnsignedInt64(u64),
    // UnsignedInt32(u32),
    // UnsignedInt16(u16),
    // UnsignedInt8(u8),
    Bool(bool),
    Null,
}

impl ValueType {
    pub fn to_type_string(&self) -> String {
        match self {
            ValueType::Float64(_) => "Float64".to_string(),
            ValueType::Int64(_) => "Int64".to_string(),
            ValueType::Bool(_) => "Bool".to_string(),
            ValueType::Null => "Null".to_string(),
        }
    }
}

pub trait Pow: Sized {
    fn pow(self, exp: Self) -> Result<Self, String>;
}

impl Neg for ValueType {
    type Output = Result<Self, String>;

    fn neg(self) -> Result<Self, String> {
        match self {
            ValueType::Float64(a) => Ok(ValueType::Float64(-a)),
            ValueType::Int64(a) => Ok(ValueType::Int64(-a)),
            _ => Err(format!("Cannot negate {:?}", self.to_type_string())),
        }
    }
}

impl Add for ValueType {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Result<Self, String> {
        match (&self, &other) {
            (ValueType::Float64(a), ValueType::Float64(b)) => Ok(ValueType::Float64(a + b)),
            (ValueType::Int64(a), ValueType::Int64(b)) => Ok(ValueType::Int64(a + b)),
            _ =>
                Err(
                    format!(
                        "Cannot add {:?} and {:?}",
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }
}

impl Sub for ValueType {
    type Output = Result<Self, String>;

    fn sub(self, other: Self) -> Result<Self, String> {
        match (&self, &other) {
            (ValueType::Float64(a), ValueType::Float64(b)) => Ok(ValueType::Float64(a - b)),
            (ValueType::Int64(a), ValueType::Int64(b)) => Ok(ValueType::Int64(a - b)),
            _ =>
                Err(
                    format!(
                        "Cannot subtract {:?} and {:?}",
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }
}

impl Mul for ValueType {
    type Output = Result<Self, String>;

    fn mul(self, other: Self) -> Result<Self, String> {
        match (&self, &other) {
            (ValueType::Float64(a), ValueType::Float64(b)) => Ok(ValueType::Float64(a * b)),
            (ValueType::Int64(a), ValueType::Int64(b)) => Ok(ValueType::Int64(a * b)),
            _ =>
                Err(
                    format!(
                        "Cannot multiply {:?} and {:?}",
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }
}

impl Div for ValueType {
    type Output = Result<Self, String>;

    fn div(self, other: Self) -> Result<Self, String> {
        match (&self, &other) {
            (ValueType::Float64(a), ValueType::Float64(b)) => Ok(ValueType::Float64(a / b)),
            (ValueType::Int64(a), ValueType::Int64(b)) => Ok(ValueType::Int64(a / b)),
            _ =>
                Err(
                    format!(
                        "Cannot divide {:?} and {:?}",
                        self.to_type_string(),
                        other.to_type_string()
                    )
                ),
        }
    }
}

impl Pow for ValueType {
    fn pow(self, exp: Self) -> Result<Self, String> {
        match (&self, &exp) {
            (ValueType::Float64(a), ValueType::Float64(b)) => Ok(ValueType::Float64(a.powf(*b))),
            (ValueType::Int64(a), ValueType::Int64(b)) => Ok(ValueType::Int64(a.pow(*b as u32))),
            _ =>
                Err(
                    format!(
                        "Cannot raise {:?} to the power of {:?}",
                        self.to_type_string(),
                        exp.to_type_string()
                    )
                ),
        }
    }
}

#[derive(Debug)]
pub struct ValueArray {
    values: Vec<ValueType>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::with_capacity(256) }
    }

    pub fn write(&mut self, value: ValueType) -> usize {
        let count = self.values.len();
        self.values.push(value);
        count
    }

    pub fn free(&mut self) {
        self.values.clear();
    }

    #[cfg(any(feature = "debug_trace_execution", feature = "debug_print_code"))]
    pub fn print_value(&self, constant: u8) {
        print!("{:?}", self.values[constant as usize])
    }

    pub fn read(&self, index: usize) -> ValueType {
        self.values[index]
    }
}
