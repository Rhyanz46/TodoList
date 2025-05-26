use iced::{Application, Command, Element, Length, Theme};
use iced::widget::{button, checkbox, container, row, scrollable, text, text_input};

use crate::models::{Message, Priority, Todo, TodoApp};

impl Priority {
    fn all() -> &'static [Priority] {
        &[Priority::High, Priority::Medium, Priority::Low]
    }

    fn as_str(&self) -> &str {
        match self {
            Priority::High => "🔴 Tinggi",
            Priority::Medium => "🟡 Sedang",
            Priority::Low => "🟢 Rendah",
        }
    }

    fn emoji(&self) -> &str {
        match self {
            Priority::High => "🔴",
            Priority::Medium => "🟡",
            Priority::Low => "🟢",
        }
    }
}

impl Application for TodoApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let mut app = TodoApp {
            current_date: chrono::Local::now().date_naive(),
            ..Default::default()
        };

        app.load_todos();
        (app, Command::none())
    }

    fn title(&self) -> String {
        format!("Daily Todo - {}", self.current_date.format("%d %B %Y"))
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::AddTodo => {
                if !self.todo_input.trim().is_empty() {
                    let due_time = if self.time_input.trim().is_empty() {
                        None
                    } else {
                        chrono::NaiveTime::parse_from_str(&self.time_input.trim(), "%H:%M").ok()
                    };

                    let todo = Todo {
                        id: self.next_id,
                        text: self.todo_input.trim().to_string(),
                        completed: false,
                        due_time,
                        priority: self.selected_priority.clone(),
                        created_at: chrono::Local::now(),
                    };
                    self.todos.push(todo);
                    self.next_id += 1;
                    self.todo_input.clear();
                    self.time_input.clear();
                    self.selected_priority = Priority::Medium;
                    self.save_todos();
                }
            }
            Message::TodoInputChanged(value) => {
                self.todo_input = value;
            }
            Message::TimeInputChanged(value) => {
                self.time_input = value;
            }
            Message::PrioritySelected(priority) => {
                self.selected_priority = priority;
            }
            Message::ToggleTodo(id) => {
                if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
                    todo.completed = !todo.completed;
                    self.save_todos();
                }
            }
            Message::DeleteTodo(id) => {
                self.todos.retain(|t| t.id != id);
                self.save_todos();
            }
            Message::ClearCompleted => {
                self.todos.retain(|t| !t.completed);
                self.save_todos();
            }
            Message::NewDay => {
                let today = chrono::Local::now().date_naive();
                if today != self.current_date {
                    self.current_date = today;
                    self.todos.clear();
                    self.todo_input.clear();
                    self.next_id = 0;
                    self.load_todos();
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let title = text(format!("Todo Harian - {}", self.current_date.format("%d %B %Y")))
            .size(24)
            .width(Length::Fill);

        let todo_input = text_input("Tambah tugas baru...", &self.todo_input)
            .on_input(Message::TodoInputChanged)
            .on_submit(Message::AddTodo)
            .padding(10)
            .width(Length::FillPortion(3));

        let time_input = text_input("HH:MM", &self.time_input)
            .on_input(Message::TimeInputChanged)
            .on_submit(Message::AddTodo)
            .padding(10)
            .width(Length::FillPortion(1));

        let priority_picker = iced::widget::pick_list(
            &Priority::all()[..],
            Some(self.selected_priority.clone()),
            Message::PrioritySelected, )
            .width(Length::FillPortion(1))
            .padding(10);

        let add_button = button("Tambah")
            .on_press(Message::AddTodo)
            .padding(10);

        let input_row = row![todo_input, time_input, priority_picker, add_button].spacing(10);
        // let input_row = row![todo_input, time_input, add_button].spacing(10);

        let new_day_button = button("Hari Baru")
            .on_press(Message::NewDay)
            .padding(5);

        let clear_button = button("Hapus Selesai")
            .on_press(Message::ClearCompleted)
            .padding(5);

        let controls = row![new_day_button, clear_button].spacing(10);

        let mut indices: Vec<usize> = (0..self.todos.len()).collect();
        indices.sort_by(|&a, &b| {
            let todo_a = &self.todos[a];
            let todo_b = &self.todos[b];

            // First sort by completion status
            match (todo_a.completed, todo_b.completed) {
                (true, false) => std::cmp::Ordering::Greater,
                (false, true) => std::cmp::Ordering::Less,
                _ => {
                    // Then by overdue status
                    match (todo_a.is_overdue(), todo_b.is_overdue()) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => {
                            // Then by priority
                            let priority_order = |p: &Priority| match p {
                                Priority::High => 0,
                                Priority::Medium => 1,
                                Priority::Low => 2,
                            };
                            match priority_order(&todo_a.priority).cmp(&priority_order(&todo_b.priority)) {
                                std::cmp::Ordering::Equal => {
                                    // Finally by due time
                                    match (todo_a.due_time, todo_b.due_time) {
                                        (Some(a_time), Some(b_time)) => a_time.cmp(&b_time),
                                        (Some(_), None) => std::cmp::Ordering::Less,
                                        (None, Some(_)) => std::cmp::Ordering::Greater,
                                        (None, None) => std::cmp::Ordering::Equal,
                                    }
                                }
                                other => other,
                            }
                        }
                    }
                }
            }
        });

        let todo_list = if self.todos.is_empty() {
            iced::widget::column![text("Belum ada tugas untuk hari ini").size(16)]
        } else {
            let mut todos_column = iced::widget::column![].spacing(8);
            for &index in &indices {
                let todo = &self.todos[index];
                let checkbox = checkbox("", todo.completed)
                    .on_toggle(move |_| Message::ToggleTodo(todo.id));

                let todo_text_style = if todo.completed {
                    iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5))
                } else if todo.is_overdue() {
                    iced::theme::Text::Color(iced::Color::from_rgb(0.8, 0.2, 0.2))
                } else {
                    iced::theme::Text::Default
                };

                let todo_text_content = if let Some(due_time) = todo.due_time {
                    format!("{} ({})", todo.text, due_time.format("%H:%M"))
                } else {
                    todo.text.clone()
                };

                let todo_text = text(todo_text_content)
                    .size(16)
                    .style(todo_text_style);

                let priority_badge = text(todo.priority.emoji())
                    .size(12);

                let time_info = if let Some(time_str) = todo.time_until_due() {
                    text(time_str)
                        .size(12)
                        .style(if todo.is_overdue() {
                            iced::theme::Text::Color(iced::Color::from_rgb(0.8, 0.2, 0.2))
                        } else {
                            iced::theme::Text::Color(iced::Color::from_rgb(0.3, 0.6, 0.3))
                        })
                } else {
                    text("")
                };

                let delete_button = button("🗑")
                    .on_press(Message::DeleteTodo(todo.id))
                    .padding(5);

                let todo_row = row![
                    checkbox,
                    priority_badge,
                    todo_text.width(Length::Fill),
                    time_info,
                    delete_button
                ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center);

                todos_column = todos_column.push(todo_row);
            }
            todos_column
        };

        let stats = {
            let total = self.todos.len();
            let completed = self.todos.iter().filter(|t| t.completed).count();
            let remaining = total - completed;

            text(format!("Total: {} | Selesai: {} | Tersisa: {}", total, completed, remaining)).size(14)
        };

        let content = iced::widget::column![
            title,
            input_row,
            controls,
            stats,
            scrollable(todo_list).height(Length::Fill)
        ].spacing(20).padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}