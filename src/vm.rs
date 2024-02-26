use crate::chunk::Chunk;
use crate::compiler::Compiler;
use crate::opcodes::OpCode;
use crate::value::{ Pow, Value, ValueType, Variable };

use std::collections::hash_map::Entry;
use std::collections::HashMap;
#[cfg(feature = "debug_trace_execution")]
use std::time::{ SystemTime, UNIX_EPOCH };

#[derive(Debug, PartialEq)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
    Debug(Value),
}

#[derive(Debug)]
pub struct VM {
    chunk: Option<Chunk>,
    ip: usize,
    stack: Vec<Value>,
    had_runtime_error: bool,
    globals: HashMap<String, Variable>,
    // stack_top: Value,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunk: None,
            ip: 0,
            stack: Vec::with_capacity(256),
            had_runtime_error: false,
            globals: HashMap::with_capacity(4),
        }
    }

    pub fn get_stack(&self) -> &Vec<Value> {
        &self.stack
    }

    pub fn reset_stack(&mut self) {
        self.stack.clear();
    }

    pub fn free_vm(&mut self) {
        self.chunk = None;
        self.ip = 0;
        self.stack.clear()
    }

    pub fn init_chunk(&mut self, chunk: Chunk) {
        self.chunk = Some(chunk);
        self.ip = 0;
    }

    pub fn free_chunk(&mut self) {
        self.chunk = None;
        self.ip = 0;
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        #[cfg(feature = "debug_elapsed_time")]
        let secs_start = std::time::SystemTime
            ::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        let mut chunk = Chunk::new();

        let mut compiler = Compiler::new(source, &mut chunk);

        if compiler.compile() {
            self.free_chunk();
            return InterpretResult::CompileError;
        }

        self.init_chunk(chunk);

        let result = self.run();

        self.free_chunk();

        #[cfg(feature = "debug_elapsed_time")]
        {
            let secs_end = std::time::SystemTime
                ::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            let elapsed = (secs_end - secs_start) * 1000.0;
            println!("Elapsed time: {}ms", elapsed);
        }

        result
    }

    #[cfg(test)]
    pub fn interpret_chunk(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.ip = 0;
        self.run()
    }

    pub fn runtime_error(&mut self, message: &str) {
        self.had_runtime_error = true;

        let line: usize = self.chunk
            .as_ref()
            .unwrap()
            .get_line(self.ip - 1);
        eprintln!("[line {}]: {}", line, message);

        self.reset_stack();
    }

    fn run(&mut self) -> InterpretResult {
        #[cfg(any(test, feature = "debug_trace_execution"))]
        let mut _debug_result: Value = Value::Null;

        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("          ");
                for slot in &self.stack {
                    print!("[ {:?} ]", slot);
                }
                println!();

                if self.chunk.is_some() {
                    self.chunk.as_ref().unwrap().disassemble_instruction(self.ip);
                }
            }

            let instruction = self.read_byte().into();

            match instruction {
                OpCode::OpReturn => {
                    #[cfg(test)]
                    {
                        return InterpretResult::Debug(_debug_result);
                    }

                    #[cfg(feature = "debug_trace_execution")]
                    {
                        if !self.had_runtime_error && self.stack.len() > 0 {
                            println!("{:?}", _debug_result);
                        }
                    }

                    #[allow(unreachable_code)]
                    {
                        if self.had_runtime_error {
                            return InterpretResult::RuntimeError;
                        }
                        return InterpretResult::Ok;
                    }
                }
                OpCode::OpConstant => {
                    let constant = self.read_constant();
                    self.stack.push(constant);
                }
                OpCode::OpNull => {
                    self.stack.push(Value::Null);
                }
                OpCode::OpTrue => {
                    self.stack.push(Value::Bool(true));
                }
                OpCode::OpFalse => {
                    self.stack.push(Value::Bool(false));
                }
                OpCode::OpConstantLong => {
                    let constant = self.read_long_constant();
                    self.stack.push(constant);
                }
                OpCode::OpNegate => {
                    // Reimplement this
                    // if let Some(last) = self.stack.last_mut() {
                    //     *last *= Value::Float64(-1.0);
                    // }

                    let value = self.stack.pop().unwrap();

                    match -value {
                        Ok(result) => self.stack.push(result),
                        Err(msg) => eprintln!("{}", msg),
                    }
                }
                OpCode::OpNot => {
                    let v = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(v.is_falsey()));
                }
                OpCode::OpAdd => self.binary_op(|a, b| a + b),
                OpCode::OpSubtract => self.binary_op(|a, b| a - b),
                OpCode::OpMultiply => self.binary_op(|a, b| a * b),
                OpCode::OpDivide => self.binary_op(|a, b| a / b),
                OpCode::OpPower => self.binary_op(|a, b| a.pow(b)),

                OpCode::OpEqualEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a == b));
                }
                OpCode::OpBangEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a != b));
                }
                OpCode::OpGreater => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a > b));
                }
                OpCode::OpGreaterEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a >= b));
                }
                OpCode::OpLess => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a < b));
                }
                OpCode::OpLessEqual => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a <= b));
                }
                OpCode::OpInterpolate => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::String(format!("{}{}", a.to_string(), b.to_string())));
                }
                OpCode::OpPrint => {
                    let value = self.stack.pop().unwrap();
                    println!("{}", value.to_string());
                }
                OpCode::OpPop => {
                    #[cfg(any(test, feature = "debug_trace_execution"))]
                    {
                        _debug_result = self.stack.pop().unwrap();
                        continue;
                    }
                    #[allow(unreachable_code)]
                    {
                        self.stack.pop();
                    }
                }
                OpCode::OpDefineGlobal => {
                    let constant = self.read_constant();

                    if let Value::VariableDefinition(variable_definition) = constant {
                        let value = self.stack.pop().unwrap();

                        let value_type: ValueType;
                        if variable_definition.get_value_type() == ValueType::Dynamic {
                            value_type = value.to_value_type();
                        } else {
                            self.runtime_error("Could not determine the type of the variable");
                            continue;
                        }

                        let variable = Variable::new(
                            variable_definition.get_name(),
                            value,
                            value_type,
                            variable_definition.is_mutable()
                        );

                        self.globals.insert(variable_definition.get_name(), variable);
                    } else {
                        self.runtime_error("Expected a variable to define");
                    }
                }
                OpCode::OpGetGlobal => {
                    let constant = self.read_constant();
                    if let Value::VariableLookup(variable_name) = constant {
                        if let Some(value) = self.globals.get(&variable_name) {
                            self.stack.push(value.get_value());
                        } else {
                            self.runtime_error("Undefined variable");
                        }
                    } else {
                        self.runtime_error("Expected a variable to get");
                    }
                }
                OpCode::OpSetGlobal => {
                    let constant = self.read_constant();
                    if let Value::VariableLookup(variable_name) = constant {
                        let value = self.stack.last().unwrap().clone();
                        if let Entry::Occupied(mut e) = self.globals.entry(variable_name.clone()) {
                            let result = (*e.get_mut()).set_value(value);
                            if !result {
                                self.runtime_error(
                                    format!("Cannot reassign immutable variable: {}", variable_name).as_str()
                                );
                            }
                        } else {
                            self.runtime_error(
                                format!("Undefined variable '{}'", variable_name).as_str()
                            );
                        }
                    } else {
                        self.runtime_error("Expected a variable to set");
                    }
                }
            }
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        let ip = self.ip;
        if let Some(chunk) = &self.chunk {
            if let Some(code) = chunk.get_code().get(ip - 1) {
                *code
            } else {
                self.runtime_error("Could not get operation (OPCODE)");
                0
            }
        } else {
            0
        }
    }
    fn read_bytes(&mut self, n: usize) -> usize {
        let mut result = 0;
        for _ in 0..n {
            result = (result << 8) | (self.read_byte() as usize);
        }
        result
    }

    fn read_constant(&mut self) -> Value {
        let byte = self.read_byte().into();
        self.chunk.as_ref().unwrap().read_constant(byte)
    }

    fn read_long_constant(&mut self) -> Value {
        let bytes = self.read_bytes(3);
        self.chunk
            .as_ref()
            .unwrap()
            .read_constant(bytes as u16)
    }

    fn binary_op(&mut self, op: fn(a: Value, b: Value) -> Result<Value, String>) {
        let b = match self.stack.pop() {
            Some(b) => b,
            None => {
                self.runtime_error("Expected two operands for binary operation, but got only one.");

                return;
            }
        };
        let a = match self.stack.pop() {
            Some(a) => a,
            None => {
                self.runtime_error("Expected two operands for binary operation, but got only one.");
                return;
            }
        };

        match op(a, b) {
            Ok(result) => self.stack.push(result),
            Err(msg) => self.runtime_error(msg.as_str()),
        }
    }
}
