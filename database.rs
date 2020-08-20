use rusqlite::{Connection,Result};
use rusqlite::NO_PARAMS;

fn main() -> Result<()> {
    let conn =  Connection::open("reddit.db")?;
    conn.execute(
        "Create table if not exists reddit (
            id integer primary key,
            name text not null unique
        ) ",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not esists cats (
            id integer primary key,
            name text not null unique,
            color_id integer not null refrences reddit(id) 
        )",
    )?;
    Ok(())
}