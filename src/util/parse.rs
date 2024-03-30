use std::cell::RefCell;

use comrak::{arena_tree::Node, nodes::{Ast, AstNode, NodeHeading, NodeValue}, parse_document, Arena, Options};

use crate::{models::{note::Note, sources::Source}, util::extract_id};

use super::{error::UtilError, NoteFromMd, Reference};

pub fn md_to_new_note(text: String) -> Result<NoteFromMd, UtilError>{
    let parser = MdToNoteParser::default();

    let note = parser.parse(text);

    note
}

pub fn note_to_md(note: Note, internal: Vec<Note>, external: Vec<Source>) -> String {
    let mut md_note = format!("# [{}] {}{}\n## References\n### Internal\n", note.id, note.title, note.contents);

    for (i, note) in internal.iter().enumerate() {
        let reference = format!("{}. [{}] {}\n", i + 1, note.id, note.title);
        md_note.push_str(&reference);
    }

    md_note.push_str("\n### External\n");

    for source in external {
        let reference = format!(" - [{}] {}\n", source.id, source.title);
        md_note.push_str(&reference);
    }

    md_note
}

#[derive(Default)]
struct MdToNoteParser {
    note: NoteFromMd,
    stage: ParsingStage
}

impl MdToNoteParser {
    pub fn parse(mut self, text: String) -> Result<NoteFromMd, UtilError> {
        let arena = Arena::new();

        let root = parse_document(
            &arena,
            &text,
            &Options::default()
        );

        iter_nodes(root, &mut |node| -> Result<(), UtilError> {self.parse_node(node)?; Ok(())})?;
        self.extract_contents(text);

        Ok(self.note)
    }

    fn parse_node(&mut self, node: &Node<'_, RefCell<Ast>>) -> Result<(), UtilError> {
        match &node.data.borrow().value {
            NodeValue::Heading(NodeHeading { level: 1, .. }) => self.title_stage()?,
            NodeValue::Heading(NodeHeading { level: 2, .. }) => self.references_stage()?,
            NodeValue::Heading(NodeHeading { level: 3, .. }) 
                if self.stage == ParsingStage::References || self.stage == ParsingStage::InternalReferenceItems => self.stage.next(),
            NodeValue::Text(text) => self.handle_current_stage(text),
            _ => ()
        };

        Ok(())
    }

    fn extract_contents(&mut self, text: String) {
        let mut contents = String::new();
        let mut adding = false;
        for line in text.lines() {
            if line.trim().starts_with("# ") {
                adding = true;
                continue;
            }
            if line.trim().starts_with("## References") {
                break;
            }
            if adding {
                contents = format!("{contents}\n{line}");
            }
        }

        self.note.contents = contents;
    }

    fn title_stage(&mut self) -> Result<(), UtilError> {
        if self.stage != ParsingStage::Start {
            return Err(UtilError::InvalidNoteMarkdown)
        }
        self.stage.next();
        Ok(())
    }


    fn references_stage(&mut self) -> Result<(), UtilError> {
        if self.stage != ParsingStage::Title {
            return Err(UtilError::InvalidNoteMarkdown)
        }
        self.stage.next();
        Ok(())
    }

    fn handle_current_stage(&mut self, text: &str) {
        match self.stage {
            ParsingStage::Title => {
                if self.note.title.is_empty() {
                    if let Some(id) = extract_id(text) {
                        self.note.title = text.replace(&format!("[{}]", id), "").trim().to_string();
                        self.note.id = Some(id);
                        return
                    }
                    self.note.title = text.trim().to_string();
                }
            }
            ParsingStage::References => {
                if text == "References" {
                    return
                }

                self.stage.prev()
            }
            ParsingStage::InternalReferences => {
                if text == "Internal" {
                    self.stage.next();
                    return 
                }

                self.stage.prev()
            }
            ParsingStage::InternalReferenceItems => {
                let reference = Reference { id: extract_id(text), title: Some(text.to_string())};
                self.note.references.internal.push(reference);
            }
            ParsingStage::ExternalReferences => {
                if text == "External" {
                    self.stage.next();
                    return 
                }

                self.stage.prev()
            }
            ParsingStage::ExternalReferenceItems => {
                let reference = Reference { id: extract_id(text), title: Some(text.to_string())};
                self.note.references.external.push(reference);
            }
            _ => ()
        }

    }
}

fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &mut F) -> Result<(), UtilError>
    where F : FnMut(&'a AstNode<'a>) -> Result<(), UtilError> {
    f(node)?;
    for c in node.children() {
        iter_nodes(c, f)?;
    }

    Ok(())
}


#[derive(PartialEq, Default, Debug)]
enum ParsingStage {
    #[default]
    Start,
    Title,
    References,
    InternalReferences,
    InternalReferenceItems,
    ExternalReferences,
    ExternalReferenceItems,
    Finish
}

impl ParsingStage {
    pub fn next(&mut self) {
        *self = match self {
            Self::Start => Self::Title,
            Self::Title => Self::References,
            Self::References => Self::InternalReferences,
            Self::InternalReferences => Self::InternalReferenceItems,
            Self::InternalReferenceItems => Self::ExternalReferences,
            Self::ExternalReferences => Self::ExternalReferenceItems,
            Self::ExternalReferenceItems => Self::Finish,
            Self::Finish => Self::Finish,
        };
    }

    pub fn prev(&mut self) {
        *self = match self {
            Self::Finish => Self::ExternalReferenceItems,
            Self::ExternalReferenceItems => Self::ExternalReferences,
            Self::ExternalReferences => Self::InternalReferenceItems,
            Self::InternalReferenceItems => Self::InternalReferences,
            Self::InternalReferences => Self::References,
            Self::References => Self::Title,
            Self::Title => Self::Start,
            Self::Start => Self::Start,
        };
    }
}
