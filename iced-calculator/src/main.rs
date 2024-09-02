use iced::{
    theme,
    widget::{button, column, container, row, text},
    Element, Length, Sandbox, Settings,
};

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
        fn btn(contents: &str, op: Option<Op>) -> Element<Message> {
            match contents.parse::<u8>() {
                Ok(n) => button(
                    container(text(n))
                        .width(30)
                        .height(30)
                        .center_x()
                        .center_y(),
                )
                .style(theme::Button::Primary)
                .on_press(Message::NumBtnPressed(n))
                .into(),
                Err(_) => button(
                    container(text(contents))
                        .width(30)
                        .height(30)
                        .center_x()
                        .center_y(),
                )
                .style(theme::Button::Secondary)
                .on_press(Message::OpBtnPressed(
                    op.expect("op btn should have op set"),
                ))
                .into(),
            }
        }

        container(column![
            column![text(self.answer).size(20), text(self.temp),].padding(10),
            row![column![
                row![btn("7", None), btn("8", None), btn("9", None),].spacing(5),
                row![btn("4", None), btn("5", None), btn("6", None),].spacing(5),
                row![btn("1", None), btn("2", None), btn("3", None),].spacing(5),
                row![
                    btn("+", Some(Op::Add),),
                    btn("-", Some(Op::Sub),),
                    btn("C", Some(Op::Clear),),
                ]
                .spacing(5),
                row![
                    btn("×", Some(Op::Mul),),
                    btn("÷", Some(Op::Div),),
                    btn("=", Some(Op::Equal),),
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
