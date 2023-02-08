use colored::Colorize;
use itertools::Itertools;
use std::str::FromStr;

use crate::database::column::Column;
use crate::database::database::Database;
use crate::database::table::Table;

pub fn create(command: &Vec<String>, database: &mut Database) -> Result<String, failure::Error> {
    let table_name = &command[1];

    if database.get_table_by_name(&table_name).is_some() {
        return Err(failure::format_err!(
            "{}{}{}{}",
            "ERROR!".red().bold(),
            " Could not create table with name \"",
            table_name.yellow(),
            "\" since it already exists."
        ));
    }

    if (command.len() - 2) % 2 != 0 {
        return Err(failure::format_err!(
            "{}{}",
            "ERROR!".red().bold(),
            " Could not create table. Uneven column parameters."
        ));
    }

    use crate::database::column::ParseColumnError;
    let columns: Vec<Result<Column, ParseColumnError>> = command
        .iter()
        .skip(2)
        .chunks(2)
        .into_iter()
        .map(|mut pair| pair.join(" "))
        .map(|pair| Column::from_str(&pair))
        .collect_vec();

    for col in columns.iter() {
        if col.is_err() {
            return Err(failure::format_err!(
                "{}{}",
                "ERROR!".red().bold(),
                " Could not create table. Invalid column type."
            ));
        }
    }

    let columns = columns.into_iter().map(|col| col.unwrap()).collect_vec();

    let new_table = Table {
        name: table_name.clone(),
        length: 0,
        columns: columns,
    };

    database.tables.push(new_table);

    Ok(format!(
        "{}{}{}{}",
        "OK!".green().bold(),
        " Table \"",
        table_name.green(),
        "\" created!"
    ))
}
