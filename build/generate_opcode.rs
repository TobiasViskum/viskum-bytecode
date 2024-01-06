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

    writeln!(file, "impl OpCode {{")?;
    writeln!(file, "    pub fn to_string(&self) -> String {{")?;
    writeln!(file, "        match self {{")?;
    for opcode in opcodes {
        let split = opcode.split(" = ").collect::<Vec<&str>>();
        let opcode = split[0].trim();

        //Replaces any uppercase letters with a space and the corresponding lowercase letter.
        let display_text = opcode
            .replace("Op", "")
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_lowercase().to_string()
                } else if c.is_uppercase() {
                    format!(" {}", c.to_lowercase())
                } else {
                    c.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("");

        writeln!(file, "            OpCode::{} => \"{}\".to_string(),", opcode, display_text)?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
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
