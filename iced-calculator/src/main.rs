use iced::theme;
use iced::widget::{button, column, container, row, text};
use iced::{Element, Sandbox, Settings};

fn main() -> iced::Result {
    Calculator::run(Settings::default())
}

struct Calculator {
    answer: i32,
    current_op: Option<Op>,
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
            current_op: None,
        }
    }

    fn title(&self) -> String {
        String::from("Calculator")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::OpBtnPressed(op) => {
                self.current_op = Some(op);
            }
            Message::NumBtnPressed(n) => match &self.current_op {
                None => self.answer = format!("{}{}", self.answer, n).parse::<i32>().unwrap(),
                Some(op) => match op {
                    Op::Add => {
                        self.answer += n as i32;
                        self.current_op = None;
                    }
                    Op::Sub => {
                        self.answer -= n as i32;
                        self.current_op = None;
                    }
                    Op::Mul => {
                        self.answer *= n as i32;
                        self.current_op = None;
                    }
                    Op::Div => {
                        self.answer /= n as i32;
                        self.current_op = None;
                    }
                    Op::Clear => {
                        self.answer = 0;
                        self.current_op = None;
                    }
                    Op::Equal => todo!(),
                },
            },
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        fn btn<'a>(txt: &'a str, on_press: Message, style: theme::Button) -> Element<'a, Message> {
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

        fn answer<'a>(res: i32) -> Element<'a, Message> {
            container(text(res))
                .style(theme::Container::Box)
                .padding(5)
                .into()
        }

        row![
            column![
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
            .spacing(5),
            answer(self.answer)
        ]
        .into()
    }
}
