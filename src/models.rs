// src/models.rs
//Data structures (Structs)
#[derive(Clone, Debug, PartialEq)]
pub struct Project {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub link: String,
}