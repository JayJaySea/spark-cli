use rusqlite::Connection;

use super::{error::DbError, sources::Source};

#[derive(Debug)]
pub struct ExternalReference {
    pub id: String,
    pub note_id: String,
    pub reference_id: String,
}

impl ExternalReference {
    pub fn new(id: String, note_id: String, reference_id: String) -> Self {
        Self {
            id,
            note_id,
            reference_id
        }
    }
    pub fn add(&self, conn: &Connection) -> Result<(), DbError> {
        conn.execute(
            "INSERT INTO external_references (id, note_id, reference_id) VALUES (?1, ?2, ?3)",
            (&self.id, &self.note_id, &self.reference_id),
        )?;

        Ok(())
    }

    pub fn get_by_note_id(note_id: &str, conn: &Connection) -> Result<Vec<Source>, DbError> {
        let mut stmt = conn.prepare("SELECT reference_id FROM external_references where note_id = ?1")?;

        let references = stmt.query_map([note_id], |row| {
            Ok(row.get::<usize, String>(0)?)
        })?;

        let mut sources = vec![];
        for reference_id in references {
            let source = Source::get_by_id(reference_id?, conn)?
                .ok_or(DbError::InternalError)?;
            sources.push(source);
        }

        sources.sort_by(|a, b| a.title.cmp(&b.title));
        Ok(sources)
    }

    pub fn delete_by_note_id(note_id: &str, conn: &Connection) -> Result<(), DbError> {
        conn.execute(
            "DELETE FROM external_references WHERE note_id = ?1", 
            (&note_id,),
            )?;

        Ok(())
    }
}
