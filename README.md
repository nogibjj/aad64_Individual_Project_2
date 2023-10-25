# aad64_Individual_Project_2
Rust CLI Binary with SQLite

use csv::ReaderBuilder;
use rusqlite::Error as RusqliteError;
use rusqlite::{Connection, Result, Row};
use std::error::Error;

fn create_table(conn: &Connection) -> Result<()> {
    let sql = "CREATE TABLE IF NOT EXISTS Iris (
                                sepal_length REAL,
                                sepal_width REAL,
                                petal_length REAL,
                                petal_width REAL,
                                class TEXT
                            )";
    conn.execute(sql, [])?;
    Ok(())
}

fn insert_data(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("../iris.csv")?;
    while let Some(result) = reader.records().next() {
        let record = result?;
        let sepal_length = &record[0];
        let sepal_width = &record[1];
        let petal_length = &record[2];
        let petal_width = &record[3];
        let class = &record[4]; 
        
        if let Err(err) = conn.execute(
            "INSERT INTO iris_info (sepal_length, sepal_width, petal_length, petal_width, species) values (?1, ?2, ?3, ?4, ?5)",
            &[&sepal_length, &sepal_width, &petal_length, &petal_width, &class],
        ) {
            eprintln!("Error inserting row: {}", err);
        }

    }

    Ok(())
}

pub fn see_five_query(statement: &str, conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM iris_info LIMIT 5")?;

    let mut result_set = stmt.query([])?;

    let mut counter = 0; 

    while let Some(row) = result_set.next()? {
        println!("{:?}", row);
        counter += 1;

        if counter == 5 {
            break; 
        }
    }

    Ok(())
}

pub fn insert_query(conn: &Connection) -> Result<(), RusqliteError> {
    conn.execute("INSERT INTO Iris (sepal_length, sepal_width, petal_length, petal_width, class) VALUES (?, ?, ?, ?, ?)", &[7.5, 7.5, 7.5, 7.5, "Test"])?;
    Ok(())
}

pub fn update_query(conn: &Connection) -> Result<(), RusqliteError> {
    conn.execute("INSERT INTO Iris (sepal_length, sepal_width, petal_length, petal_width, class) VALUES (?, ?, ?, ?, ?)", &[7.5, 7.5, 7.5, 7.5, "Test"])?;
    conn.execute("UPDATE Iris SET sepal_length = ? WHERE sepal_width = ?", [7.5, 7.5])?;
    Ok(())
}

pub fn order_query(conn: &Connection) -> Result<(), RusqliteError> {
    let mut stmt = conn.prepare("SELECT * FROM Iris ORDER BY sepal_length DESC")?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        println!("{:?}", row);
    }

    Ok(())
}