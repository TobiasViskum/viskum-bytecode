mod generate_opcode;

use std::io;

fn main() -> io::Result<()> {
    let opcodes = [
        "OpReturn = 0",
        "OpConstant = 1",
        "OpConstantLong = 2",
        "OpNegate = 3",
        "OpAdd = 4",
        "OpSubtract = 5",
        "OpMultiply = 6",
        "OpDivide = 7",
    ];

    generate_opcode::generate_opcode(&opcodes)?;

    Ok(())
}
