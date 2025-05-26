use std::fs;
use std::path::PathBuf;
use crate::models::{Todo, TodoApp};

impl TodoApp {
    fn data_file_path(&self) -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_default();
        path.push(".daily_todo");
        fs::create_dir_all(&path).ok();
        path.push(format!("todos_{}.json", self.current_date.format("%Y_%m_%d")));
        path
    }

    pub(crate) fn save_todos(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.todos) {
            fs::write(self.data_file_path(), json).ok();
        }
    }

    pub(crate) fn load_todos(&mut self) {
        let path = self.data_file_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(todos) = serde_json::from_str::<Vec<Todo>>(&content) {
                    self.todos = todos;
                    self.next_id = self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
                }
            }
        }
    }
}

impl Todo {
    pub(crate) fn is_overdue(&self) -> bool {
        if let Some(due_time) = self.due_time {
            let now = chrono::Local::now().time();
            !self.completed && now > due_time
        } else {
            false
        }
    }

    pub(crate) fn time_until_due(&self) -> Option<String> {
        println!("test until due");
        None
    }
}