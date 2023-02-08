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
            "{} Table \"{}\" does not exist.",
            "ERROR!".red().bold(),
            table_name.yellow()
        );
        return;
    }

    let table = table.unwrap();
    let add_type = &command[2];

    match AddType::from_str(add_type) {
        Err(_) => println!(
            "{} Invalid add parameter \"{}\".",
            "ERROR!".red().bold(),
            add_type.yellow()
        ),
        Ok(add_type) => match add_type {
            AddType::COLUMN => {
                let col_type = ColumnType::from_str(&command[4]);

                if col_type.is_err() {
                    println!("{} Invalid column type", "ERROR!".red().bold());
                    return;
                }

                let col_name = &command[3];

                use crate::database::column::Column;
                table.columns.push(Column {
                    name: col_name.clone(),
                    items: Vec::new(),
                    item_type: col_type.unwrap(),
                });
            }
            AddType::ROW => {
                // add people row "Freddy", "Cansick"
                let raw_row_fields = &command[3..command.len()].to_vec();
                let row_fields = raw_row_fields.iter().join(" ");
                let row_fields = row_fields
                    .split(",")
                    .map(|field| field.trim().replace("\"", ""))
                    .collect_vec();

                // TODO parse types then test if they are compatible

                for (col_idx, column) in table.columns.iter_mut().enumerate() {
                    column.items.push(row_fields[col_idx].clone());
                }
            }
        },
    }
}
