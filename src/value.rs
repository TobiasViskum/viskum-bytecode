pub type Value = f64;

pub struct ValueArray {
    values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) -> usize {
        let count = self.values.len();
        self.values.push(value);
        count
    }

    pub fn free(&mut self) {
        self.values.clear();
    }

    pub fn print_value(&self, constant: u8) {
        print!("{}", self.values[constant as usize])
    }

    pub fn read(&self, index: usize) -> Value {
        self.values[index]
    }
}
