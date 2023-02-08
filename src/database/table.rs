use std::fmt::Display;

use crate::database::column::Column;

#[derive(derive_more::Constructor, Debug)]
pub struct Table {
    pub name: String,
    pub length: usize,
    pub columns: Vec<Column>,
}

impl Table {
    pub fn get_column_by_name(&self, name: &String) -> Option<&Column> {
        self.columns.iter().find(|column| column.name == *name)
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table_str = prettytable::Table::new();

        // add column name headings
        let mut row: Vec<prettytable::Cell> = Vec::with_capacity(self.columns.len());
        for col_idx in 0..self.columns.len() {
            let col = &self.columns[col_idx];
            row.push(prettytable::Cell::new(
                &(col.name.clone() + " : " + &col.item_type.to_string()),
            ));
        }
        table_str.add_row(prettytable::Row::new(row));

        // add row data
        for row_idx in 0..self.columns[0].items.len() {
            let mut row: Vec<prettytable::Cell> = Vec::with_capacity(self.columns.len());

            for col_idx in 0..self.columns.len() {
                row.push(prettytable::Cell::new(
                    &self.columns[col_idx].items[row_idx].to_string(),
                ));
            }

            table_str.add_row(prettytable::Row::new(row));
        }

        // let table_str = table_str.to_string().chars().next_back().unwrap().to_string();

        write!(f, "{table_str}")
    }
}
