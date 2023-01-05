use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub desc: String,
    pub is_done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTodo {
    pub title: String,
    pub desc: String,
    pub is_done: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub desc: Option<String>,
    pub is_done: Option<bool>,
}
