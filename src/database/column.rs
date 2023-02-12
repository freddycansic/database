use std::str::FromStr;
use strum::Display;

#[derive(derive_more::Constructor, Default, Debug, Clone)]
pub struct Currency {
    pub value: u32,
}

#[derive(Debug, strum::EnumString, Clone, Display, PartialEq)]
pub enum ColumnType {
    #[strum(ascii_case_insensitive)]
    FLOAT,
    #[strum(ascii_case_insensitive)]
    INT,
    #[strum(ascii_case_insensitive)]
    UINT,
    #[strum(ascii_case_insensitive)]
    BOOL,
    #[strum(ascii_case_insensitive)]
    STRING,
    #[strum(ascii_case_insensitive)]
    DATE,
    #[strum(ascii_case_insensitive)]
    CURRENCY,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseColumnError;

impl ColumnType {
    pub fn get_string_type(s: &str) -> Result<Self, ParseColumnError> {
        if s.starts_with('.') || s.ends_with('.') {
            return Err(ParseColumnError);
        }

        if s.parse::<i64>().is_ok() {
            return Ok(ColumnType::INT);
        }

        if s.parse::<f64>().is_ok() {
            return Ok(ColumnType::FLOAT);
        }

        if s.starts_with("\"") && s.ends_with("\"") {
            return Ok(ColumnType::STRING);
        }

        Err(ParseColumnError)
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub items: Vec<String>,
    pub item_type: ColumnType,
    pub primary_key: bool,
}
