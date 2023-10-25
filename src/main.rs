use rusqlite::{params, Connection};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    // Establishing a connection to the database
    let conn = Connection::open("iris.db")?;

    // Reading SQL queries from queries.sql
    let queries = std::fs::read_to_string("src/queries.sql")?;

    // Creating a markdown file for appending
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("queries_results.md")?;

    // Executing each query and writing the result to the markdown file
    for query in queries.split(';') {
        if !query.trim().is_empty() {
            // Executing the query
            let mut stmt = conn.prepare(query)?;
            // Writing the query to the markdown file
            writeln!(file, "Query: {}", query.trim())?;

            // Executing the query and getting the result
            let mut rows = stmt.query(params![])?;

            // Write the result to the markdown file
            while let Some(row) = rows.next()? {
                let mut row_values = Vec::new();
                for i in 0..row.column_count() {
                    let value: Option<String> = row.get(i)?;
                    if let Some(value) = value {
                        row_values.push(value);
                    }
                }
                writeln!(file, "{}", row_values.join(", "))?;
            }
        }
    }

    // Close the file
    drop(file);

    Ok(())
}