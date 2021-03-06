use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsmlFlow {
    pub id: String,
    pub name: String,
    pub content: String,
    pub commands: Vec<String>,
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl CsmlFlow {
    pub fn new(id: &str, name: &str, content: &str, commands: Vec<String>) -> Self {
        Self {
            id: id.to_owned(),
            name: name.to_owned(),
            content: content.to_owned(),
            commands,
        }
    }
}
