use colored::Colorize;

use crate::Database;

pub fn show(command: &Vec<String>, database: &mut Database) -> Result<String, failure::Error> {
    let table_name = &command[1];
    let table_str = database.get_table_by_name(table_name)?;
    Ok(table_str.to_string())
}
