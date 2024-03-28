use rusqlite::Connection;

use super::error::DbError;

#[derive(Debug)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub contents: String,
}

#[derive(Debug)]
pub struct NoteListItem {
    pub id: String,
    pub title: String
}

impl Note {
    pub fn add(&self, conn: &Connection) -> Result<(), DbError> {
        conn.execute(
            "INSERT INTO notes (id, title, contents) VALUES (?1, ?2, ?3)",
            (&self.id, &self.title, &self.contents),
        )?;

        Ok(())
    }

    pub fn exists(title: String, conn: &Connection) -> Result<bool, DbError> {
        let mut statement = conn.prepare("select * from notes where title = ?1")?;
        let exists = statement.exists([title])?;

        Ok(exists)
    }

    pub fn get_by_title(title: String, conn: &Connection) -> Result<Option<Note>, DbError> {
        let note = conn.query_row("select * from notes where title = ?1", [title], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                contents: row.get(2)?
            })
        });
        
        match note {
            Ok(note) => Ok(Some(note)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into())
        }
    }

    pub fn get_by_id(id: String, conn: &Connection) -> Result<Option<Note>, DbError> {
        let note = conn.query_row("select * from notes where id = ?1", [id], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                contents: row.get(2)?
            })
        });
        
        match note {
            Ok(note) => Ok(Some(note)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into())
        }
    }

    pub fn list(conn: &Connection) -> Result<Vec<NoteListItem>, DbError> {
        let mut stmt = conn.prepare("select id, title from notes")?;
        let notes: Result<Vec<NoteListItem>, rusqlite::Error> = stmt.query_map([], |row| {
            Ok(NoteListItem {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        })?.collect();

        Ok(notes?)
    }

    pub fn update(&self, conn: &Connection) -> Result<(), DbError> {
        conn.execute(
            "update notes set title = ?1, contents = ?2 where id = ?3",
            (&self.title, &self.contents, &self.id)
        )?;

        Ok(())
    }
}
