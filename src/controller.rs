use std::{fmt::Display, fs::File, io::Write};

use clap::ArgMatches;
use console::style;
use csv::Writer;
use rusqlite::{Connection, Transaction};

use crate::{cli::{error::CliError, subcommands::{GetNote, NoteField, NoteFields, SourceField, SourceFields}}, models::{external::ExternalReference, internal::InternalReference, note::{Note, NoteListItem}, sources::Source}, util::{generate_id, parse::note_to_md, NoteFromMd, Reference}};


pub struct Controller {
    pub conn: Connection,
}

impl Controller {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn handle_command(mut self, matches: ArgMatches) -> Result<&'static str, CliError> {
        match matches.subcommand() {
            Some(("add", args)) => self.add(NoteFromMd::try_from(args)?),
            Some(("list", args)) => self.list(args),
            Some(("get", args)) => self.get(args),
            Some(("update", args)) => self.update(NoteFromMd::try_from(args)?),
            Some(("set", args)) => self.set(NoteFromMd::try_from(args)?),
            _ => Ok("")
        }
    }

    fn add(&mut self, note_from_md: NoteFromMd) -> Result<&'static str, CliError> {
        let note: Note = (&note_from_md).into();
        let note_id = note.id.clone();
        if note_from_md.title.trim().is_empty() {
            return Err(CliError::NoteTitleEmpty)
        }

        let tx = self.conn.transaction().unwrap();
        note.add(&tx)?;
        note_from_md.references.internal.iter()
            .try_for_each(|r| Self::add_internal_reference(r, &note_id, &tx))?;
        note_from_md.references.external.iter()
            .try_for_each(|r| Self::add_external_reference(r, &note_id, &tx))?;
        tx.commit().unwrap();

        Ok("Note added successfuly")

    }

    fn add_internal_reference(reference: &Reference, note_id: &str, tx: &Transaction) -> Result<(), CliError> {
        match (&reference.id, &reference.title) {
            (Some(id), _) => {
                if InternalReference::exists(note_id, &id, &tx)? {
                    return Ok(())
                }
                InternalReference::new(generate_id(), note_id.to_string(), id.to_string())
                                .add(&tx)?
            },
            (_, Some(title)) => Self::add_internal_reference_by_title(title, note_id, tx)?,
            (None, None) => Err(CliError::InvalidReference)?
        }

        Ok(())
    }

    fn add_internal_reference_by_title(title: &str, note_id: &str, tx: &Transaction) -> Result<(), CliError> {
        let note = Note::get_by_title(title.to_string(), &tx)?;
        if let Some(note) = note {
            if InternalReference::exists(note_id, &note.id, &tx)? {
                return Ok(())
            }
            InternalReference::new(generate_id(), note_id.to_string(), note.id)
                .add(&tx)?;
        }

        Err(CliError::ReferenceDoesNotExist(title.to_string()))
    }

    fn add_external_reference(reference: &Reference, note_id: &str, tx: &Transaction) -> Result<(), CliError> {
        match (&reference.id, &reference.title) {
            (Some(id), _) => {
                if ExternalReference::exists(note_id, &id, &tx)? {
                    return Ok(())
                }
                ExternalReference::new(generate_id(), note_id.to_string(), id.to_string())
                                .add(&tx)?
            },
            (_, Some(title)) => Self::add_external_reference_by_title(title, note_id, tx)?,
            (None, None) => Err(CliError::InvalidReference)?
        };

        Ok(())
    }

    fn add_external_reference_by_title(title: &str, note_id: &str, tx: &Transaction) -> Result<(), CliError> {
        let source = Source::get_by_title(title.to_string(), &tx)?;
        let source = source.unwrap_or(Source::create(generate_id(), title.to_string(), &tx)?);

        if ExternalReference::exists(note_id, &source.id, &tx)? {
            return Ok(())
        }

        ExternalReference::new(generate_id(), note_id.to_string(), source.id)
            .add(&tx)?;

        Ok(())
    }

    fn list(&self, args: &ArgMatches) -> Result<&'static str, CliError> {
        match args.subcommand() {
            Some(("notes", args)) => self.list_notes(NoteFields::try_from(args)?),
            Some(("sources", args)) => self.list_sources(SourceFields::try_from(args)?),
            _ => Ok("")
        }
    }

    fn list_notes(&self, fields: NoteFields) -> Result<&'static str, CliError> {
        let mut wtr = Writer::from_writer(vec![]);
        let notes = Note::list(&self.conn)?;

        for NoteListItem { id, title } in notes {
            let record = Self::note_item_to_record(&id, &title, &fields);

            if Self::handle_single_column(&record) {
                continue; 
            }

            wtr.write_record(&record)
                .map_err(|_| CliError::InternalError)?;
        }

        Self::print_csv_record(wtr)?;


        Ok("")
    }

    fn list_sources(&self, fields: SourceFields) -> Result<&'static str, CliError> {
        let mut wtr = Writer::from_writer(vec![]);
        let notes = Source::list(&self.conn)?;

        for Source { id, title } in notes {
            let record = Self::source_to_record(&id, &title, &fields);

            if Self::handle_single_column(&record) {
                continue; 
            }

            wtr.write_record(&record)
                .map_err(|_| CliError::InternalError)?;
        }

        Self::print_csv_record(wtr)?;


        Ok("")
    }

    fn note_item_to_record<'a>(id: &'a str, title: &'a str, fields: &NoteFields) -> Vec<&'a str> {
        let mut record = vec![];
        for item in &fields.items {
            match item {
                NoteField::Id => record.push(id),
                NoteField::Title => record.push(title)
            }
        }

        record
    }

    fn source_to_record<'a>(id: &'a str, title: &'a str, fields: &SourceFields) -> Vec<&'a str> {
        let mut record = vec![];
        for item in &fields.items {
            match item {
                SourceField::Id => record.push(id),
                SourceField::Title => record.push(title)
            }
        }

        record
    }

    fn handle_single_column(record: &[&str]) -> bool {
        if record.len() == 1 {
            println!("{}", record[0]);
            return true
        }

        false
    }
    
    fn print_csv_record(wtr: Writer<Vec<u8>>) -> Result<(), CliError> {
        let contents = wtr.into_inner().map_err(|_| CliError::InternalError)?;
        let contents = String::from_utf8(contents).map_err(|_| CliError::InvalidUtf8)?;
        let contents = contents.trim();

        if !contents.is_empty() {
            println!("{}", contents.trim());
        }

        Ok(())
    }


    fn get(&self, args: &ArgMatches) -> Result<&'static str, CliError> {
        match args.subcommand() {
            Some(("note", args)) => self.get_note(GetNote::try_from(args)?),
            _ => Ok("")
        }
    }

    fn get_note(&self, get_note: GetNote) -> Result<&'static str, CliError> {
        let note = Note::get_by_id(get_note.id, &self.conn)?
            .ok_or(CliError::NoteNotFound)?;
        let internal = InternalReference::get_by_note_id(&note.id, &self.conn)?;
        let external = ExternalReference::get_by_note_id(&note.id, &self.conn)?;
        
        let md_note = note_to_md(note, internal, external);

        let mut file = File::create(&get_note.path)?;
        file.write_all(md_note.as_bytes())?;

        let message = format!("Note written to {} successfuly", get_note.path);
        
        eprintln!("{}", style(message).bold().green());

        Ok("")
    }
    
    fn update(&mut self, note_from_md: NoteFromMd) -> Result<&'static str, CliError> {
        let note: Note = (&note_from_md).into();
        if Note::get_by_id(note.id.clone(), &self.conn)?.is_none() {
            return Err(CliError::NoteNotFound)
        }

        let note_id = note.id.clone();

        let tx = self.conn.transaction().unwrap();
        note.update(&tx)?;

        InternalReference::delete_by_note_id(&note_id, &tx)?;

        note_from_md.references.internal.iter()
            .try_for_each(|r| Self::add_internal_reference(r, &note_id, &tx))?;

        ExternalReference::delete_by_note_id(&note_id, &tx)?;
        note_from_md.references.external.iter()
            .try_for_each(|r| Self::add_external_reference(r, &note_id, &tx))?;

        tx.commit().unwrap();

        Ok("Note updated successfuly")

    }

    fn set(&mut self, mut note_from_md: NoteFromMd) -> Result<&'static str, CliError> {
        let msg = "Note set successfuly";

        match note_from_md.id {
            Some(ref id) => {
                let note = Note::get_by_id(id.clone(), &self.conn)?;

                match note {
                    Some(_) => self.update(note_from_md)?,
                    None => self.add(note_from_md)?
                };
            },
            None => {
                let note = Note::get_by_title(note_from_md.title.clone(), &self.conn)?;

                match note {
                    Some(note) => {
                        note_from_md.id = Some(note.id);
                        self.update(note_from_md)?
                    },
                    None => self.add(note_from_md)?,
                };
            },
        }



        Ok(msg)
    }
}

impl From<&NoteFromMd> for Note {
    fn from(value: &NoteFromMd) -> Self {
        Self {
            id: value.id.clone().unwrap_or(generate_id()),
            title: value.title.clone(),
            contents: value.contents.clone()
        }
    }
}

impl Display for NoteListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id = &self.id;
        let title  = &self.title;
        write!(f, "{id} | {title}")
    }
}
