use std::ops::{ Add, Div, Mul, Neg, Sub };

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum ValueType {
    Float64,
    Int64,
    Int32,
    String,
    Bool,
    Null,
    Dynamic,
    Empty,
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub struct Variable {
    name: String,
    value_type: ValueType,
}

impl Variable {
    pub fn new(name: String, value_type: ValueType) -> Self {
        Self { name, value_type }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_value_type(&self) -> ValueType {
        self.value_type.clone()
    }
}

#[derive(Debug, PartialEq, Clone, PartialOrd)]
pub enum Value {
    Float64(f64),
    // Float32(f32),
    Int64(i64),
    Int32(i32),
    // Int16(i16),
    // Int8(i8),
    // Int(isize),
    // UnsignedInt64(u64),
    // UnsignedInt32(u32),
    // UnsignedInt16(u16),
    // UnsignedInt8(u8),
    // UnsignedInt(usize),
    String(String),
    Bool(bool),
    Null,
    Variable(Variable),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Float64(a) => a.to_string(),
            Value::Int64(a) => a.to_string(),
            Value::Int32(a) => a.to_string(),
            Value::Bool(a) => a.to_string(),
            Value::Null => "null".to_string(),
            Value::String(a) => a.to_string(),
            Value::Variable(a) => a.name.to_string(),
        }
    }

    pub fn to_type_string(&self) -> String {
        match self {
            Value::Float64(_) => "Float64".to_string(),
            Value::Int64(_) => "Int64".to_string(),
            Value::Int32(_) => "Int32".to_string(),
            Value::Bool(_) => "Bool".to_string(),
            Value::Null => "Null".to_string(),
            Value::String(_) => "String".to_string(),
            Value::Variable(a) => format!("{:?}", a.value_type),
        }
    }

    pub fn is_falsey(&self) -> bool {
        matches!(self, Self::Null | Self::Bool(false))
    }
}

pub trait Pow: Sized {
    fn pow(self, exp: Self) -> Result<Self, String>;
}

impl Neg for Value {
    type Output = Result<Self, String>;

    fn neg(self) -> Result<Self, String> {
        match self {
            Value::Float64(a) => Ok(Value::Float64(-a)),
            Value::Int64(a) => Ok(Value::Int64(-a)),
            Value::Int32(a) => Ok(Value::Int32(-a)),
            _ => Err(format!("Cannot negate {:?}", self.to_type_string())),
        }
    }
}

impl Add for Value {
    type Output = Result<Self, String>;

    fn add(self, other: Self) -> Result<Self, String> {
        match (&self, &other) {
            (Value::Float64(a), Value::Float64(b)) => Ok(Value::Float64(a + b)),
            (Value::Int64(a), Value::Int64(b)) => Ok(Value::Int64(a + b)),
            (Value::Int32(a), Value::Int32(b)) => Ok(Value::Int32(a + b)),

            (Value::Int64(a), Value::Int32(b)) => Ok(Value::Int64(a + (*b as i64))),
            (Value::Int32(a), Value::Int64(b)) => Ok(Value::Int64((*a as i64) + b)),

            (Value::Float64(a), Value::Int64(b)) => Ok(Value::Float64(a + (*b as f64))),
            (Value::Int64(a), Value::Float64(b)) => Ok(Value::Float64((*a as f64) + b)),

            (Value::Float64(a), Value::Int32(b)) => Ok(Value::Float64(a + (*b as f64))),
            (Value::Int32(a), Value::Float64(b)) => Ok(Value::Float64((*a as f64) + b)),

            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),

            (Value::String(a), Value::Int64(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), Value::Int32(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), Value::Float64(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), Value::Bool(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), Value::Null) => Ok(Value::String(format!("{}null", a))),

            (Value::Int64(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::Int32(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::Float64(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::Bool(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::Null, Value::String(b)) => Ok(Value::String(format!("null{}", b))),

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

impl Sub for Value {
    type Output = Result<Self, String>;

    fn sub(self, other: Self) -> Result<Self, String> {
        match (&self, &other) {
            (Value::Float64(a), Value::Float64(b)) => Ok(Value::Float64(a - b)),
            (Value::Int64(a), Value::Int64(b)) => Ok(Value::Int64(a - b)),
            (Value::Int32(a), Value::Int32(b)) => Ok(Value::Int32(a - b)),

            (Value::Int64(a), Value::Int32(b)) => Ok(Value::Int64(a - (*b as i64))),
            (Value::Int32(a), Value::Int64(b)) => Ok(Value::Int64((*a as i64) - b)),

            (Value::Float64(a), Value::Int64(b)) => Ok(Value::Float64(a - (*b as f64))),
            (Value::Int64(a), Value::Float64(b)) => Ok(Value::Float64((*a as f64) - b)),

            (Value::Float64(a), Value::Int32(b)) => Ok(Value::Float64(a - (*b as f64))),
            (Value::Int32(a), Value::Float64(b)) => Ok(Value::Float64((*a as f64) - b)),

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

impl Mul for Value {
    type Output = Result<Self, String>;

    fn mul(self, other: Self) -> Result<Self, String> {
        match (&self, &other) {
            (Value::Float64(a), Value::Float64(b)) => Ok(Value::Float64(a * b)),
            (Value::Int64(a), Value::Int64(b)) => Ok(Value::Int64(a * b)),
            (Value::Int32(a), Value::Int32(b)) => Ok(Value::Int32(a * b)),

            (Value::Int64(a), Value::Int32(b)) => Ok(Value::Int64(a * (*b as i64))),
            (Value::Int32(a), Value::Int64(b)) => Ok(Value::Int64((*a as i64) * b)),

            (Value::Float64(a), Value::Int64(b)) => Ok(Value::Float64(a * (*b as f64))),
            (Value::Int64(a), Value::Float64(b)) => Ok(Value::Float64((*a as f64) * b)),

            (Value::Float64(a), Value::Int32(b)) => Ok(Value::Float64(a * (*b as f64))),
            (Value::Int32(a), Value::Float64(b)) => Ok(Value::Float64((*a as f64) * b)),
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

impl Div for Value {
    type Output = Result<Self, String>;

    fn div(self, other: Self) -> Result<Self, String> {
        match (&self, &other) {
            (Value::Float64(a), Value::Float64(b)) => Ok(Value::Float64(a / b)),
            (Value::Int64(a), Value::Int64(b)) => Ok(Value::Float64((*a as f64) / (*b as f64))),
            (Value::Int32(a), Value::Int32(b)) => Ok(Value::Float64((*a as f64) / (*b as f64))),

            (Value::Int64(a), Value::Int32(b)) => Ok(Value::Float64((*a as f64) / (*b as f64))),
            (Value::Int32(a), Value::Int64(b)) => Ok(Value::Float64((*a as f64) / (*b as f64))),

            (Value::Float64(a), Value::Int64(b)) => Ok(Value::Float64(a / (*b as f64))),
            (Value::Int64(a), Value::Float64(b)) => Ok(Value::Float64((*a as f64) / b)),

            (Value::Float64(a), Value::Int32(b)) => Ok(Value::Float64(a / (*b as f64))),
            (Value::Int32(a), Value::Float64(b)) => Ok(Value::Float64((*a as f64) / b)),
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

impl Pow for Value {
    fn pow(self, exp: Self) -> Result<Self, String> {
        match (&self, &exp) {
            (Value::Float64(a), Value::Float64(b)) => Ok(Value::Float64(a.powf(*b))),
            (Value::Int64(a), Value::Int64(b)) => Ok(Value::Float64((*a as f64).powf(*b as f64))),
            (Value::Int32(a), Value::Int32(b)) => Ok(Value::Float64((*a as f64).powf(*b as f64))),

            (Value::Int64(a), Value::Int32(b)) => Ok(Value::Float64((*a as f64).powf(*b as f64))),
            (Value::Int32(a), Value::Int64(b)) => Ok(Value::Float64((*a as f64).powf(*b as f64))),

            (Value::Float64(a), Value::Int64(b)) => Ok(Value::Float64(a.powf(*b as f64))),
            (Value::Int64(a), Value::Float64(b)) => Ok(Value::Float64((*a as f64).powf(*b))),

            (Value::Float64(a), Value::Int32(b)) => Ok(Value::Float64(a.powf(*b as f64))),
            (Value::Int32(a), Value::Float64(b)) => Ok(Value::Float64((*a as f64).powf(*b))),

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
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::with_capacity(256) }
    }

    pub fn write(&mut self, value: Value) -> usize {
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

    pub fn read(&self, index: usize) -> Value {
        self.values[index].clone()
    }
}
