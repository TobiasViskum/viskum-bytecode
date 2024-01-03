use std::{ io::Write, fs::File, io };

pub fn generate_opcode(opcodes: &[&str]) -> io::Result<()> {
    let out_dir = "src/opcodes.rs";

    let mut file = File::create(out_dir)?;

    writeln!(file, "pub enum OpCode {{")?;
    for opcode in opcodes {
        let split = opcode.split(" = ").collect::<Vec<&str>>();
        let opcode = split[0].trim();
        let code = split[1].trim();
        writeln!(file, "    {} = {},", opcode, code)?;
    }
    writeln!(file, "}}")?;

    writeln!(file, "impl From<u8> for OpCode {{")?;
    writeln!(file, "    fn from(byte: u8) -> Self {{")?;
    writeln!(file, "        match byte {{")?;
    for opcode in opcodes {
        let split = opcode.split(" = ").collect::<Vec<&str>>();
        let opcode = split[0].trim();
        let code = split[1].trim();
        writeln!(file, "            {} => OpCode::{},", code, opcode)?;
    }
    writeln!(file, "            _ => unimplemented!(\"Unknown opcode: {{}}\", byte),")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    writeln!(file, "impl From<OpCode> for u8 {{")?;
    writeln!(file, "    fn from(opcode: OpCode) -> Self {{")?;
    writeln!(file, "        opcode as u8")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    Ok(())
}
