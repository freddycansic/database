use std::str::FromStr;

use colored::Colorize;

use crate::{database::table::Table, Database};

#[derive(Debug, strum::EnumString, Clone)]
enum DeleteType {
    #[strum(ascii_case_insensitive)]
    ROW,
    #[strum(ascii_case_insensitive)]
    COLUMN,
}

pub fn delete(command: &Vec<String>, database: &mut Database) -> Result<String, failure::Error> {
    let table_name = &command[1];
    let table = database.get_table_by_name(table_name)?;

    if command.len() <= 2 {
        database.tables.retain(|table| table.name != *table_name);

        return Ok(format!(
            "{} \"{}\" deleted.",
            "OK!".green().bold(),
            table_name.yellow()
        ));
    }

    let delete_type = DeleteType::from_str(&command[2]).map_err(|_| {
        failure::format_err!(
            "{} Deletion type must be {} or {}.",
            "ERROR!".red().bold(),
            "row".yellow(),
            "column".yellow()
        )
    })?;

    return match delete_type {
        DeleteType::COLUMN => delete_column(command, table),
        DeleteType::ROW => delete_row(command, table),
    };
}

fn delete_row(command: &Vec<String>, table: &mut Table) -> Result<String, failure::Error> {
    Ok("sfd".to_owned())
}

fn delete_column(command: &Vec<String>, table: &mut Table) -> Result<String, failure::Error> {
    Ok("sfd".to_owned())
}
