use std::env;
use rusqlite::Connection;

fn main() {
    if let Err(err) = run_database_operation() {
        eprintln!("Error: {:?}", err);
    }
}

fn run_database_operation() -> Result<(), rusqlite::Error> {
    let conn = Connection::open("iris.db")?;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <function>", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "create" => {
            println!("Creating Iris table...");
            iris_functions::create_iris_table(&conn)?;
            println!("Iris table exists: {}", iris_functions::table_exists(&conn, "Iris")?);
            println!("Iris table created successfully.");
        }
        "insert" => {
            println!("Inserting Iris data...");
            iris_functions::insert_iris_data(&conn)?;
            println!("Iris data inserted successfully.");
        }
        "print" => {
            println!("Printing first five rows...");
            iris_functions::print_first_five_rows(&conn)?;
            println!("Printed first five rows successfully.");
        }
        "initial" => {
            println!("Inserting initial data...");
            iris_functions::insert_initial_data(&conn)?;
            println!("Initial data inserted successfully.");
        }
        "update" => {
            println!("Updating Iris table...");
            iris_functions::update_iris_table(&conn)?;
            println!("Iris table updated successfully.");
        }
        "order" => {
            println!("Ordering Iris table...");
            iris_functions::order_iris_table(&conn)?;
            println!("Iris table ordered successfully.");
        }
        "drop" => {
            println!("Dropping Iris table...");
            iris_functions::drop_iris_table(&conn)?;
            println!("Iris table exists: {}", iris_functions::table_exists(&conn, "Iris")?);
            println!("Iris table dropped successfully.");
        }
        _ => {
            eprintln!("Invalid function: {}", args[1]);
            return Ok(());
        }
    }

    Ok(())
}