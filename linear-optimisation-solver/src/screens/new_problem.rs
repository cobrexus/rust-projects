use std::fmt::Display;

use chrono::{offset::Local, DateTime};
use iced::{
    widget::{
        button, column, container, pick_list, row, scrollable, text, text_editor, text_input,
    },
    Alignment, Element, Length,
};

use crate::{Message, Screen};

#[derive(Default)]
pub struct NewProblem {
    title: String,
    description: text_editor::Content,

    problem_type: ProblemType,
    objective_function_variable_0_name: String,

    objective_function_variable_1_sign: Sign,
    objective_function_variable_1_magnitude: String,
    objective_function_variable_2_sign: Sign,
    objective_function_variable_2_magnitude: String,
    objective_function_variable_3_sign: Sign,
    objective_function_variable_3_magnitude: String,

    constraint_1_variable_1_sign: Sign,
    constraint_1_variable_1_magnitude: String,
    constraint_1_variable_2_sign: Sign,
    constraint_1_variable_2_magnitude: String,
    constraint_1_variable_3_sign: Sign,
    constraint_1_variable_3_magnitude: String,
    constraint_1_inequality: Inequality,
    constraint_1_variable_4_sign: Sign,
    constraint_1_variable_4_magnitude: String,

    constraint_2_variable_1_sign: Sign,
    constraint_2_variable_1_magnitude: String,
    constraint_2_variable_2_sign: Sign,
    constraint_2_variable_2_magnitude: String,
    constraint_2_variable_3_sign: Sign,
    constraint_2_variable_3_magnitude: String,
    constraint_2_inequality: Inequality,
    constraint_2_variable_4_sign: Sign,
    constraint_2_variable_4_magnitude: String,

    constraint_3_variable_1_sign: Sign,
    constraint_3_variable_1_magnitude: String,
    constraint_3_variable_2_sign: Sign,
    constraint_3_variable_2_magnitude: String,
    constraint_3_variable_3_sign: Sign,
    constraint_3_variable_3_magnitude: String,
    constraint_3_inequality: Inequality,
    constraint_3_variable_4_sign: Sign,
    constraint_3_variable_4_magnitude: String,

    error: Error,
}

#[derive(Debug, Clone)]
pub enum NewProblemMessage {
    BackButtonPressed,
    TitleChanged(String),
    DescriptionChanged(text_editor::Action),

    ProblemTypeChanged(ProblemType),
    ObjectiveFunctionVariable0NameChanged(String),
    ObjectiveFunctionVariable1SignChanged(Sign),
    ObjectiveFunctionVariable1MagnitudeChanged(String),
    ObjectiveFunctionVariable2SignChanged(Sign),
    ObjectiveFunctionVariable2MagnitudeChanged(String),
    ObjectiveFunctionVariable3SignChanged(Sign),
    ObjectiveFunctionVariable3MagnitudeChanged(String),

    Constraint1Variable1SignChanged(Sign),
    Constraint1Variable1MagnitudeChanged(String),
    Constraint1Variable2SignChanged(Sign),
    Constraint1Variable2MagnitudeChanged(String),
    Constraint1Variable3SignChanged(Sign),
    Constraint1Variable3MagnitudeChanged(String),
    Constraint1InequalityChanged(Inequality),
    Constraint1Variable4SignChanged(Sign),
    Constraint1Variable4MagnitudeChanged(String),

    Constraint2Variable1SignChanged(Sign),
    Constraint2Variable1MagnitudeChanged(String),
    Constraint2Variable2SignChanged(Sign),
    Constraint2Variable2MagnitudeChanged(String),
    Constraint2Variable3SignChanged(Sign),
    Constraint2Variable3MagnitudeChanged(String),
    Constraint2InequalityChanged(Inequality),
    Constraint2Variable4SignChanged(Sign),
    Constraint2Variable4MagnitudeChanged(String),

    Constraint3Variable1SignChanged(Sign),
    Constraint3Variable1MagnitudeChanged(String),
    Constraint3Variable2SignChanged(Sign),
    Constraint3Variable2MagnitudeChanged(String),
    Constraint3Variable3SignChanged(Sign),
    Constraint3Variable3MagnitudeChanged(String),
    Constraint3InequalityChanged(Inequality),
    Constraint3Variable4SignChanged(Sign),
    Constraint3Variable4MagnitudeChanged(String),

    SolveButtonPressed,
}

#[derive(Default)]
enum Error {
    #[default]
    None,
    TitleEmpty,
    MagnitudeInvalid,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::None => write!(f, ""),
            Error::TitleEmpty => write!(f, "Title cannot be empty"),
            Error::MagnitudeInvalid => write!(f, "Variable values must be numbers"),
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub enum ProblemType {
    #[default]
    Maximise,
    Minimise,
}

impl Display for ProblemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProblemType::Maximise => write!(f, "Maximise"),
            ProblemType::Minimise => write!(f, "Minimise"),
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub enum Sign {
    #[default]
    Add,
    Sub,
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sign::Add => write!(f, "+"),
            Sign::Sub => write!(f, "-"),
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub enum Inequality {
    #[default]
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

impl Display for Inequality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inequality::GreaterThan => write!(f, ">"),
            Inequality::GreaterThanOrEqual => write!(f, "≥"),
            Inequality::LessThan => write!(f, "<"),
            Inequality::LessThanOrEqual => write!(f, "≤"),
        }
    }
}

impl NewProblem {
    pub fn update(&mut self, message: NewProblemMessage) -> Option<Screen> {
        match message {
            NewProblemMessage::BackButtonPressed => Some(Screen::AllProblems),
            NewProblemMessage::TitleChanged(s) => {
                self.title = s;
                None
            }
            NewProblemMessage::DescriptionChanged(a) => {
                self.description.perform(a);
                None
            }
            NewProblemMessage::ProblemTypeChanged(p) => {
                self.problem_type = p;
                None
            }
            NewProblemMessage::ObjectiveFunctionVariable0NameChanged(s) => {
                self.objective_function_variable_0_name = s;
                None
            }
            NewProblemMessage::ObjectiveFunctionVariable1SignChanged(s) => {
                self.objective_function_variable_1_sign = s;
                None
            }
            NewProblemMessage::ObjectiveFunctionVariable1MagnitudeChanged(s) => {
                self.objective_function_variable_1_magnitude = s;
                None
            }
            NewProblemMessage::ObjectiveFunctionVariable2SignChanged(s) => {
                self.objective_function_variable_2_sign = s;
                None
            }
            NewProblemMessage::ObjectiveFunctionVariable2MagnitudeChanged(s) => {
                self.objective_function_variable_2_magnitude = s;
                None
            }
            NewProblemMessage::ObjectiveFunctionVariable3SignChanged(s) => {
                self.objective_function_variable_3_sign = s;
                None
            }
            NewProblemMessage::ObjectiveFunctionVariable3MagnitudeChanged(s) => {
                self.objective_function_variable_3_magnitude = s;
                None
            }
            NewProblemMessage::Constraint1Variable1SignChanged(s) => {
                self.constraint_1_variable_1_sign = s;
                None
            }
            NewProblemMessage::Constraint1Variable1MagnitudeChanged(s) => {
                self.constraint_1_variable_1_magnitude = s;
                None
            }
            NewProblemMessage::Constraint1Variable2SignChanged(s) => {
                self.constraint_1_variable_2_sign = s;
                None
            }
            NewProblemMessage::Constraint1Variable2MagnitudeChanged(s) => {
                self.constraint_1_variable_2_magnitude = s;
                None
            }
            NewProblemMessage::Constraint1Variable3SignChanged(s) => {
                self.constraint_1_variable_3_sign = s;
                None
            }
            NewProblemMessage::Constraint1Variable3MagnitudeChanged(s) => {
                self.constraint_1_variable_3_magnitude = s;
                None
            }
            NewProblemMessage::Constraint1InequalityChanged(i) => {
                self.constraint_1_inequality = i;
                None
            }
            NewProblemMessage::Constraint1Variable4SignChanged(s) => {
                self.constraint_1_variable_4_sign = s;
                None
            }
            NewProblemMessage::Constraint1Variable4MagnitudeChanged(s) => {
                self.constraint_1_variable_4_magnitude = s;
                None
            }
            NewProblemMessage::Constraint2Variable1SignChanged(s) => {
                self.constraint_2_variable_1_sign = s;
                None
            }
            NewProblemMessage::Constraint2Variable1MagnitudeChanged(s) => {
                self.constraint_2_variable_1_magnitude = s;
                None
            }
            NewProblemMessage::Constraint2Variable2SignChanged(s) => {
                self.constraint_2_variable_2_sign = s;
                None
            }
            NewProblemMessage::Constraint2Variable2MagnitudeChanged(s) => {
                self.constraint_2_variable_2_magnitude = s;
                None
            }
            NewProblemMessage::Constraint2Variable3SignChanged(s) => {
                self.constraint_2_variable_3_sign = s;
                None
            }
            NewProblemMessage::Constraint2Variable3MagnitudeChanged(s) => {
                self.constraint_2_variable_3_magnitude = s;
                None
            }
            NewProblemMessage::Constraint2InequalityChanged(i) => {
                self.constraint_2_inequality = i;
                None
            }
            NewProblemMessage::Constraint2Variable4SignChanged(s) => {
                self.constraint_2_variable_4_sign = s;
                None
            }
            NewProblemMessage::Constraint2Variable4MagnitudeChanged(s) => {
                self.constraint_2_variable_4_magnitude = s;
                None
            }
            NewProblemMessage::Constraint3Variable1SignChanged(s) => {
                self.constraint_3_variable_1_sign = s;
                None
            }
            NewProblemMessage::Constraint3Variable1MagnitudeChanged(s) => {
                self.constraint_3_variable_1_magnitude = s;
                None
            }
            NewProblemMessage::Constraint3Variable2SignChanged(s) => {
                self.constraint_3_variable_2_sign = s;
                None
            }
            NewProblemMessage::Constraint3Variable2MagnitudeChanged(s) => {
                self.constraint_3_variable_2_magnitude = s;
                None
            }
            NewProblemMessage::Constraint3Variable3SignChanged(s) => {
                self.constraint_3_variable_3_sign = s;
                None
            }
            NewProblemMessage::Constraint3Variable3MagnitudeChanged(s) => {
                self.constraint_3_variable_3_magnitude = s;
                None
            }
            NewProblemMessage::Constraint3InequalityChanged(i) => {
                self.constraint_3_inequality = i;
                None
            }
            NewProblemMessage::Constraint3Variable4SignChanged(s) => {
                self.constraint_3_variable_4_sign = s;
                None
            }
            NewProblemMessage::Constraint3Variable4MagnitudeChanged(s) => {
                self.constraint_3_variable_4_magnitude = s;
                None
            }
            NewProblemMessage::SolveButtonPressed => {
                if self.title.trim().is_empty() {
                    self.error = Error::TitleEmpty;
                }

                enum Variable {
                    P,
                    X,
                    Y,
                    Z,
                    R,
                    S,
                    T,
                }

                const NUM_COLS: usize = 6;
                const NUM_ROWS: usize = 3;

                let tableau = [[0.0; NUM_COLS]; NUM_ROWS];

                let basic_variables = [Variable::R, Variable::S, Variable::T, Variable::P];

                let column_variables = [
                    Variable::X,
                    Variable::Y,
                    Variable::Z,
                    Variable::R,
                    Variable::S,
                    Variable::T,
                ];

                if self.problem_type == ProblemType::Maximise {
                    while tableau
                        .last()
                        .unwrap()
                        .iter()
                        .copied()
                        .filter(|&x| x <= 0.0)
                        .next()
                        .is_some()
                    {
                        let mut pivot_column: (usize, f64) = (0, 0.0);

                        for el in tableau.last().unwrap()[0..NUM_COLS - 1]
                            .iter()
                            .copied()
                            .enumerate()
                        {
                            if el.1 < pivot_column.1 {
                                pivot_column = el;
                            }
                        }

                        let mut theta_values = [0.0; NUM_ROWS];

                        for row in tableau.iter().copied().enumerate() {
                            theta_values[row.0] = row.1.last().unwrap() / row.1[pivot_column.0];
                        }

                        let mut theta: (usize, f64) = (0, 0.0);

                        for th in theta_values.iter().copied().enumerate() {
                            if th.1 >= 0.0 && th.1 <= theta.1 {
                                theta = th;
                            }
                        }

                        let pivot = (pivot_column.0, theta.0);
                    }
                } else {
                }

                None
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        scrollable(
            container(
                column![
                    button(text("Back").center())
                        .on_press(Message::NewProblemMessage(
                            NewProblemMessage::BackButtonPressed
                        ))
                        .width(700),
                    text("Title"),
                    text_input("", &self.title)
                        .on_input(|s| {
                            Message::NewProblemMessage(NewProblemMessage::TitleChanged(s))
                        })
                        .width(700),
                    text("Description"),
                    text_editor(&self.description)
                        .on_action(|s| {
                            Message::NewProblemMessage(NewProblemMessage::DescriptionChanged(s))
                        })
                        .width(700)
                        .height(75),
                    row![
                        pick_list(
                            [ProblemType::Maximise, ProblemType::Minimise],
                            Some(&self.problem_type),
                            |p| Message::NewProblemMessage(NewProblemMessage::ProblemTypeChanged(
                                p
                            ))
                        )
                        .width(100),
                        text_input("", &self.objective_function_variable_0_name)
                            .on_input(|n| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::ObjectiveFunctionVariable0NameChanged(n),
                                )
                            })
                            .width(65),
                        text("="),
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.objective_function_variable_1_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::ObjectiveFunctionVariable1SignChanged(s)
                            )
                        )
                        .width(65),
                        text_input("", &self.objective_function_variable_1_magnitude)
                            .on_input(|m| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::ObjectiveFunctionVariable1MagnitudeChanged(
                                        m,
                                    ),
                                )
                            })
                            .width(65),
                        text("x"),
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.objective_function_variable_2_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::ObjectiveFunctionVariable2SignChanged(s)
                            )
                        )
                        .width(65),
                        text_input("", &self.objective_function_variable_2_magnitude)
                            .on_input(|m| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::ObjectiveFunctionVariable2MagnitudeChanged(
                                        m,
                                    ),
                                )
                            })
                            .width(65),
                        text("y"),
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.objective_function_variable_3_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::ObjectiveFunctionVariable3SignChanged(s)
                            )
                        )
                        .width(65),
                        text_input("", &self.objective_function_variable_3_magnitude)
                            .on_input(|m| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::ObjectiveFunctionVariable3MagnitudeChanged(
                                        m,
                                    ),
                                )
                            })
                            .width(65),
                        text("z"),
                    ]
                    .align_y(Alignment::Center)
                    .spacing(10),
                    text("Subject to:"),
                    row![
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.constraint_1_variable_1_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::Constraint1Variable1SignChanged(s)
                            )
                        )
                        .width(60),
                        text_input("", &self.constraint_1_variable_1_magnitude)
                            .on_input(|s| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::Constraint1Variable1MagnitudeChanged(s),
                                )
                            })
                            .width(60),
                        text("x"),
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.constraint_1_variable_2_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::Constraint1Variable2SignChanged(s)
                            )
                        )
                        .width(60),
                        text_input("", &self.constraint_1_variable_2_magnitude)
                            .on_input(|s| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::Constraint1Variable2MagnitudeChanged(s),
                                )
                            })
                            .width(60),
                        text("y"),
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.constraint_1_variable_3_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::Constraint1Variable3SignChanged(s)
                            )
                        )
                        .width(60),
                        text_input("", &self.constraint_1_variable_3_magnitude)
                            .on_input(|s| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::Constraint1Variable3MagnitudeChanged(s),
                                )
                            })
                            .width(60),
                        text("z"),
                        pick_list(
                            [
                                Inequality::GreaterThan,
                                Inequality::GreaterThanOrEqual,
                                Inequality::LessThan,
                                Inequality::LessThanOrEqual,
                            ],
                            Some(&self.constraint_1_inequality),
                            |i| Message::NewProblemMessage(
                                NewProblemMessage::Constraint1InequalityChanged(i)
                            )
                        )
                        .width(60),
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.constraint_1_variable_4_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::Constraint1Variable4SignChanged(s)
                            )
                        )
                        .width(60),
                        text_input("", &self.constraint_1_variable_4_magnitude)
                            .on_input(|s| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::Constraint1Variable4MagnitudeChanged(s),
                                )
                            })
                            .width(60),
                    ]
                    .align_y(Alignment::Center)
                    .spacing(10),
                    row![
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.constraint_2_variable_1_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::Constraint2Variable1SignChanged(s)
                            )
                        )
                        .width(60),
                        text_input("", &self.constraint_2_variable_1_magnitude)
                            .on_input(|s| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::Constraint2Variable1MagnitudeChanged(s),
                                )
                            })
                            .width(60),
                        text("x"),
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.constraint_2_variable_2_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::Constraint2Variable2SignChanged(s)
                            )
                        )
                        .width(60),
                        text_input("", &self.constraint_2_variable_2_magnitude)
                            .on_input(|s| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::Constraint2Variable2MagnitudeChanged(s),
                                )
                            })
                            .width(60),
                        text("y"),
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.constraint_2_variable_3_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::Constraint2Variable3SignChanged(s)
                            )
                        )
                        .width(60),
                        text_input("", &self.constraint_2_variable_3_magnitude)
                            .on_input(|m| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::Constraint2Variable3MagnitudeChanged(m),
                                )
                            })
                            .width(60),
                        text("z"),
                        pick_list(
                            [
                                Inequality::GreaterThan,
                                Inequality::GreaterThanOrEqual,
                                Inequality::LessThan,
                                Inequality::LessThanOrEqual,
                            ],
                            Some(&self.constraint_2_inequality),
                            |i| Message::NewProblemMessage(
                                NewProblemMessage::Constraint2InequalityChanged(i)
                            )
                        )
                        .width(60),
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.constraint_2_variable_4_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::Constraint2Variable4SignChanged(s)
                            )
                        )
                        .width(60),
                        text_input("", &self.constraint_2_variable_4_magnitude)
                            .on_input(|s| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::Constraint2Variable4MagnitudeChanged(s),
                                )
                            })
                            .width(60),
                    ]
                    .align_y(Alignment::Center)
                    .spacing(10),
                    row![
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.constraint_3_variable_1_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::Constraint3Variable1SignChanged(s)
                            )
                        )
                        .width(60),
                        text_input("", &self.constraint_3_variable_1_magnitude)
                            .on_input(|m| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::Constraint3Variable1MagnitudeChanged(m),
                                )
                            })
                            .width(60),
                        text("x"),
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.constraint_3_variable_2_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::Constraint3Variable2SignChanged(s)
                            )
                        )
                        .width(60),
                        text_input("", &self.constraint_3_variable_2_magnitude)
                            .on_input(|m| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::Constraint3Variable2MagnitudeChanged(m),
                                )
                            })
                            .width(60),
                        text("y"),
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.constraint_3_variable_3_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::Constraint3Variable3SignChanged(s)
                            )
                        )
                        .width(60),
                        text_input("", &self.constraint_3_variable_3_magnitude)
                            .on_input(|m| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::Constraint3Variable3MagnitudeChanged(m),
                                )
                            })
                            .width(60),
                        text("z"),
                        pick_list(
                            [
                                Inequality::GreaterThan,
                                Inequality::GreaterThanOrEqual,
                                Inequality::LessThan,
                                Inequality::LessThanOrEqual,
                            ],
                            Some(&self.constraint_3_inequality),
                            |i| Message::NewProblemMessage(
                                NewProblemMessage::Constraint3InequalityChanged(i)
                            )
                        )
                        .width(60),
                        pick_list(
                            [Sign::Add, Sign::Sub],
                            Some(&self.constraint_3_variable_4_sign),
                            |s| Message::NewProblemMessage(
                                NewProblemMessage::Constraint3Variable4SignChanged(s)
                            )
                        )
                        .width(60),
                        text_input("", &self.constraint_3_variable_4_magnitude)
                            .on_input(|m| {
                                Message::NewProblemMessage(
                                    NewProblemMessage::Constraint3Variable4MagnitudeChanged(m),
                                )
                            })
                            .width(60),
                    ]
                    .align_y(Alignment::Center)
                    .spacing(10),
                    button(text("Solve").center())
                        .on_press(Message::NewProblemMessage(
                            NewProblemMessage::SolveButtonPressed
                        ))
                        .width(700),
                    text(self.error.to_string()),
                ]
                .spacing(20),
            )
            .center_x(Length::Fill),
        )
        .width(Length::Fill)
        .into()
    }
}
