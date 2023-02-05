use colored::Colorize;

use crate::Database;

pub fn show(command: &Vec<String>, database: &mut Database) {
    if command.len() > 2 {
        println!(
            "{}{}{}{}",
            "ERROR!".red().bold(),
            " Invalid argument length (",
            (command.len() - 1).to_string().yellow(),
            ")"
        );
        return;
    }

    let table_name = &command[1];

    match database.get_table_by_name(table_name) {
        Some(table) => {
            println!("{}", table);
        }
        None => println!(
            "{}{}{}{}",
            "ERROR!".red().bold(),
            " Table \"",
            table_name.yellow(),
            "\" does not exist."
        ),
    }
}
