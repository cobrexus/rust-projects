use iced::alignment::{Horizontal, Vertical};
use iced::theme::{self, Theme};
use iced::widget::{
    button, column, container, row, text, text_input, Button, Container, TextInput,
};
use iced::{
    Alignment, Background, Border, Color, Element, Length, Padding, Sandbox, Settings, Shadow,
    Vector,
};

fn main() -> iced::Result {
    RustUI::run(Settings::default())
}

struct RustUI {
    theme: Theme,
    page: Page,
    login_field: LoginFields,
}

struct LoginFields {
    email: String,
    password: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Page {
    Login,
    Register,
}

#[derive(Debug, Clone)]
enum Message {
    ToggleTheme,
    LoginSubmit,
    Router(Route),
    LoginFieldsChanged(String, String),
}

#[derive(Debug, Clone)]
enum Route {
    Login,
    Register,
}

impl Sandbox for RustUI {
    type Message = Message;

    fn new() -> Self {
        Self {
            theme: Theme::Dark,
            page: Page::Login,
            login_field: LoginFields {
                email: String::new(),
                password: String::new(),
            },
        }
    }

    fn title(&self) -> String {
        match self.page {
            Page::Login => String::from("Login - Iced Modern GUI"),
            Page::Register => String::from("Register - Iced Modern GUI"),
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleTheme => {
                self.theme = if self.theme == Theme::Light {
                    Theme::Dark
                } else {
                    Theme::Light
                }
            }
            Message::LoginFieldsChanged(email, password) => {
                self.login_field.email = email;
                self.login_field.password = password;
            }
            Message::LoginSubmit => (),
            Message::Router(route) => match route {
                Route::Login => {
                    self.page = Page::Login;
                }
                Route::Register => {
                    self.page = Page::Register;
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self.page {
            Page::Login => login_page(&self.login_field),
            Page::Register => register_page(&self.login_field),
        };

        let wrapper = column![
            content,
            match self.page {
                Page::Login => footer(
                    button("Register")
                        .on_press(Message::Router(Route::Register))
                        .style(theme::Button::Custom(Box::new(ButtonStyle::FooterButton))),
                ),
                Page::Register => footer(
                    button("Login")
                        .on_press(Message::Router(Route::Login))
                        .style(theme::Button::Custom(Box::new(ButtonStyle::FooterButton))),
                ),
            }
        ]
        .spacing(50)
        .width(Length::Fill)
        .align_items(Alignment::Center);

        container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from(20))
            .center_x()
            .center_y()
            .style(theme::Container::Custom(Box::new(ContainerStyle)))
            .into()
    }
}

fn footer(route_btn: Button<Message>) -> Container<Message> {
    let footer = row![
        button("Toggle Theme")
            .on_press(Message::ToggleTheme)
            .style(theme::Button::Custom(Box::new(ButtonStyle::FooterButton))),
        text("•"),
        route_btn,
    ]
    .align_items(Alignment::Center)
    .spacing(10);

    container(footer).center_x().center_y()
}

fn login_page(login_field: &LoginFields) -> Container<Message> {
    let column = column![
        text("Login").size(30),
        input_field("Email Address", &login_field.email)
            .on_input(|email| Message::LoginFieldsChanged(email, login_field.password.clone())),
        input_field("Password", &login_field.password).on_input(|password| {
            Message::LoginFieldsChanged(login_field.email.clone(), password)
        }),
        submit_btn("Login", Message::LoginSubmit),
    ]
    .padding(Padding::from([50, 20]))
    .align_items(Alignment::Center)
    .spacing(40);

    container(column)
        .padding(Padding::from(20))
        .style(theme::Container::Custom(Box::new(ContainerStyle)))
        .into()
}

fn register_page(login_field: &LoginFields) -> Container<Message> {
    let column = column![
        text("Register").size(30),
        input_field("Email Address", &login_field.email)
            .on_input(|email| Message::LoginFieldsChanged(email, login_field.password.clone())),
        input_field("Password", &login_field.password).on_input(|password| {
            Message::LoginFieldsChanged(login_field.email.clone(), password)
        }),
        submit_btn("Register", Message::LoginSubmit),
    ]
    .padding(Padding::from([50, 20]))
    .align_items(Alignment::Center)
    .spacing(40);

    container(column)
        .padding(Padding::from(20))
        .style(theme::Container::Custom(Box::new(ContainerStyle)))
        .into()
}

fn input_field(placeholder: &str, value: &str) -> TextInput<'static, Message> {
    text_input(placeholder, value)
        .width(Length::Fixed(500.0))
        .padding(Padding::from(10))
        .line_height(text::LineHeight::Relative(1.75))
}

fn submit_btn(name: &str, event: Message) -> Button<Message> {
    button(
        text(name)
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center)
            .size(21),
    )
    .on_press(event)
    .width(Length::Fixed(500.0))
    .height(Length::Fixed(45.0))
    .style(theme::Button::Custom(Box::new(ButtonStyle::Standard)))
}

enum ButtonStyle {
    Standard,
    FooterButton,
}

impl button::StyleSheet for ButtonStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(match self {
                Self::Standard => Color::from_rgb(0.059, 0.463, 0.702),
                Self::FooterButton => Color::default(),
            })),
            border: match self {
                Self::Standard => Border::with_radius(5),
                Self::FooterButton => Border::default(),
            },
            shadow: Default::default(),
            text_color: {
                if style == &Theme::Light {
                    match self {
                        Self::Standard => Color::WHITE,
                        Self::FooterButton => Color::BLACK,
                    }
                } else {
                    match self {
                        Self::Standard => Color::WHITE,
                        Self::FooterButton => Color::WHITE,
                    }
                }
            },
            shadow_offset: Default::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(match self {
                Self::Standard => Color::from_rgb(0.059, 0.463, 0.702),
                Self::FooterButton => Color::default(),
            })),
            border: match self {
                Self::Standard => Border::with_radius(5),
                Self::FooterButton => Border::default(),
            },
            text_color: {
                if style == &Theme::Light {
                    match self {
                        Self::Standard => Color::WHITE,
                        Self::FooterButton => Color::BLACK,
                    }
                } else {
                    match self {
                        Self::Standard => Color::WHITE,
                        Self::FooterButton => Color::WHITE,
                    }
                }
            },
            shadow_offset: Default::default(),
            shadow: match self {
                Self::Standard => Shadow {
                    color: Color::BLACK,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 20.0,
                },
                Self::FooterButton => Shadow::default(),
            },
        }
    }
}

struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Default::default(),
            border: Border::with_radius(5),
            background: None,
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 40.0,
            },
        }
    }
}
