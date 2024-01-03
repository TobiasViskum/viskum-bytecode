mod generate_opcode;

use std::io;

fn main() -> io::Result<()> {
    let opcodes = ["OpReturn = 0", "OpConstant = 1", "OpConstantLong = 2", "OpNegate = 3"];

    generate_opcode::generate_opcode(&opcodes)?;

    Ok(())
}
