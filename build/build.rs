mod generate_opcode;
mod generate_token_types;
mod build_info;

use build_info::{ OPCODES, TOKENTYPES };

use std::io;

fn main() -> io::Result<()> {
    generate_opcode::generate_opcode(&OPCODES)?;
    generate_token_types::generate_token_types(&TOKENTYPES)?;

    Ok(())
}
