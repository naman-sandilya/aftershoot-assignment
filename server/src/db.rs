use rocket::serde::json::{json, Value};
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct ImageData {
    id: String,
    name: String,
    metadata: String,
}

pub fn create() -> Result<(), crate::errors::Error> {
    let conn = Connection::open("metadata.db")?;
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS image_data (
                id text primary key,
                name text,
                metadata BLOB
            )
        ",
        [],
    )?;

    Ok(())
}

pub fn insert(id: &str, name: &str, metadata: &str) -> Result<Value, crate::errors::Error> {
    let conn = Connection::open("metadata.db")?;
    conn.execute(
        "
        INSERT INTO image_data VALUES (?1, ?2, ?3)
        ",
        (id, name, metadata),
    )?;

    Ok(json!({
        "id": id,
        "name": name,
        "metadata": metadata,
    }))
}

pub fn select(id: &str) -> Result<Value, crate::errors::Error> {
    let conn = Connection::open("metadata.db")?;
    let mut stmt = conn.prepare(
        format!(
            "SELECT id, name, metadata FROM image_data WHERE id = \"{}\"",
            id
        )
        .as_str(),
    )?;

    let selected_row = stmt.query_row([], |row| {
        Ok(ImageData {
            id: row.get(0)?,
            name: row.get(1)?,
            metadata: row.get(2)?,
        })
    })?;

    Ok(json!({
        "id": selected_row.id,
        "name": selected_row.name,
        "metadata": selected_row.metadata,
    }))
}

pub fn select_all() -> Result<Value, crate::errors::Error> {
    let conn = Connection::open("metadata.db")?;
    let mut stmt = conn.prepare("SELECT id, name, metadata FROM image_data")?;

    let selected_rows = stmt.query_map([], |row| {
        Ok(ImageData {
            id: row.get(0)?,
            name: row.get(1)?,
            metadata: row.get(2)?,
        })
    })?;

    let mut result: Vec<Value> = Vec::new();
    for row in selected_rows {
        let img_data = row?;
        result.push(json!({
            "id": img_data.id,
            "name": img_data.name,
        }));
        println!("{:?}", img_data.id);
    }

    Ok(json!(result))
}
