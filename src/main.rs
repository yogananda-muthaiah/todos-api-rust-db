use reqwest::Error;
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::result::Result;

#[derive(Debug, Deserialize)]
struct Todo {
    #[serde(rename = "userId")] // Map JSON's `userId` to Rust's `user_id`
    user_id: u32,
    id: u32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Step 1: Fetch data from the API
    let url = "https://jsonplaceholder.typicode.com/todos";
    let todos: Vec<Todo> = reqwest::get(url).await?.json().await?;

    // Step 2: Connect to SQLite database
    let conn = Connection::open("todos.db").expect("Could not open database");

    // Step 3: Create the todos table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            userId INTEGER NOT NULL,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        )",
        [],
    )
    .expect("Failed to create table");

    // Step 4: Insert the fetched data into the todos table
    for todo in todos {
        conn.execute(
            "INSERT INTO todos (id, userId, title, completed) VALUES (?1, ?2, ?3, ?4)",
            params![todo.id, todo.user_id, todo.title, todo.completed],
        )
        .expect("Failed to insert todo");
    }

    println!("Data inserted successfully!");

    Ok(())
}
