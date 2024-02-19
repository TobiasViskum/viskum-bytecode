use std::{ fs::File, io::{ self, Write } };

pub fn generate_rules_store(parse_rules: &[&str]) -> io::Result<()> {
    let out_file = "src/parse_rule/rules_store.rs";

    let mut file = File::create(out_file)?;

    writeln!(file, "extern crate lazy_static;")?;
    writeln!(file, "use std::collections::HashMap;")?;
    writeln!(file, "use lazy_static::lazy_static;")?;
    writeln!(
        file,
        "use crate::{{ precedence::Precedence, token::token_type::TokenType::{{ self, * }} }};"
    )?;
    writeln!(file, "use super::ParseRule;")?;
    writeln!(file, "lazy_static! {{")?;
    writeln!(file, "    pub static ref PARSE_RULES: HashMap<TokenType,  ParseRule> = {{")?;
    writeln!(file, "        let mut map = HashMap::new();")?;
    for parse_rule in parse_rules {
        let split = parse_rule.split("=").collect::<Vec<&str>>();

        let token_name = format!("Token{}", split[0].trim());

        let second = split[1].replace("{", "").replace("}", "").trim().to_string();
        let args = second.split(",").collect::<Vec<&str>>();

        let arg1 = args[0].trim();
        let prefix = if arg1 == "None" {
            "None".to_string()
        } else {
            format!("Some(|c| c.{}())", arg1)
        };

        let arg2 = args[1].trim();
        let infix = if arg2 == "None" {
            "None".to_string()
        } else {
            format!("Some(|c| c.{}())", arg2)
        };

        let precedence = format!("Precedence::{}", args[2].trim());

        writeln!(file, "        map.insert({}, ParseRule {{", token_name)?;

        writeln!(file, "            prefix: {},", prefix)?;
        writeln!(file, "            infix: {},", infix)?;
        writeln!(file, "            precedence: {},", precedence)?;
        writeln!(file, "        }});")?;
    }
    writeln!(file, "        map")?;
    writeln!(file, "    }};")?;
    writeln!(file, "}}")?;

    Ok(())
}

/*

extern crate lazy_static;

use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::{ precedence::Precedence, token::token_type::TokenType::{ self, * } };

use super::ParseRule;

lazy_static! {
    pub static ref PARSE_RULES: HashMap<TokenType,  ParseRule<'static>> = {
        let mut map = HashMap::new();
        map.insert(TokenLeftParen, ParseRule {
            prefix: Some(|c| c.grouping()),
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenRightParen, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenLeftBrace, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenRightBrace, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenComma, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenDot, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenMinus, ParseRule {
            prefix: Some(|c| c.unary()),
            infix: Some(|c| c.binary()),
            precedence: Precedence::PrecTerm,
        });
        map.insert(TokenPlus, ParseRule {
            prefix: None,
            infix: Some(|c| c.binary()),
            precedence: Precedence::PrecTerm,
        });
        map.insert(TokenSemicolon, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenSlash, ParseRule {
            prefix: None,
            infix: Some(|c| c.binary()),
            precedence: Precedence::PrecFactor,
        });
        map.insert(TokenStar, ParseRule {
            prefix: None,
            infix: Some(|c| c.binary()),
            precedence: Precedence::PrecFactor,
        });
        map.insert(TokenBang, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenBangEqual, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenEqual, ParseRule {
            prefix: None,
            infix: None,
            precedence: Precedence::PrecNone,
        });
        map.insert(TokenNumber, ParseRule {
            prefix: Some(|c| c.number()),
            infix: None,
            precedence: Precedence::PrecNone,
        });
        


        map
    };
}


*/
