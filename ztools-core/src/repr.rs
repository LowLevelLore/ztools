use std::str::FromStr;

use crate::ZToolsError;

#[derive(Debug, Clone)]
pub enum Representations {
    Decimal,
    Binary,
    Octal,
    HexaDecimal,
}

impl FromStr for Representations {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "d" | "decimal" => Ok(Representations::Decimal),
            "b" | "binary" => Ok(Representations::Binary),
            "o" | "octal" => Ok(Representations::Octal),
            "x" | "hex" | "hexadecimal" => Ok(Representations::HexaDecimal),
            _ => Err("Unknown representation".to_string()),
        }
    }
}

pub fn convert_repr(value: &str, target: Representations) -> Result<String, ZToolsError> {
    let parsed_value = if let Some(stripped) = value.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16)
    } else if let Some(stripped) = value.strip_prefix("0b") {
        u64::from_str_radix(stripped, 2)
    } else if let Some(stripped) = value.strip_prefix("0o") {
        u64::from_str_radix(stripped, 8)
    } else {
        value.parse::<u64>()
    };

    match parsed_value {
        Ok(number) => {
            let result = match target {
                Representations::Decimal => number.to_string(),
                Representations::Binary => format!("0b{:b}", number),
                Representations::Octal => format!("0o{:o}", number),
                Representations::HexaDecimal => format!("0x{:x}", number),
            };
            Ok(result)
        }
        Err(_) => Err(ZToolsError::InvalidInput(
            "Invalid representation number".to_owned(),
        )),
    }
}
