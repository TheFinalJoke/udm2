// use std::io::Error;

// use crate::rpc_types;
// use crate::udm_traits;
// use itertools::Itertools;
use log;
// use rusqlite::Connection;
use sql_query_builder;
// use std::path::Path;
// Open or create file

// --Create --
// Create file
// Create Database

// Open
// Make sure db exists
// if no db Exists
// Create DB

// Since we are going to not relying on the db very often
// Times we need to DB
// modify, creation, and reading on first time(will cache DB in memory)

// The front end will write, and utilize write back cache for optimzation

// Maybe implement some callbacks??
// On executing changes to DB
// https://docs.rs/rusqlite/latest/rusqlite/hooks/index.html

// Implement Config log

// Implement Cached Statements for all recipes and stuff

// struct SqliteOperations {
//     connection: Option<Connection>,
//     data: Option<String>,
//     bulk_data: Option<Vec<String>>,
// }

pub(crate) fn insert_transaction_sql_generator(
    table_name: &str,
    columns: &str,
    values: &str,
) -> String {
    log::trace!(
        "Entered Insert Transaction Query Builder on {:?}",
        table_name
    );
    let insert_statement = sql_query_builder::Insert::new()
        .insert_into(
            format!(
                "({table_name} {columns})",
                table_name = table_name,
                columns = columns
            )
            .as_str(),
        )
        .values(values);
    log::debug!(
        "Inserting to {} with {}",
        table_name,
        &insert_statement.clone().debug()
    );
    insert_statement.as_string()
}

pub(crate) fn update_transaction_sql_generator(
    table: &str,
    set_values: &str,
    where_clause: &str,
) -> String {
    log::trace!(
        "Entered Update Transaction Query Builder on {table} with set values {:?} and where {:?}",
        set_values,
        where_clause,
    );
    let update_statement = sql_query_builder::Update::new()
        .update(table)
        .where_clause(format!("{}", where_clause).as_str())
        .set(set_values);

    log::debug!(
        "updating to {} with {}",
        &table,
        &update_statement.clone().debug()
    );
    update_statement.as_string()
}

pub(crate) fn delete_transaction_sql_generator(table: &str, where_clause: &str) -> String {
    log::trace!(
        "Entered Delete Transaction Query Builder on {:?} with where_clause {:?}",
        table,
        where_clause
    );
    let delete_statement = sql_query_builder::Delete::new()
        .delete_from(table)
        .where_clause(where_clause);
    log::debug!(
        "Inserting to {} with {}",
        &table,
        &delete_statement.clone().debug()
    );
    delete_statement.as_string()
}
