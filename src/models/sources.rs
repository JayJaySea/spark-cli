use rusqlite::Connection;

use super::error::DbError;

#[derive(Debug)]
pub struct Source {
    pub id: String,
    pub title: String,
}

impl Source {
    pub fn new(id: String, title: String) -> Self {
        Self {
            id,
            title
        }
    }
    pub fn create(id: String, title: String, conn: &Connection) -> Result<Self, DbError> {
        let source = Self {
            id,
            title
        };
        source.add(conn)?;
        Ok(source)
    }

    pub fn add(&self, conn: &Connection) -> Result<(), DbError> {
        conn.execute(
            "INSERT INTO sources (id, title) VALUES (?1, ?2)",
            (&self.id, &self.title),
        )?;

        Ok(())
    }

    pub fn get_by_id(id: String, conn: &Connection) -> Result<Option<Source>, DbError> {
        let source = conn.query_row("select * from sources where id = ?1", [id], |row| {
            Ok(Source {
                id: row.get(0)?,
                title: row.get(1)?,
            })
        });
        
        match source {
            Ok(source) => Ok(Some(source)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into())
        }
    }

    pub fn get_by_title(title: String, conn: &Connection) -> Result<Option<Source>, DbError> {
        let source = conn.query_row("select * from sources where title = ?1", [title], |row| {
            Ok(Source {
                id: row.get(0)?,
                title: row.get(1)?
            })
        });
        
        match source {
            Ok(source) => Ok(Some(source)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into())
        }
    }
}
