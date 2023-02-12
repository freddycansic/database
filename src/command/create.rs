use colored::Colorize;
use itertools::Itertools;
use std::str::FromStr;

use crate::database::column::{Column, ColumnType, ParseColumnError};
use crate::database::database::Database;
use crate::database::table::Table;

pub fn create(command: &Vec<String>, database: &mut Database) -> Result<String, failure::Error> {
    let table_name = &command[1];

    let table = database.get_table_by_name(&table_name).map_err(|_| {
        failure::format_err!(
            "{}{}{}{}",
            "ERROR!".red().bold(),
            " Could not create table with name \"",
            table_name.yellow(),
            "\" since it already exists."
        )
    });

    match table {
        Ok(value) => return Err(failure::format_err!("{}", value)),
        Err(_) => (),
    }

    let mut columns: Vec<Column> = Vec::new();

    let num_tokens = command.len();
    for i in (2..num_tokens).step_by(2) {
        let col_name = &command[i];
        let col_type = ColumnType::from_str(&command[i + 1])?;
        let mut primary_key = false;

        println!("{col_name} {col_type}");

        if i+2 < num_tokens {
            primary_key = if command[i+2] == "pK" { true } else { false };
        }

        columns.push(Column {
            name: col_name.clone(),
            items: Vec::new(),
            item_type: col_type,
            primary_key: primary_key,
        })
    }

    database.tables.push(Table {
        name: table_name.clone(),
        columns: columns,
    });

    Ok(format!(
        "{}{}{}{}",
        "OK!".green().bold(),
        " Table \"",
        table_name.green(),
        "\" created!"
    ))
}
