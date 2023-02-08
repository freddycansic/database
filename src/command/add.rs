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
    let table = database.get_table_by_name(table_name);

    if table.is_none() {
        return Err(failure::format_err!(
            "{} Table \"{}\" does not exist.",
            "ERROR!".red().bold(),
            table_name.yellow()
        ));
    }

    let table = table.unwrap();
    let add_type = &command[2];

    match AddType::from_str(add_type) {
        Err(_) => {
            return Err(failure::format_err!(
                "{} Invalid add parameter \"{}\".",
                "ERROR!".red().bold(),
                add_type.yellow()
            ))
        }
        Ok(add_type) => match add_type {
            AddType::COLUMN => add_column(command, table),
            AddType::ROW => add_row(command, table),
        },
    }
}

fn add_column(command: &Vec<String>, table: &mut Table) -> Result<String, failure::Error> {
    let col_type = ColumnType::from_str(&command[4]);

    if col_type.is_err() {
        return Err(failure::format_err!(
            "{} Invalid column type",
            "ERROR!".red().bold()
        ));
    }

    let col_name = &command[3];

    use crate::database::column::Column;
    table.columns.push(Column {
        name: col_name.clone(),
        items: vec!["".to_owned(); table.length],
        item_type: col_type.unwrap(),
    });

    return Ok(format!(""));
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
        let row_field_type = ColumnType::get_string_type(&row_field);

        if row_field_type.is_err() {
            return Err(failure::format_err!(
                "{} Cannot parse input field \"{}\"!",
                "ERROR!".red().bold(),
                row_field
            ));
        }

        let row_field_type = row_field_type.unwrap();

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

    println!("{:?}", row_fields);

    for (col_idx, column) in table.columns.iter_mut().enumerate() {
        let row_field = &row_fields[col_idx];

        if ColumnType::get_string_type(row_field).unwrap() == ColumnType::STRING {
            column.items.push(
                // replace first and last occurences of quote marks
                row_field
                    .replacen('\"', "", 1)
                    .chars()
                    .rev()
                    .join("")
                    .replacen('\"', "", 1)
                    .chars()
                    .rev()
                    .join(""),
            )
        } else {
            column.items.push(row_field.to_string());
        }
    }

    table.length += 1;

    Ok(format!(""))
}
