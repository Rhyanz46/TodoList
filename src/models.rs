use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Message {
    AddTodo,
    InputChanged(String),
    ToggleTodo(usize),
    DeleteTodo(usize),
    ClearCompleted,
    NewDay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub(crate) id: usize,
    pub(crate) text: String,
    pub(crate) completed: bool,
    pub(crate) created_at: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, Default)]
pub struct TodoApp {
    pub(crate) todos: Vec<Todo>,
    pub(crate) input_value: String,
    pub(crate) next_id: usize,
    pub(crate) current_date: chrono::NaiveDate,
}