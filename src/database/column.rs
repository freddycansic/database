use std::{str::FromStr, vec};
use strum::Display;

#[derive(derive_more::Constructor, Default, Debug, Clone)]
pub struct Currency {
    pub value: u32,
}

#[derive(Debug, strum::EnumString, Clone, Display)]
pub enum ColumnType {
    #[strum(ascii_case_insensitive)]
    FLOAT(f32),
    #[strum(ascii_case_insensitive)]
    INT(i32),
    #[strum(ascii_case_insensitive)]
    LONG(i64),
    #[strum(ascii_case_insensitive)]
    UINT(u32),
    #[strum(ascii_case_insensitive)]
    ULONG(u64),
    #[strum(ascii_case_insensitive)]
    BOOL(bool),
    #[strum(ascii_case_insensitive)]
    STRING(String),
    #[strum(ascii_case_insensitive)]
    DATE(chrono::NaiveDate),
    #[strum(ascii_case_insensitive)]
    CURRENCY(Currency),
}

impl ColumnType {
    pub fn to_vec(self) -> Vec<ColumnType> {
        match self {
            ColumnType::FLOAT(value) => vec![ColumnType::FLOAT(value); 0],
            ColumnType::INT(value) => vec![ColumnType::INT(value); 0],
            ColumnType::LONG(value) => vec![ColumnType::LONG(value); 0],
            ColumnType::UINT(value) => vec![ColumnType::UINT(value); 0],
            ColumnType::ULONG(value) => vec![ColumnType::ULONG(value); 0],
            ColumnType::BOOL(value) => vec![ColumnType::BOOL(value); 0],
            ColumnType::STRING(value) => vec![ColumnType::STRING(value.clone()); 0],
            ColumnType::DATE(value) => vec![ColumnType::DATE(value); 0],
            ColumnType::CURRENCY(value) => vec![ColumnType::CURRENCY(value.clone()); 0],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub items: Vec<ColumnType>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseColumnError;

impl FromStr for Column {
    type Err = ParseColumnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, col_type) = s.split_once(" ").ok_or(ParseColumnError)?;

        let items = ColumnType::from_str(col_type)
            .map_err(|_| ParseColumnError)?
            .to_vec();

        Ok(Column {
            name: name.to_string(),
            items: items,
        })
    }
}
