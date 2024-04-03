use std::fs;

use clap::{arg, value_parser, ArgMatches, Command};

use crate::util::{parse, NoteFromMd, References};

use super::{error::CliError, ParseArgs};

pub fn add() -> Command {
    Command::new("add")
        .args([
            arg!(-p --path <path> "Path to .md file in compatible format")
                .value_parser(value_parser!(String))
                .required(true)
        ])
}

pub fn list() -> Command {
    Command::new("list")
        .subcommand(list_notes())
        .subcommand(list_sources())
}

pub fn list_notes() -> Command {
    Command::new("notes")
        .args([
            arg!(--id "Show note id"), 
            arg!(--title "Show note title")
        ])
}

pub fn list_sources() -> Command {
    Command::new("sources")
        .args([
            arg!(--id "Show source id"), 
            arg!(--title "Show source title")
        ])
}

pub fn get() -> Command {
    Command::new("get")
        .subcommand(get_note())
}

pub fn get_note() -> Command {
    Command::new("note")
        .args([
            arg!(<id> "Id of a note to get")
                .required(true)
                .value_parser(value_parser!(String)),
            arg!(-p --path <path> "Will output the note there")
                .value_parser(value_parser!(String))
                .required(true)
        ])
}

pub fn update() -> Command {
    Command::new("update")
        .args([
            arg!(-p --path <path> "Path to .md file in compatible format")
                .value_parser(value_parser!(String))
                .required(true)
        ])
}

pub fn set() -> Command {
    Command::new("set")
        .args([
            arg!(-p --path <path> "Path to .md file in compatible format")
                .value_parser(value_parser!(String))
                .required(true)
        ])
}

impl ParseArgs for NoteFromMd {}
impl TryFrom<&ArgMatches> for NoteFromMd {
    type Error = CliError;
    fn try_from(value: &ArgMatches) -> Result<Self, Self::Error> {
        let path = Self::parse_option_string(value, "path")
            .ok_or(CliError::InternalError)?;

        let contents = fs::read_to_string(path)
            .map_err(|msg| CliError::CannotOpenFile(msg.to_string()))?;

        let note = parse::md_to_new_note(contents)?;

        Ok(note)
    }
}

#[derive(Debug, Clone)]
pub struct NoteFields {
    pub items: Vec<NoteField>
}

#[derive(Debug, Clone)]
pub enum NoteField {
    Id,
    Title
}

impl Default for NoteFields {
    fn default() -> Self {
        Self {
            items: vec![NoteField::Id, NoteField::Title]
        }
    }
}

impl ParseArgs for NoteFields {}

impl TryFrom<&ArgMatches> for NoteFields {
    type Error = CliError;

    fn try_from(value: &ArgMatches) -> Result<Self, Self::Error> {
        let id = Self::parse_option(value, "id")
            .unwrap_or(false);
        let title = Self::parse_option(value, "title")
            .unwrap_or(false);

        let list;

        list = if !title && !id {
             NoteFields::default()
        }
        else {
             let mut items = Vec::new();
             if id { items.push(NoteField::Id) }
             if title { items.push(NoteField::Title)}

             NoteFields { items }
        };

        Ok(list)
    }
}

#[derive(Debug, Clone)]
pub struct SourceFields {
    pub items: Vec<SourceField>
}

#[derive(Debug, Clone)]
pub enum SourceField {
    Id,
    Title
}

impl Default for SourceFields {
    fn default() -> Self {
        Self {
            items: vec![SourceField::Id, SourceField::Title]
        }
    }
}

impl ParseArgs for SourceFields {}

impl TryFrom<&ArgMatches> for SourceFields {
    type Error = CliError;

    fn try_from(value: &ArgMatches) -> Result<Self, Self::Error> {
        let id = Self::parse_option(value, "id")
            .unwrap_or(false);
        let title = Self::parse_option(value, "title")
            .unwrap_or(false);

        let list;

        list = if !title && !id {
             SourceFields::default()
        }
        else {
             let mut items = Vec::new();
             if id { items.push(SourceField::Id) }
             if title { items.push(SourceField::Title)}

             SourceFields { items }
        };

        Ok(list)
    }
}

#[derive(Debug, Clone)]
pub struct GetNote {
    pub id: String,
    pub path: String
}

impl ParseArgs for GetNote { }

impl TryFrom<&ArgMatches> for GetNote {
    type Error = CliError;

    fn try_from(value: &ArgMatches) -> Result<Self, Self::Error> {
        let id = Self::parse_option_string(value, "id")
            .ok_or(CliError::InternalError)?;
        let path = Self::parse_option_string(value, "path")
            .ok_or(CliError::InternalError)?;

        let get_note = GetNote {
            id,
            path
        };

        Ok(get_note)
    }
}
