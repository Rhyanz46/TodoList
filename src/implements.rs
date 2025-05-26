use iced::{Application, Command, Element, Length, Theme};
use iced::widget::{button, checkbox, container, row, scrollable, text, text_input};

use crate::models::{Message, Todo, TodoApp};

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
                if !self.input_value.trim().is_empty() {
                    let todo = Todo {
                        id: self.next_id,
                        text: self.input_value.trim().to_string(),
                        completed: false,
                        created_at: chrono::Local::now(),
                    };
                    self.todos.push(todo);
                    self.next_id += 1;
                    self.input_value.clear();
                    self.save_todos();
                }
            }
            Message::InputChanged(value) => {
                self.input_value = value;
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
                    self.input_value.clear();
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

        let input = text_input("Tambah tugas baru...", &self.input_value)
            .on_input(Message::InputChanged)
            .on_submit(Message::AddTodo)
            .padding(10);

        let add_button = button("Tambah")
            .on_press(Message::AddTodo)
            .padding(10);

        let input_row = row![input, add_button].spacing(10);

        let new_day_button = button("Hari Baru")
            .on_press(Message::NewDay)
            .padding(5);

        let clear_button = button("Hapus Selesai")
            .on_press(Message::ClearCompleted)
            .padding(5);

        let controls = row![new_day_button, clear_button].spacing(10);

        let todo_list = if self.todos.is_empty() {
            iced::widget::column![text("Belum ada tugas untuk hari ini").size(16)]
        } else {
            let mut todos = iced::widget::column![].spacing(5);
            for todo in &self.todos {
                let checkbox = checkbox("", todo.completed)
                    .on_toggle(move |_| Message::ToggleTodo(todo.id));

                let todo_text = text(&todo.text)
                    .size(16)
                    .style(if todo.completed {
                        iced::theme::Text::Color(iced::Color::from_rgb(0.5, 0.5, 0.5))
                    } else {
                        iced::theme::Text::Default
                    });

                let delete_button = button("🗑")
                    .on_press(Message::DeleteTodo(todo.id))
                    .padding(5);

                let todo_row = row![
                    checkbox,
                    todo_text.width(Length::Fill),
                    delete_button
                ]
                    .spacing(10)
                    .align_items(iced::Alignment::Center);

                todos = todos.push(todo_row);
            }
            todos
        };

        let stats = {
            let total = self.todos.len();
            let completed = self.todos.iter().filter(|t| t.completed).count();
            let remaining = total - completed;

            text(format!("Total: {} | Selesai: {} | Tersisa: {}", total, completed, remaining))
                .size(14)
        };

        let content = iced::widget::column![
            title,
            input_row,
            controls,
            stats,
            scrollable(todo_list).height(Length::Fill)
        ]
            .spacing(20)
            .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}