use iced::{
    color, theme,
    widget::{
        button, column, container, row, scrollable,
        scrollable::{Direction, Scrollbar},
        text,
    },
    Element, Length, Theme,
};

pub fn main() -> iced::Result {
    iced::application("Calculator - Iced", Calculator::update, Calculator::view)
        .theme(|_| {
            Theme::custom(
                "Iced Calculator Theme".to_string(),
                theme::Palette {
                    background: color!(0x000000),
                    text: color!(0xFFFFFF),
                    primary: color!(0xFFFFFF),
                    success: color!(0x88FF88),
                    danger: color!(0xFF8888),
                },
            )
        })
        .run()
}

#[derive(Default)]
struct Calculator {
    display_1: f64,
    display_2: f64,
    current_op: Op,
    editing_display_2: bool,
}

#[derive(Debug, Clone)]
enum Message {
    NumBtnPressed(Num),
    OpBtnPressed(Op),
}

#[derive(Debug, Clone)]
enum Num {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, Clone, PartialEq, Default)]
enum Op {
    #[default]
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Clear,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "−"),
            Op::Mul => write!(f, "×"),
            Op::Div => write!(f, "÷"),
            _ => unreachable!(),
        }
    }
}

impl Calculator {
    fn update(&mut self, message: Message) {
        match message {
            Message::OpBtnPressed(Op::Clear) => {
                self.display_1 = 0.0;
                self.display_2 = 0.0;
                self.editing_display_2 = false;
            }
            Message::OpBtnPressed(Op::Eq) => {
                self.editing_display_2 = false;

                match self.current_op {
                    Op::Add => {
                        self.display_1 += self.display_2;
                        self.display_2 = 0.0;
                    }
                    Op::Sub => {
                        self.display_1 -= self.display_2;
                        self.display_2 = 0.0;
                    }
                    Op::Mul => {
                        self.display_1 *= self.display_2;
                        self.display_2 = 0.0;
                    }
                    Op::Div => {
                        self.display_1 /= self.display_2;
                        self.display_2 = 0.0;
                    }
                    Op::Clear => unreachable!(),
                    Op::Eq => unreachable!(),
                }
            }
            Message::OpBtnPressed(op) => {
                self.current_op = op;
                self.editing_display_2 = true;
            }
            Message::NumBtnPressed(num) => {
                if self.editing_display_2 {
                    self.display_2 = format!("{}{}", self.display_2, num as i32).parse().unwrap();
                } else {
                    self.display_1 = format!("{}{}", self.display_1, num as i32).parse().unwrap();
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                scrollable(text(self.display_2).size(50))
                    .direction(Direction::Horizontal(Scrollbar::new()))
                    .width(210),
                row![
                    scrollable(text(self.display_1).size(50))
                        .direction(Direction::Horizontal(Scrollbar::new()))
                        .width(210),
                    text(self.current_op.to_string()).size(50)
                ],
                row![
                    button(text("C").center())
                        .on_press(Message::OpBtnPressed(Op::Clear))
                        .width(160)
                        .height(50),
                    button(text("+").center())
                        .width(50)
                        .on_press(Message::OpBtnPressed(Op::Add))
                        .height(50)
                ]
                .spacing(5),
                row![
                    button(text("1").center())
                        .on_press(Message::NumBtnPressed(Num::One))
                        .width(50)
                        .height(50),
                    button(text("2").center())
                        .width(50)
                        .on_press(Message::NumBtnPressed(Num::Two))
                        .height(50),
                    button(text("3").center())
                        .width(50)
                        .on_press(Message::NumBtnPressed(Num::Three))
                        .height(50),
                    button(text("−").center())
                        .on_press(Message::OpBtnPressed(Op::Sub))
                        .width(50)
                        .height(50)
                ]
                .spacing(5),
                row![
                    button(text("4").center())
                        .on_press(Message::NumBtnPressed(Num::Four))
                        .width(50)
                        .height(50),
                    button(text("5").center())
                        .on_press(Message::NumBtnPressed(Num::Five))
                        .width(50)
                        .height(50),
                    button(text("6").center())
                        .on_press(Message::NumBtnPressed(Num::Six))
                        .width(50)
                        .height(50),
                    button(text("×").center())
                        .width(50)
                        .on_press(Message::OpBtnPressed(Op::Mul))
                        .height(50)
                ]
                .spacing(5),
                row![
                    button(text("7").center())
                        .width(50)
                        .on_press(Message::NumBtnPressed(Num::Seven))
                        .height(50),
                    button(text("8").center())
                        .width(50)
                        .on_press(Message::NumBtnPressed(Num::Eight))
                        .height(50),
                    button(text("9").center())
                        .width(50)
                        .on_press(Message::NumBtnPressed(Num::Nine))
                        .height(50),
                    button(text("÷").center())
                        .on_press(Message::OpBtnPressed(Op::Div))
                        .width(50)
                        .height(50)
                ]
                .spacing(5),
                row![
                    button(text("0").center())
                        .width(160)
                        .on_press(Message::NumBtnPressed(Num::Zero))
                        .height(50),
                    button(text("=").center())
                        .on_press(Message::OpBtnPressed(Op::Eq))
                        .width(50)
                        .height(50)
                ]
                .spacing(5),
            ]
            .spacing(5),
        )
        .center(Length::Fill)
        .into()
    }
}
