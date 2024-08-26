use iced::theme;
use iced::widget::{button, column, container, row, text};
use iced::Length;
use iced::{Element, Sandbox, Settings};

fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

struct Calculator {
    answer: i128,
    temp: i128,
    op: Option<Op>,
}

#[derive(Debug, Clone)]
enum Message {
    NumBtnPressed(u8),
    OpBtnPressed(Op),
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Clear,
    Equal,
}

impl Sandbox for Calculator {
    type Message = Message;

    fn new() -> Self {
        Self {
            answer: 0,
            temp: 0,
            op: None,
        }
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::NumBtnPressed(n) => {
                self.temp = format!("{}{}", self.temp, n)
                    .parse::<i128>()
                    .unwrap_or(i128::MAX);
            }
            Message::OpBtnPressed(Op::Add) => {
                if self.answer == 0 {
                    self.answer = self.temp;
                    self.temp = 0;
                }
                self.op = Some(Op::Add);
            }
            Message::OpBtnPressed(Op::Sub) => {
                if self.answer == 0 {
                    self.answer = self.temp;
                    self.temp = 0;
                }
                self.op = Some(Op::Sub);
            }
            Message::OpBtnPressed(Op::Mul) => {
                if self.answer == 0 {
                    self.answer = self.temp;
                    self.temp = 0;
                }
                self.op = Some(Op::Mul);
            }
            Message::OpBtnPressed(Op::Div) => {
                if self.answer == 0 {
                    self.answer = self.temp;
                    self.temp = 0;
                }
                self.op = Some(Op::Div);
            }
            Message::OpBtnPressed(Op::Clear) => {
                self.answer = 0;
                self.temp = 0;
            }
            Message::OpBtnPressed(Op::Equal) if self.op.is_some() => {
                match self.op.as_ref().unwrap() {
                    Op::Add => {
                        self.answer += self.temp;
                        self.temp = 0;
                    }
                    Op::Sub => {
                        self.answer -= self.temp;
                        self.temp = 0;
                    }
                    Op::Mul => {
                        self.answer *= self.temp;
                        self.temp = 0;
                    }
                    Op::Div => {
                        self.answer *= self.temp;
                        self.temp = 0;
                    }
                    _ => (),
                }
            }
            Message::OpBtnPressed(Op::Equal) => (), // no operation selected
        }
    }

    fn view(&self) -> Element<Message> {
        fn btn<'a>(txt: String, on_press: Message, style: theme::Button) -> Element<'a, Message> {
            button(
                container(text(txt))
                    .width(30)
                    .height(30)
                    .center_x()
                    .center_y(),
            )
            .style(style)
            .on_press(on_press)
            .into()
        }

        fn num_btn_row<'a>(n: u8) -> Element<'a, Message> {
            btn(
                n.to_string(),
                Message::NumBtnPressed(n),
                theme::Button::Primary,
            )
        }

        container(column![
            column![text(self.answer).size(20), text(self.temp),].padding(10),
            row![column![
                row![
                    btn("7", Message::NumBtnPressed(7), theme::Button::Primary),
                    btn("8", Message::NumBtnPressed(8), theme::Button::Primary),
                    btn("9", Message::NumBtnPressed(9), theme::Button::Primary),
                ]
                .spacing(5),
                row![
                    btn("4", Message::NumBtnPressed(4), theme::Button::Primary),
                    btn("5", Message::NumBtnPressed(5), theme::Button::Primary),
                    btn("6", Message::NumBtnPressed(6), theme::Button::Primary),
                ]
                .spacing(5),
                row![
                    btn("1", Message::NumBtnPressed(1), theme::Button::Primary),
                    btn("2", Message::NumBtnPressed(2), theme::Button::Primary),
                    btn("3", Message::NumBtnPressed(3), theme::Button::Primary),
                ]
                .spacing(5),
                row![
                    btn(
                        "+",
                        Message::OpBtnPressed(Op::Add),
                        theme::Button::Secondary
                    ),
                    btn(
                        "-",
                        Message::OpBtnPressed(Op::Sub),
                        theme::Button::Secondary
                    ),
                    btn(
                        "C",
                        Message::OpBtnPressed(Op::Clear),
                        theme::Button::Secondary
                    ),
                ]
                .spacing(5),
                row![
                    btn(
                        "×",
                        Message::OpBtnPressed(Op::Mul),
                        theme::Button::Secondary
                    ),
                    btn(
                        "÷",
                        Message::OpBtnPressed(Op::Div),
                        theme::Button::Secondary
                    ),
                    btn(
                        "=",
                        Message::OpBtnPressed(Op::Equal),
                        theme::Button::Secondary
                    ),
                ]
                .spacing(5),
            ]
            .spacing(5)]
        ])
        .width(Length::Fill)
        .center_x()
        .height(Length::Fill)
        .center_y()
        .into()
    }
}
