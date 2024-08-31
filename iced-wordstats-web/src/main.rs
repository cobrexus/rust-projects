use iced::border::Radius;
use iced::widget::{column, container, row, text, text_editor};
use iced::{executor, theme};
use iced::{Application, Background, Border, Color, Command, Element, Length, Settings, Theme};

pub fn main() -> iced::Result {
    Wordstats::run(Settings::default())
}

#[derive(Default)]
struct Wordstats {
    content: text_editor::Content,
    char_count: usize,
    word_count: usize,
    sentence_count: usize,
    paragraph_count: usize,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Application for Wordstats {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Wordstats, Command<Message>) {
        (
            Wordstats {
                content: text_editor::Content::new(),
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Wordstats")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
                self.char_count = self.content.text().len();
                self.word_count = self
                    .content
                    .text()
                    .split_whitespace()
                    .filter(|x| *x != "")
                    .collect::<Vec<_>>()
                    .len();
                self.sentence_count = self
                    .content
                    .text()
                    .split(&['.', '!', '?'][..])
                    .filter(|x| *x != "")
                    .collect::<Vec<_>>()
                    .len();
                self.paragraph_count = self
                    .content
                    .text()
                    .split("\n")
                    .filter(|x| *x != "")
                    .collect::<Vec<_>>()
                    .len();

                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                row![
                    text(format!("{} characters", self.char_count)),
                    text("•"),
                    text(format!("{} words", self.word_count)),
                    text("•"),
                    text(format!("{} sentences", self.sentence_count)),
                    text("•"),
                    text(format!("{} paragraphs", self.paragraph_count)),
                ]
                .padding(10)
                .spacing(20),
                custom_text_editor(&self.content)
            ]
            .width(1000),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

#[derive(Default)]
struct TextEditorStyle;

fn custom_text_editor(content: &text_editor::Content) -> Element<Message> {
    text_editor(content)
        .on_action(Message::Edit)
        .height(500)
        .padding(20)
        .style(theme::TextEditor::Custom(Box::new(TextEditorStyle)))
        .into()
}

impl text_editor::StyleSheet for TextEditorStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                color: Color::WHITE,
                width: 1.0,
                radius: Radius::from(10.0),
            },
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                color: Color::WHITE,
                width: 1.0,
                radius: Radius::from(10.0),
            },
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.666, 0.666, 0.666)
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        Color::WHITE
    }

    fn disabled_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.666, 0.666, 0.666)
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.666, 0.666, 0.666)
    }

    fn disabled(&self, _style: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                color: Color::from_rgb(0.666, 0.666, 0.666),
                width: 3.0,
                radius: Radius::from(10.0),
            },
        }
    }
}
