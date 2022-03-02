use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub finished: bool,
    pub content: String,
}

pub type TodoList = Vec<Todo>;
