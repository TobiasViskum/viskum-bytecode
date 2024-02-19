pub type Value = f64;

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

    #[cfg(feature = "debug_trace_execution")]
    pub fn print_value(&self, constant: u8) {
        print!("{}", self.values[constant as usize])
    }

    pub fn read(&self, index: usize) -> Value {
        self.values[index]
    }
}
