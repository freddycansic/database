use crate::command::{
    add::add, command::Command, create::create, remove::remove, select::select, show::show,
};
use crate::database::table::Table;

#[derive(derive_more::Constructor, Default)]
pub struct Database {
    pub tables: Vec<Table>,
}

impl Database {
    pub fn get_table_by_name(&mut self, name: &String) -> Option<&mut Table> {
        self.tables.iter_mut().find(|table| table.name == *name)
    }

    pub fn execute_command(&mut self, command: Command, command_str: &Vec<String>) {
        let result = match command {
            Command::CREATE => create(command_str, self),
            Command::SELECT => select(command_str, self),
            Command::SHOW => show(command_str, self),
            Command::ADD => add(command_str, self),
            Command::REMOVE => remove(command_str, self),
        };

        match result {
            Ok(ok) => println!("{ok}"),
            Err(err) => println!("{err}"),
        }
    }
}
