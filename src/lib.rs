// lib.rs
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Iris {
    #[allow(dead_code)]
    sepal_length: f64,
    #[allow(dead_code)]
    sepal_width: f64,
    #[allow(dead_code)]
    petal_length: f64,
    #[allow(dead_code)]
    petal_width: f64,
    #[allow(dead_code)]
    class: String,
}

pub fn drop_iris_table(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE IF EXISTS iris", [])?;
    Ok(())
}

pub fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?")?;
    let rows = stmt.query_map(&[table_name], |_| Ok(()))?;
    let exists = rows.count() > 0;
    Ok(exists)
}

pub fn create_iris_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS iris (
            sepal_length REAL,
            sepal_width REAL,
            petal_length REAL,
            petal_width REAL,
            class TEXT
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_iris_data(conn: &Connection) -> Result<(), rusqlite::Error> {
    match csv::Reader::from_path("iris.csv") {
        Ok(mut reader) => {
            for result in reader.records() {
                match result {
                    Ok(record) => {
                        let sepal_length = &record[0];
                        let sepal_width = &record[1];
                        let petal_length = &record[2];
                        let petal_width = &record[3];
                        let class = &record[4];
                        conn.execute(
                            "INSERT INTO iris (sepal_length, sepal_width, petal_length, petal_width, class)
                             VALUES (?1, ?2, ?3, ?4, ?5)",
                            params![sepal_length, sepal_width, petal_length, petal_width, class],
                        )?;
                    }
                    Err(err) => {
                        eprintln!("Error reading CSV record: {:?}", err);
                    }
                }
            }
            let mut stmt = conn.prepare("SELECT * FROM iris")?;
            let rows = stmt.query_map([], |row| {
                Ok(Iris {
                    sepal_length: row.get(0)?,
                    sepal_width: row.get(1)?,
                    petal_length: row.get(2)?,
                    petal_width: row.get(3)?,
                    class: row.get(4)?,
                })
            })?;
            for row in rows {
                println!("{:?}", row?);
            }
            Ok(())
        }
        Err(err) => {
            eprintln!("Error opening CSV file: {:?}", err);
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }
}

pub fn print_first_five_rows(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM Iris LIMIT 5")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, f64>(0)?,
            row.get::<usize, f64>(1)?,
            row.get::<usize, f64>(2)?,
            row.get::<usize, f64>(3)?,
            row.get::<usize, String>(4)?,
        ))
    })?;
    for row in rows {
        println!("{:?}", row?);
    }
    Ok(())
}

pub fn insert_initial_data(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO Iris (sepal_length, sepal_width, petal_length, petal_width, class) VALUES (?, ?, ?, ?, ?)",
        params![7.5, 7.5, 7.5, 7.5, "Test"],
    )?;
    Ok(())
}

pub fn update_iris_table(conn: &Connection) -> Result<()> {
    println!("Updating rows...");
    let rows_updated = conn.execute(
        "UPDATE Iris SET sepal_length = 4.5 WHERE class = 'Test'",
        [],
    )?;
    println!("Updated {} rows", rows_updated);
    Ok(())
}

pub fn order_iris_table(conn: &Connection) -> Result<()> {
    println!("Ordering Iris table by sepal length in descending order...");
    let mut stmt = conn.prepare("SELECT * FROM Iris ORDER BY sepal_length DESC LIMIT 10")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, f64>(0)?,
            row.get::<usize, f64>(1)?,
            row.get::<usize, f64>(2)?,
            row.get::<usize, f64>(3)?,
            row.get::<usize, String>(4)?,
        ))
    })?;
    for row in rows {
        println!("{:?}", row?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Result;

    #[test]
    fn test_create_iris_table() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_iris_table(&conn)?;
        let table_names: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>>>()?;
        assert_eq!(table_names, vec!["iris"]);
        Ok(())
    }

    #[test]
    fn test_insert_iris_data() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_iris_table(&conn)?;
        insert_iris_data(&conn)?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM iris", [], |row| row.get(0))?;
        assert_eq!(count, 150);
        Ok(())
    }

    #[test]
    fn test_print_first_five_rows() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_iris_table(&conn)?;
        insert_iris_data(&conn)?;
        let result = print_first_five_rows(&conn);
        result.unwrap();
        Ok(())
    }

    #[test]
    fn test_insert_initial_data() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_iris_table(&conn)?;
        insert_initial_data(&conn)?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM iris", [], |row| row.get(0))?;
        assert_eq!(count, 1);
        Ok(())
    }

    #[test]
    fn test_update_iris_table() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_iris_table(&conn)?;
        insert_initial_data(&conn)?;
        update_iris_table(&conn)?;
        let sepal_length: f64 = conn.query_row("SELECT sepal_length FROM iris WHERE class = 'Test'", [], |row| row.get(0))?;
        assert_eq!(sepal_length, 4.5);
        Ok(())
    }

    #[test]
    fn test_order_iris_table() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_iris_table(&conn)?;
        insert_iris_data(&conn)?;
        let result = order_iris_table(&conn);
        result.unwrap();
        Ok(())
    }
}