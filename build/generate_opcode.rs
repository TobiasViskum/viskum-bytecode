use std::{ fmt::Write as OtherWrite, fs::File, io::{ self, Read, Write } };
use regex::Regex;
struct OpcodeInput {
    pub opcode: String,
    pub number: String,
    pub instruction: String,
}

impl OpcodeInput {
    fn new(str: &str) -> Self {
        let split = str.split("=").collect::<Vec<&str>>();
        let opcode = split[0].trim().to_string();

        let split_2 = split[1].split("|").collect::<Vec<&str>>();
        let number = split_2[0].trim().to_string();
        let instruction = split_2[1].trim().to_string();

        Self { opcode, number, instruction }
    }

    // Can be deleted
    fn opcode_to_string(&self) -> String {
        self.opcode
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
            .join("")
    }

    fn opcode_to_instruction_string(&self) -> String {
        self.opcode
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_string()
                } else if c.is_uppercase() {
                    format!("_{}", c)
                } else {
                    c.to_uppercase().to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }
}

fn generate_instructions(opcodes: &[&str]) -> io::Result<()> {
    let target_file = "src/chunk/debug.rs";

    let mut file = File::open(target_file)?;

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut content = String::from_utf8_lossy(&buffer).to_string();

    /*
    
    impl Chunk {
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
    
    */

    // Write this as a regex: /* AUTO-GENERATED */

    let re = Regex::new(r"(?s)/\* AUTO-GENERATED \*/.*").unwrap();

    let mut new_content = "/* AUTO-GENERATED */\n".to_string();
    new_content.write_str("impl Chunk {\n").unwrap();
    new_content
        .write_str("    pub fn disassemble_instruction(&self, offset: usize) -> usize {\n")
        .unwrap();
    new_content.write_str("        print!(\"{:04} \", \"\");\n").unwrap();
    new_content
        .write_str(
            "        if offset > 0 && self.get_line(offset) == self.get_line(offset - 1) {\n"
        )
        .unwrap();
    new_content.write_str("            print!(\"   | \");\n").unwrap();
    new_content.write_str("        } else {\n").unwrap();
    new_content.write_str("            print!(\"{:4} \", self.get_line(offset));\n").unwrap();
    new_content.write_str("        }\n").unwrap();
    new_content.write_str("        let instruction = OpCode::from(self.code[offset]);\n").unwrap();
    new_content.write_str("        match instruction {\n").unwrap();

    for opcode in opcodes {
        let opcode_input = OpcodeInput::new(opcode);

        new_content
            .write_str(
                &format!(
                    "            OpCode::{} => self.{}(\"{}\", offset),\n",
                    opcode_input.opcode,
                    opcode_input.instruction,
                    opcode_input.opcode_to_instruction_string()
                )
            )
            .unwrap();
    }

    new_content.write_str("        }\n").unwrap();
    new_content.write_str("    }\n").unwrap();
    new_content.write_str("}").unwrap();

    // Check if matches
    if re.is_match(&content) {
        content = re.replace_all(&content, new_content.as_str()).to_string();
    } else {
        content.push_str(new_content.as_str());
    }

    let mut new_file = File::create(target_file)?;

    new_file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn generate_opcode(opcodes: &[&str]) -> io::Result<()> {
    generate_instructions(opcodes)?;

    let out_file = "src/opcodes.rs";

    let mut file = File::create(out_file)?;

    writeln!(file, "#[derive(Debug)]")?;
    writeln!(file, "pub enum OpCode {{")?;
    for opcode in opcodes {
        let opcode_input = OpcodeInput::new(opcode);

        writeln!(file, "    {} = {},", opcode_input.opcode, opcode_input.number)?;
    }
    writeln!(file, "}}")?;

    writeln!(file, "impl OpCode {{")?;
    writeln!(file, "    pub fn to_string(&self) -> String {{")?;
    writeln!(file, "        match self {{")?;
    for opcode in opcodes {
        let opcode_input = OpcodeInput::new(opcode);

        writeln!(
            file,
            "            OpCode::{} => \"{}\".to_string(),",
            opcode_input.opcode,
            opcode_input.opcode_to_string()
        )?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    writeln!(file, "impl From<u8> for OpCode {{")?;
    writeln!(file, "    fn from(byte: u8) -> Self {{")?;
    writeln!(file, "        match byte {{")?;
    for opcode in opcodes {
        let opcode_input = OpcodeInput::new(opcode);

        writeln!(file, "            {} => OpCode::{},", opcode_input.number, opcode_input.opcode)?;
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
