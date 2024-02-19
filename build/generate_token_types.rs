use std::{ fs::File, io, io::Write };

pub fn generate_token_types(tokentypes: &[&str]) -> io::Result<()> {
    let out_dir = "src/token/token_type.rs";

    let mut file = File::create(out_dir)?;

    writeln!(file, "#[derive(PartialEq, Debug, Clone, Eq, Hash)]")?;

    writeln!(file, "pub enum TokenType {{")?;
    for (_, tokentype) in tokentypes.iter().enumerate() {
        let tokentype = tokentype.split("<=>").collect::<Vec<&str>>()[0].trim();

        writeln!(file, "    {},", tokentype)?;
    }
    writeln!(file, "}}")?;

    writeln!(file, "impl TokenType {{")?;

    writeln!(file, "    pub fn as_str(&self) -> &str {{")?;
    writeln!(file, "        match self {{")?;
    for (_, tokentype) in tokentypes.iter().enumerate() {
        let split_token = tokentype.split("<=>").collect::<Vec<&str>>();
        let tokentype = split_token[0].trim();

        let display_text = split_token[1].trim();

        writeln!(file, "            TokenType::{} => \"{}\",", tokentype, display_text)?;
    }
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;

    Ok(())
}
