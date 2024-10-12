use iced::{
    application, border, color, daemon,
    widget::{
        button, checkbox, column, container, keyed_column, mouse_area, row, text_input,
        vertical_space,
    },
    Background, Border, Color, Element, Font, Length, Pixels, Result, Settings, Shadow,
};

pub fn main() -> Result {
    application("Iced Todos", App::update, App::view)
        .settings(App::settings())
        .style(|_, _| daemon::Appearance {
            background_color: color!(0x000000),
            text_color: color!(0xFFFFFF),
        })
        .run()
}

#[derive(Default)]
struct App {
    new_item_input_value: String,
    items: Vec<Item>,
    next_task_id: i32,
}

#[derive(Debug, Clone)]
enum Message {
    NewItemInputValueChanged(String),
    NewItemInputSubmitted,
    ItemToggled(i32, bool),
}

#[derive(PartialEq)]
struct Item {
    id: i32,
    content: String,
    done: bool,
}

impl App {
    fn settings() -> Settings {
        Settings {
            id: None,
            fonts: vec![include_bytes!("../JetBrainsMono-Regular.ttf")
                .as_slice()
                .into()],
            default_font: Font::with_name("JetBrains Mono"),
            default_text_size: Pixels(15.0),
            antialiasing: Default::default(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NewItemInputValueChanged(s) => {
                self.new_item_input_value = s;
            }
            Message::NewItemInputSubmitted => {
                self.items.push(Item {
                    id: self.next_task_id,
                    content: self.new_item_input_value.clone(),
                    done: false,
                });
                self.new_item_input_value.clear();
                self.next_task_id += 1;
            }
            Message::ItemToggled(id, done) => {
                self.items[id as usize].done = !done;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                vertical_space().height(30),
                row![
                    text_input("Add item", &self.new_item_input_value)
                        .on_input(Message::NewItemInputValueChanged)
                        .on_submit(Message::NewItemInputSubmitted)
                        .style(|_, _| text_input::Style {
                            background: Background::Color(Color::TRANSPARENT),
                            border: Border {
                                color: color!(0xFFFFFF),
                                width: 1.0,
                                radius: border::Radius {
                                    top_left: 5.0,
                                    top_right: 5.0,
                                    bottom_right: 5.0,
                                    bottom_left: 5.0,
                                },
                            },
                            icon: color!(0xFFFFFF),
                            placeholder: color!(0xAAAAAA),
                            value: color!(0xFFFFFF),
                            selection: color!(0x777777),
                        })
                        .width(800)
                        .padding(10),
                    button("Add")
                        .on_press(Message::NewItemInputSubmitted)
                        .style(|_, status| button::Style {
                            background: match status {
                                button::Status::Active => None,
                                button::Status::Hovered =>
                                    Some(Background::Color(color!(0x777777))),
                                button::Status::Pressed => None,
                                button::Status::Disabled =>
                                    Some(Background::Color(color!(0x777777))),
                            },
                            text_color: color!(0xFFFFFF),
                            border: Border {
                                color: color!(0xFFFFFF),
                                width: 1.0,
                                radius: border::Radius {
                                    top_left: 5.0,
                                    top_right: 5.0,
                                    bottom_right: 5.0,
                                    bottom_left: 5.0
                                }
                            },
                            shadow: Shadow {
                                ..Default::default()
                            }
                        })
                        .padding(10),
                ]
                .spacing(10),
                keyed_column(self.items.iter().map(|task| {
                    (
                        task.id,
                        mouse_area(
                            container(
                                checkbox(&task.content, task.done)
                                    .on_toggle(|_| Message::ItemToggled(task.id, task.done))
                                    .style(|_, status| checkbox::Style {
                                        background: Background::Color(Color::TRANSPARENT),
                                        icon_color: color!(0xFFFFFF),
                                        border: Border {
                                            color: color!(0xFFFFFF),
                                            width: 1.0,
                                            radius: border::Radius {
                                                top_left: 5.0,
                                                top_right: 5.0,
                                                bottom_right: 5.0,
                                                bottom_left: 5.0,
                                            },
                                        },
                                        text_color: match status {
                                            checkbox::Status::Active { is_checked } => {
                                                if is_checked {
                                                    Some(color!(0x777777))
                                                } else {
                                                    None
                                                }
                                            }
                                            checkbox::Status::Hovered { is_checked } => {
                                                if is_checked {
                                                    Some(color!(0x777777))
                                                } else {
                                                    None
                                                }
                                            }
                                            checkbox::Status::Disabled {
                                                is_checked: _is_checked,
                                            } => Some(color!(0x777777)),
                                        },
                                    }),
                            )
                            .style(|_| container::Style {
                                text_color: None,
                                background: None,
                                border: Border {
                                    color: color!(0xFFFFFF),
                                    width: 1.0,
                                    radius: border::Radius {
                                        top_left: 5.0,
                                        top_right: 5.0,
                                        bottom_right: 5.0,
                                        bottom_left: 5.0,
                                    },
                                },
                                shadow: Default::default(),
                            })
                            .width(860)
                            .padding(10),
                        )
                        .on_press(Message::ItemToggled(task.id, task.done))
                        .into(),
                    )
                }))
                .spacing(20)
            ]
            .spacing(30),
        )
        .center_x(Length::Fill)
        .into()
    }
}
