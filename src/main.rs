use colored::Colorize;
use std::{io::Write, str::FromStr};

mod command;
mod database;

use command::command::Command;
use database::database::Database;

fn spawn_cli(database: &mut Database) {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();

        if std::io::stdin().read_line(&mut line).is_err() {
            println!("{} Could not read command.", "ERROR!".red().bold());
            continue;
        }

        let tokens: Vec<String> = line
            .split(' ')
            .map(|token| token.replace('\n', ""))
            .collect();

        let command = Command::from_str(tokens[0].as_str());

        match command {
            Err(_) => println!(
                "{} Command \"{}\" not found!",
                "ERROR!".red().bold(),
                tokens[0].yellow()
            ),
            Ok(command) => database.execute_command(command, &tokens),
        }
    }
}

fn main() {
    // TODO read tables from file into database
    use crate::database::column::ColumnType;
    println!("{}", ColumnType::get_string_type("\"hello\"").unwrap());
    println!("{}", ColumnType::get_string_type("1.0").unwrap());
    println!("{}", ColumnType::get_string_type("10").unwrap());
    // println!("{}", ColumnType::get_string_type("\"\"z").unwrap());

    let mut d = Database::default();

    spawn_cli(&mut d);
}
