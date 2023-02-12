use colored::Colorize;

use crate::command::{
    add::add, command::Command, create::create, delete::delete, select::select, show::show,
};
use crate::database::table::Table;

#[derive(derive_more::Constructor, Default)]
pub struct Database {
    pub tables: Vec<Table>,
}

impl Database {
    pub fn get_table_by_name(&mut self, name: &String) -> Result<&mut Table, failure::Error> {
        self.tables
            .iter_mut()
            .find(|table| table.name == *name)
            .ok_or(failure::format_err!(
                "{} Could not find table \"{}\".",
                "ERROR!".red().bold(),
                name.yellow()
            ))
    }

    pub fn execute_command(&mut self, command: Command, command_str: &Vec<String>) {
        let result = match command {
            Command::CREATE => create(command_str, self),
            Command::SELECT => select(command_str, self),
            Command::SHOW => show(command_str, self),
            Command::ADD => add(command_str, self),
            Command::DELETE => delete(command_str, self),
        };

        match result {
            Ok(ok) => println!("{ok}"),
            Err(err) => println!("{err}"),
        }
    }
}
