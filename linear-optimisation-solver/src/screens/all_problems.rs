use std::{
    fs::{self, File},
    io::Write,
};

use iced::{
    widget::{button, column, container, horizontal_space, row, text},
    Element, Length,
};
use serde::{Deserialize, Serialize};

use crate::{Message, Screen};

#[derive(Default)]
pub struct AllProblems {}

#[derive(Debug, Clone)]
pub enum AllProblemsMessage {
    NewProblemButtonPressed,
}

impl AllProblems {
    pub fn update(&mut self, message: AllProblemsMessage) -> Option<Screen> {
        match message {
            AllProblemsMessage::NewProblemButtonPressed => Some(Screen::NewProblem),
        }
    }

    pub fn view(&self) -> Element<Message> {
        #[derive(Serialize, Deserialize)]
        struct Problem {
            id: i32,
            title: String,
            description: String,
            constraints: Vec<String>,
            objective_function: String,
            solution: String,
            timestamp: String,
        }

        let db = match fs::read_to_string("db.json") {
            Ok(d) => d,
            Err(_) => {
                let data = "[]";

                File::create("db.json")
                    .expect("could not create database")
                    .write(&data.chars().map(|c| c as u8).collect::<Vec<_>>())
                    .expect("could not write to database");

                data.to_string()
            }
        };

        let problems = serde_json::from_str::<Vec<Problem>>(&db);

        container(column![
            row![
                horizontal_space(),
                button(text("New").width(Length::Fill).center())
                    .on_press(Message::AllProblemsMessage(
                        AllProblemsMessage::NewProblemButtonPressed
                    ))
                    .width(250),
                horizontal_space(),
                button(text("Analyse").width(Length::Fill).center()).width(250),
                horizontal_space(),
                button(text("Export").width(Length::Fill).center()).width(250),
                horizontal_space()
            ],
            if problems.is_err() {
                text("Database file empty or corrupt")
                    .center()
                    .width(Length::Fill)
                    .height(100)
            } else if problems.unwrap().is_empty() {
                text("No problems yet")
                    .center()
                    .width(Length::Fill)
                    .height(100)
            } else {
                text("Problems exist")
                    .center()
                    .width(Length::Fill)
                    .height(100)
            }
        ])
        .center(Length::Fill)
        .into()
    }
}
