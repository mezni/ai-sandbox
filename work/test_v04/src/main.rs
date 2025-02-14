use rusqlite::{params, Connection, Result};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

fn main() -> Result<()> {
    // Open a connection to the SQLite database (creates it if it doesn't exist)
    let conn = Connection::open("my_database.db")?;

    // Create a table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        [],
    )?;

    // Insert some data into the table
    let users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            age: 30,
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            age: 25,
        },
        User {
            id: 3,
            name: "Charlie".to_string(),
            age: 35,
        },
    ];

    for user in users {
        conn.execute(
            "INSERT INTO users (id, name, age) VALUES (?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET name = ?2, age = ?3",
            params![user.id, user.name, user.age],
        )?;
    }

    // Query the data from the table
    let mut stmt = conn.prepare("SELECT id, name, age FROM users")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    // Print the results
    println!("Users in the database:");
    for user in user_iter {
        match user {
            Ok(u) => println!("{:?}", u),
            Err(e) => eprintln!("Error fetching user: {}", e),
        }
    }

    Ok(())
}