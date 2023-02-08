use colored::Colorize;

use crate::Database;

pub fn show(command: &Vec<String>, database: &mut Database) -> Result<String, failure::Error> {
    if command.len() > 2 {
        return Err(failure::format_err!(
            "{}{}{}{}",
            "ERROR!".red().bold(),
            " Invalid argument length (",
            (command.len() - 1).to_string().yellow(),
            ")"
        ));
    }

    let table_name = &command[1];

    match database.get_table_by_name(table_name) {
        Some(table) => Ok(table.to_string()),
        None => Err(failure::format_err!(
            "{}{}{}{}",
            "ERROR!".red().bold(),
            " Table \"",
            table_name.yellow(),
            "\" does not exist."
        )),
    }
}
