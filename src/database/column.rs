use std::str::FromStr;
use strum::Display;

#[derive(derive_more::Constructor, Default, Debug, Clone)]
pub struct Currency {
    pub value: u32,
}

#[derive(Debug, strum::EnumString, Clone, Display)]
pub enum ColumnType {
    #[strum(ascii_case_insensitive)]
    FLOAT(f64),
    #[strum(ascii_case_insensitive)]
    INT(i64),
    #[strum(ascii_case_insensitive)]
    UINT(u64),
    #[strum(ascii_case_insensitive)]
    BOOL(bool),
    #[strum(ascii_case_insensitive)]
    STRING(String),
    #[strum(ascii_case_insensitive)]
    DATE(chrono::NaiveDate),
    #[strum(ascii_case_insensitive)]
    CURRENCY(Currency),
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: String,
    pub items: Vec<String>,
    pub item_type: ColumnType,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseColumnError;

impl FromStr for Column {
    type Err = ParseColumnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, col_type) = s.split_once(" ").ok_or(ParseColumnError)?;

        let col_type = ColumnType::from_str(col_type).map_err(|_| ParseColumnError)?;

        Ok(Column {
            name: name.to_string(),
            items: Vec::new(),
            item_type: col_type,
        })
    }
}
