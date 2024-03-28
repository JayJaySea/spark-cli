use dotenvy::dotenv;
use xdg;
use std::env;
use rusqlite::{Connection, Result};

pub fn setup_database() -> Connection {
    let mut conn = setup_conn();
    enable_fk(&mut conn);
    create_tables(&mut conn);

    conn
}

fn setup_conn() -> Connection {
    dotenv().ok();


    let database_url;
    if cfg!(debug_assertions) {
        dotenv().ok();

        database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
    }
    else {
        let xdg_dirs = xdg::BaseDirectories::new()
            .expect("Cannot open base xdg directory");

        let path = xdg_dirs.create_data_directory("spark")
            .expect("Cannot create data directory");

        database_url = path.join("spark.db")
            .into_os_string()
            .into_string()
            .expect("Invalid utf8 in database path");
    }

    let conn = Connection::open(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    conn
}

fn enable_fk(conn: &Connection) {
    conn.execute("PRAGMA foreign_keys = ON", ())
        .expect("Cannot enable foreign keys");
}

fn create_tables(conn: &Connection) {
    let msg = "Cannot create tables!";
    conn.execute("CREATE TABLE IF NOT EXISTS notes (
        id TEXT PRIMARY KEY,
        title text not null unique,
        contents text not null
    )", ()).expect(&msg);

    conn.execute("CREATE TABLE IF NOT EXISTS internal_references (
        id TEXT PRIMARY KEY,
        note_id text references notes(id) not null,
        reference_id text references notes(id) not null
    )", ()).expect(&msg);

    conn.execute("CREATE TABLE IF NOT EXISTS sources (
        id TEXT PRIMARY KEY,
        title text not null unique
    )", ()).expect(&msg);

    conn.execute("CREATE TABLE IF NOT EXISTS external_references (
        id TEXT PRIMARY KEY,
        note_id text references notes(id) not null,
        reference_id text references sources(id) not null
    )", ()).expect(&msg);
}
