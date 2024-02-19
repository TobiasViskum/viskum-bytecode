mod generate_opcode;
mod generate_token_types;
mod build_info;
mod generate_rules_store;

use build_info::{ OPCODES, TOKENTYPES, PARSE_RULES };

use std::io;

fn main() -> io::Result<()> {
    generate_opcode::generate_opcode(&OPCODES)?;
    generate_token_types::generate_token_types(&TOKENTYPES)?;
    generate_rules_store::generate_rules_store(&PARSE_RULES)?;

    Ok(())
}
