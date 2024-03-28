use regex::Regex;

pub mod parse;
pub mod error;

#[derive(Debug, Default, Clone)]
pub struct NoteFromMd {
    pub id: Option<String>,
    pub title: String,
    pub contents: String,
    pub references: References
}

#[derive(Debug, Default, Clone)]
pub struct References {
    pub internal: Vec<Reference>,
    pub external: Vec<Reference>,
}

#[derive(Debug, Default, Clone)]
pub struct Reference {
    pub id: Option<String>,
    pub title: Option<String>
}

pub fn generate_id() -> String {
    let number: i32 = rand::random();
    let id = base32::encode(base32::Alphabet::RFC4648 { padding: true }, &number.to_le_bytes());

    id[0..6].to_string()
}

pub fn extract_id(text: &str) -> Option<String> {
    let re = Regex::new(r"\[(?<id>\w{6})\]").ok()?;
    let Some(caps) = re.captures(text) else {
        return None;
    };

    Some(caps["id"].to_string())
}
