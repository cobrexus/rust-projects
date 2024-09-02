use iced::{
    alignment::Horizontal,
    widget::{button, checkbox, column, container, keyed_column, row, text_input},
    Element, Length, Sandbox, Settings, Theme,
};

pub fn main() -> iced::Result {
    TodoList::run(Settings::default())
}

struct TodoList {
    input_value: String,
    todos: Vec<Todo>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    InputSubmitted,
    TodoMessage(usize, TodoMessage),
}

impl Sandbox for TodoList {
    type Message = Message;

    fn new() -> TodoList {
        TodoList {
            input_value: String::new(),
            todos: Vec::new(),
        }
    }

    fn title(&self) -> String {
        String::from("Todo List - Iced")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(val) => {
                self.input_value = val;
            }
            Message::InputSubmitted => {
                self.todos.push(Todo::new(self.input_value.clone()));
                self.input_value = String::new();
            }
            Message::TodoMessage(idx, TodoMessage::TodoDone(done)) => self.todos[idx].done = done,
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                row![
                    text_input("Add an Item...", &self.input_value)
                        .on_input(Message::InputChanged)
                        .on_submit(Message::InputSubmitted)
                        .padding(10),
                    button(container("Add"))
                        .on_press(Message::InputSubmitted)
                        .padding(10),
                ]
                .spacing(10)
                .width(700),
                keyed_column(self.todos.iter().enumerate().map(|(idx, todo)| {
                    (
                        idx,
                        todo.view().map(move |msg| Message::TodoMessage(idx, msg)),
                    )
                }))
                .spacing(10)
            ]
            .spacing(10),
        )
        .width(Length::Fill)
        .align_x(Horizontal::Center)
        .padding(20)
        .into()
    }
}

struct Todo {
    content: String,
    done: bool,
}

#[derive(Debug, Clone)]
enum TodoMessage {
    TodoDone(bool),
}

impl Todo {
    fn new(content: String) -> Self {
        Self {
            content,
            done: false,
        }
    }

    fn view(&self) -> Element<TodoMessage> {
        checkbox(&self.content, self.done)
            .on_toggle(TodoMessage::TodoDone)
            .into()
    }
}
