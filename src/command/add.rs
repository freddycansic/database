use colored::Colorize;
use itertools::Itertools;
use std::str::FromStr;

use crate::{database::column::ColumnType, Database};

#[derive(Debug, strum::EnumString, Clone)]
enum AddType {
    #[strum(ascii_case_insensitive)]
    ROW,
    #[strum(ascii_case_insensitive)]
    COLUMN,
}

pub fn add(command: &Vec<String>, database: &mut Database) {
    let table_name = &command[1];
    let table = database.get_table_by_name(table_name);

    if table.is_none() {
        println!(
            "{}{}{}{}",
            "ERROR!".red().bold(),
            " Table \"",
            table_name.yellow(),
            "\" does not exist."
        );
        return;
    }

    let table = table.unwrap();
    let add_type = &command[2];

    match AddType::from_str(add_type) {
        Err(_) => println!(
            "{}{}{}{}",
            "ERROR!".red().bold(),
            " Invalid add parameter \"",
            add_type.yellow(),
            "\"."
        ),
        Ok(add_type) => match add_type {
            AddType::COLUMN => {
                let col_vec = ColumnType::from_str(&command[4]);

                if col_vec.is_err() {
                    println!("{}{}", "ERROR!".red().bold(), " Invalid column type");
                    return;
                }

                use crate::database::column::Column;
                table.columns.push(Column {
                    name: command[3].clone(),
                    items: col_vec.unwrap().to_vec(),
                });
            }
            AddType::ROW => {
                // add people row "Freddy", "Cansick"
                let raw_row_fields = &command[3..command.len()].to_vec();
                let row_fields = raw_row_fields.iter().join(" ");
                let row_fields = row_fields.split(",").map(|field| field.trim().replace("\"", "")).collect_vec();

                println!("{:?}", row_fields);

                for (col_idx, column) in table.columns.iter_mut().enumerate() {
                    column.items.push(ColumnType::STRING(row_fields[col_idx].clone()));
                }
            }
        },
    }
}