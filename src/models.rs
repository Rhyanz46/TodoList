use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Message {
    AddTodo,
    TodoInputChanged(String),
    TimeInputChanged(String),
    PrioritySelected(Priority),
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
    pub(crate) due_time: Option<chrono::NaiveTime>,
    pub(crate) priority: Priority,
    pub(crate) created_at: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, Default)]
pub struct TodoApp {
    pub(crate) todos: Vec<Todo>,
    pub(crate) todo_input: String,
    pub(crate) time_input: String,
    pub(crate) selected_priority: Priority,
    pub(crate) next_id: usize,
    pub(crate) current_date: chrono::NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    High, Medium, Low,
}