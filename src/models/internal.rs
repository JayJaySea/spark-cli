use rusqlite::Connection;

use super::{error::DbError, note::Note};

#[derive(Debug)]
pub struct InternalReference {
    pub id: String,
    pub note_id: String,
    pub reference_id: String,
}

impl InternalReference {
    pub fn new(id: String, note_id: String, reference_id: String) -> Self {
        Self {
            id,
            note_id,
            reference_id
        }
    }

    pub fn add(&self, conn: &Connection) -> Result<(), DbError> {
        conn.execute(
            "INSERT INTO internal_references (id, note_id, reference_id) VALUES (?1, ?2, ?3)",
            (&self.id, &self.note_id, &self.reference_id),
        )?;

        Ok(())
    }
    
    pub fn get_by_note_id(note_id: &str, conn: &Connection) -> Result<Vec<Note>, DbError> {
        let mut stmt = conn.prepare("SELECT reference_id FROM internal_references where note_id = ?1")?;

        let references = stmt.query_map([note_id], |row| {
            Ok(row.get::<usize, String>(0)?)
        })?;

        let mut notes = vec![];
        for reference_id in references {
            let note = Note::get_by_id(reference_id?, conn)?
                .ok_or(DbError::InternalError)?;
            notes.push(note);
        }

        notes.sort_by(|a, b| a.title.cmp(&b.title));

        Ok(notes)
    }

    pub fn get_by_note_id_raw(note_id: &str, conn: &Connection) -> Result<Vec<InternalReference>, DbError> {
        let mut stmt = conn.prepare("SELECT reference_id FROM internal_references where note_id = ?1")?;

        let references = stmt.query_map([note_id], |row| {
            Ok(InternalReference{
                id: row.get(0)?,
                note_id: row.get(1)?,
                reference_id: row.get(2)?,
            })
        })?;

        let references: Result<Vec<InternalReference>, rusqlite::Error> = references.collect();

        Ok(references?)
    }

    pub fn delete(self, conn: &Connection) -> Result<(), DbError> {
        conn.execute(
            "DELETE FROM internal_references WHERE id = ?1", 
            (&self.id,),
            )?;

        Ok(())
    }

    pub fn delete_by_note_id(note_id: &str, conn: &Connection) -> Result<(), DbError> {
        conn.execute(
            "DELETE FROM internal_references WHERE note_id = ?1", 
            (&note_id,),
            )?;

        Ok(())
    }

    pub fn exists(note_id: &str, reference_id: &str, conn: &Connection) -> Result<bool, DbError> {
        let mut statement = conn.prepare("select * from internal_references where note_id = ?1 and reference_id = ?2")?;
        let exists = statement.exists([note_id, reference_id])?;

        Ok(exists)
    }
}
