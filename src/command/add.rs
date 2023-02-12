use colored::Colorize;
use itertools::Itertools;
use std::str::FromStr;

use crate::{
    database::{column::ColumnType, table::Table},
    Database,
};

#[derive(Debug, strum::EnumString, Clone)]
enum AddType {
    #[strum(ascii_case_insensitive)]
    ROW,
    #[strum(ascii_case_insensitive)]
    COLUMN,
}

pub fn add(command: &Vec<String>, database: &mut Database) -> Result<String, failure::Error> {
    let table_name = &command[1];
    let table = database.get_table_by_name(table_name)?;

    let add_type = &command[2];
    let add_type = AddType::from_str(add_type).map_err(|_| {
        failure::format_err!(
            "{} Invalid add parameter \"{}\". Should be either {} or {}",
            "ERROR!".red().bold(),
            add_type.yellow(),
            "column".yellow(),
            "row".yellow()
        )
    })?;

    match add_type {
        AddType::COLUMN => add_column(command, table),
        AddType::ROW => add_row(command, table),
    }
}

fn add_column(command: &Vec<String>, table: &mut Table) -> Result<String, failure::Error> {
    if command.len() != 5 {
        return Err(failure::format_err!("{} Incorrect number of args.", "ERROR!".red().bold()))
    }
    
    let col_type = ColumnType::from_str(&command[4])
        .map_err(|_| failure::format_err!("{} Invalid column type \"{}\".", "ERROR!".red().bold(), command[4]))?;

    let col_name = &command[3];

    use crate::database::column::Column;
    table.columns.push(Column {
        name: col_name.clone(),
        items: vec!["".to_owned(); table.get_num_rows()],
        item_type: col_type,
        primary_key: false, // TODO
    });

    Ok(format!("{} Column added.", "OK!".bold().green()))
}

fn add_row(command: &Vec<String>, table: &mut Table) -> Result<String, failure::Error> {
    let raw_row_fields = &command[3..command.len()].to_vec();
    let row_fields = raw_row_fields.iter().join(" ");
    let row_fields = row_fields
        .split(",")
        .map(|field| field.trim())
        .collect_vec();

    for (col_idx, column) in table.columns.iter().enumerate() {
        let row_field = &row_fields[col_idx];

        let row_field_type = ColumnType::get_string_type(&row_field).map_err(|_| {
            failure::format_err!(
                "{} Cannot parse input field \"{}\"!",
                "ERROR!".red().bold(),
                row_field.yellow()
            )
        })?;

        if row_field_type != column.item_type {
            return Err(failure::format_err!(
                "{} Row field \"{}\" is of the wrong type for column \"{}\" of type {}!",
                "ERROR!".red().bold(),
                row_field,
                column.name,
                column.item_type
            ));
        }
    }

    for (col_idx, column) in table.columns.iter_mut().enumerate() {
        let row_field = &row_fields[col_idx];

        // if its a string then trim the surrounding quote marks
        if ColumnType::get_string_type(row_field).unwrap() == ColumnType::STRING {
            column
                .items
                .push(row_field[1..row_field.len() - 1].to_string())
        } else {
            column.items.push(row_field.to_string());
        }
    }

    Ok(format!("{} Row added.", "OK!".bold().green()))
}
