use iced::{
    application, border, color, daemon,
    widget::{
        button, checkbox, column, container, horizontal_space, keyed_column, mouse_area, row,
        text_input, vertical_space,
    },
    Background, Border, Color, Element, Font, Length, Pixels, Result, Settings, Shadow, Theme,
};
use std::collections::HashMap;

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
    items: HashMap<usize, Item>,
    next_item_id: usize,
}

#[derive(Debug, Clone)]
enum Message {
    NewItemInputValueChanged(String),
    NewItemInputSubmitted,
    ItemToggled(usize),
    DeleteItem(usize),
    EditItem(usize),
    ItemEdited(usize, String),
    SaveItem(usize),
}

#[derive(PartialEq)]
struct Item {
    id: usize,
    content: String,
    done: bool,
    editing: bool,
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
                self.items.insert(
                    self.next_item_id,
                    Item {
                        id: self.next_item_id,
                        content: self.new_item_input_value.clone(),
                        done: false,
                        editing: false,
                    },
                );
                self.new_item_input_value.clear();
                self.next_item_id += 1;
            }
            Message::ItemToggled(id) => {
                self.items
                    .entry(id)
                    .and_modify(|item| item.done = !item.done);
            }
            Message::DeleteItem(id) => {
                self.items.remove(&id);
            }
            Message::EditItem(id) => {
                self.items.entry(id).and_modify(|item| item.editing = true);
            }
            Message::ItemEdited(id, s) => {
                self.items.entry(id).and_modify(|item| item.content = s);
            }
            Message::SaveItem(id) => {
                self.items.entry(id).and_modify(|item| item.editing = false);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let new_item_text_input = text_input("Add an item...", &self.new_item_input_value)
            .on_input(Message::NewItemInputValueChanged)
            .on_submit_maybe(if self.new_item_input_value.is_empty() {
                None
            } else {
                Some(Message::NewItemInputSubmitted)
            })
            .style(|_, status| text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border: Border {
                    color: match status {
                        text_input::Status::Active => color!(0x777777),
                        text_input::Status::Hovered => color!(0x888888),
                        text_input::Status::Focused => color!(0xAAAAAA),
                        text_input::Status::Disabled => color!(0x777777),
                    },
                    width: 1.0,
                    radius: border::Radius {
                        top_left: 20.0,
                        top_right: 20.0,
                        bottom_right: 20.0,
                        bottom_left: 20.0,
                    },
                },
                icon: color!(0xFFFFFF),
                placeholder: color!(0x777777),
                value: color!(0xFFFFFF),
                selection: color!(0x777777),
            })
            .width(800)
            .padding(20);

        let new_item_btn = button("Add")
            .on_press_maybe(if self.new_item_input_value.is_empty() {
                None
            } else {
                Some(Message::NewItemInputSubmitted)
            })
            .style(|_: &Theme, status| button::Style {
                background: match status {
                    button::Status::Active => None,
                    button::Status::Hovered => Some(Background::Color(color!(0x111111))),
                    button::Status::Pressed => None,
                    button::Status::Disabled => None,
                },
                text_color: match status {
                    button::Status::Active => color!(0xFFFFFF),
                    button::Status::Hovered => color!(0xFFFFFF),
                    button::Status::Pressed => color!(0xFFFFFF),
                    button::Status::Disabled => color!(0x777777),
                },
                border: Border {
                    color: match status {
                        button::Status::Active => color!(0x777777),
                        button::Status::Hovered => color!(0x888888),
                        button::Status::Pressed => color!(0xAAAAAA),
                        button::Status::Disabled => color!(0x777777),
                    },
                    width: 1.0,
                    radius: border::Radius {
                        top_left: 20.0,
                        top_right: 20.0,
                        bottom_right: 20.0,
                        bottom_left: 20.0,
                    },
                },
                shadow: Shadow {
                    ..Default::default()
                },
            })
            .padding(20);

        fn item_container<'a>(
            id: usize,
            content: &'a str,
            done: bool,
            editing: bool,
        ) -> Element<'a, Message> {
            mouse_area(
                container(row![
                    if editing {
                        container(item_text_input(content, id))
                    } else {
                        container(item_checkbox(id, content, done))
                    },
                    horizontal_space(),
                    item_edit_btn(id, editing),
                    item_delete_btn(id)
                ])
                .style(|_| container::Style {
                    text_color: None,
                    background: None,
                    border: Border {
                        color: color!(0x777777),
                        width: 1.0,
                        radius: border::Radius {
                            top_left: 20.0,
                            top_right: 20.0,
                            bottom_right: 20.0,
                            bottom_left: 20.0,
                        },
                    },
                    shadow: Default::default(),
                })
                .width(885)
                .padding(20),
            )
            .on_press(Message::ItemToggled(id))
            .into()
        }

        fn item_text_input(content: &str, id: usize) -> Element<Message> {
            text_input(content, content)
                .on_input(move |s| Message::ItemEdited(id, s))
                .style(|_, status| text_input::Style {
                    background: Background::Color(Color::TRANSPARENT),
                    border: Border {
                        color: match status {
                            text_input::Status::Active => color!(0x777777),
                            text_input::Status::Hovered => color!(0x888888),
                            text_input::Status::Focused => color!(0xAAAAAA),
                            text_input::Status::Disabled => color!(0x777777),
                        },
                        width: 1.0,
                        radius: border::Radius {
                            top_left: 20.0,
                            top_right: 20.0,
                            bottom_right: 20.0,
                            bottom_left: 20.0,
                        },
                    },
                    icon: color!(0xFFFFFF),
                    placeholder: color!(0x777777),
                    value: color!(0xFFFFFF),
                    selection: color!(0x777777),
                })
                .padding(20)
                .into()
        }

        fn item_checkbox(id: usize, content: &str, done: bool) -> Element<Message> {
            checkbox(content, done)
                .on_toggle(move |_| Message::ItemToggled(id))
                .style(|_, status| checkbox::Style {
                    background: Background::Color(Color::TRANSPARENT),
                    icon_color: color!(0x777777),
                    border: Border {
                        color: color!(0x777777),
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
                })
                .into()
        }

        fn item_edit_btn(id: usize, editing: bool) -> Element<'static, Message> {
            button(if editing { "Save" } else { "Edit" })
                .on_press(if editing {
                    Message::SaveItem(id)
                } else {
                    Message::EditItem(id)
                })
                .style(|_, status| button::Style {
                    background: match status {
                        button::Status::Active => None,
                        button::Status::Hovered => Some(Background::Color(color!(0x111111))),
                        button::Status::Pressed => None,
                        button::Status::Disabled => None,
                    },
                    text_color: match status {
                        button::Status::Active => color!(0xFFFFFF),
                        button::Status::Hovered => color!(0xFFFFFF),
                        button::Status::Pressed => color!(0xFFFFFF),
                        button::Status::Disabled => color!(0x777777),
                    },
                    border: Border {
                        color: color!(0x777777),
                        width: 1.0,
                        radius: border::Radius {
                            top_left: 20.0,
                            top_right: 20.0,
                            bottom_right: 20.0,
                            bottom_left: 20.0,
                        },
                    },
                    shadow: Shadow {
                        ..Default::default()
                    },
                })
                .into()
        }

        fn item_delete_btn(id: usize) -> Element<'static, Message> {
            button("Delete")
                .on_press(Message::DeleteItem(id))
                .style(|_, status| button::Style {
                    background: match status {
                        button::Status::Active => None,
                        button::Status::Hovered => Some(Background::Color(color!(0x111111))),
                        button::Status::Pressed => None,
                        button::Status::Disabled => None,
                    },
                    text_color: match status {
                        button::Status::Active => color!(0xFFFFFF),
                        button::Status::Hovered => color!(0xFFFFFF),
                        button::Status::Pressed => color!(0xFFFFFF),
                        button::Status::Disabled => color!(0x777777),
                    },
                    border: Border {
                        color: color!(0x777777),
                        width: 1.0,
                        radius: border::Radius {
                            top_left: 20.0,
                            top_right: 20.0,
                            bottom_right: 20.0,
                            bottom_left: 20.0,
                        },
                    },
                    shadow: Shadow {
                        ..Default::default()
                    },
                })
                .into()
        }

        container(
            column![
                vertical_space().height(30),
                row![new_item_text_input, new_item_btn].spacing(20),
                keyed_column(self.items.iter().map(|item| (
                    item.1.id,
                    item_container(item.1.id, &item.1.content, item.1.done, item.1.editing)
                )))
                .spacing(20)
            ]
            .spacing(30),
        )
        .center_x(Length::Fill)
        .into()
    }
}
