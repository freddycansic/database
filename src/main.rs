use std::{str::FromStr, io::Write};
use itertools::{Tuples, Itertools};
use colored::Colorize;

fn any_of<T, P>(iter: impl Iterator<Item = T>, predicate: P) -> bool
where
    P: Fn(&T) -> bool,
{
    let mut ans = false;
    iter.for_each(|item| {
        if predicate(&item) {
            ans = true
        }
    });

    ans // why?
}

#[derive(derive_more::Constructor, Default, Debug)]
struct Currency {
    value: u32,
}

#[derive(Debug)]
enum ColumnType {
    Float(f32),
    Int(u32),
    Long(u64),
    Bool(bool),
    String(String),
    Date(chrono::NaiveDate),
    Currency(Currency),
}

struct Column {
    items: Vec<ColumnType>,
    name: String,
}

#[derive(derive_more::Constructor)]
struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Table {
    fn get_column_by_name(&self, name: &String) -> Option<&Column> {
        self.columns.iter().find(|column| column.name == *name)
    }
}

#[derive(derive_more::Constructor, Default)]
struct Database {
    tables: Vec<Table>,
}

impl Database {
    fn get_table_by_name(&self, name: &String) -> Option<&Table> {
        self.tables.iter().find(|table| table.name == *name)
    }
}

#[derive(strum::EnumString, Debug, PartialEq)]
enum Command {
    #[strum(ascii_case_insensitive)]
    CREATE,
    #[strum(ascii_case_insensitive)]
    SELECT,
    #[strum(ascii_case_insensitive)]
    SHOW,
}

impl Command {
    fn execute(&self, command: &Vec<String>, database: &mut Database) {
        match *self {
            Command::CREATE => create(&command, database),
            Command::SELECT => select(&command, database),
            Command::SHOW => show(&command, database),
        }
    }
}

fn create(command: &Vec<String>, database: &mut Database) {
    let table_name = &command[1];

    match database.get_table_by_name(&table_name) {
        Some(_) => {
            println!("{}{}{}{}", "ERROR!".red(), " Could not create table with name \"", table_name.yellow(), "\" since it already exists.");
            return;
        }
        None => {
            create_table(&command, database);
        }
    }
}


fn create_table(command: &Vec<String>, database: &mut Database) {
    let table_name = &command[1];

    let columns : Vec<(String, String)> = command.iter().skip(2).tuples().map(|(x, y)| (x.to_owned(), y.to_owned())).collect();

    database.tables.push(Table {
        name: table_name.clone(),
        columns: Vec::new(),
    });
    
    println!("{}", "Table created!".green())
}

fn select(command: &Vec<String>, database: &mut Database) {}

fn show(command: &Vec<String>, database: &mut Database) {}

fn spawn_cli(database: &mut Database) {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut line = String::new();

        match std::io::stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(_) => { 
                println!("{}{}", "ERROR!".red(), " Could not read command.");
                continue;
            }
        }

        let tokens: Vec<String> = line
            .split(" ")
            .map(|token| token.replace("\n", ""))
            .collect();

        match Command::from_str(tokens[0].as_str()) {
            Err(_) => println!("{}{}{}{}", "ERROR!".red(), " Command \"", tokens[0].yellow(), "\" not found!"),
            Ok(command) => command.execute(&tokens, database),
        }
    }
}

fn main() {
    // read tables from file into database

    let mut d = Database::default();

    spawn_cli(&mut d);

    // let mut d = Database::default();
    // d.tables.push(Table{
    //     name : "Hello".to_string(),
    //     columns : vec![Column{name : "h".to_string(), items : vec![ColumnType::Int(1), ColumnType::Int(2), ColumnType::Int(3)]}]
    // });

    // d.get_table_by_name(&"Hello".to_string()).unwrap().get_column_by_name(&"h".to_string()).unwrap().items.iter().for_each(|x| println!("{:?}", x));
}
