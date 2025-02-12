use iced::Element;

use screens::{
    all_problems::{AllProblems, AllProblemsMessage},
    new_problem::{NewProblem, NewProblemMessage},
};

mod screens;

fn main() -> iced::Result {
    iced::application("Linear Optimisation Solver", App::update, App::view).run()
}

#[derive(Default)]
struct App {
    screen: Screen,

    all_problems: AllProblems,
    new_problem: NewProblem,
}

#[derive(Debug, Clone)]
enum Message {
    AllProblemsMessage(AllProblemsMessage),
    NewProblemMessage(NewProblemMessage),
}

#[derive(Default, Debug, Clone)]
enum Screen {
    #[default]
    AllProblems,
    NewProblem,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::AllProblemsMessage(m) => {
                if let Some(s) = self.all_problems.update(m) {
                    self.screen = s;
                }
            }
            Message::NewProblemMessage(m) => {
                if let Some(s) = self.new_problem.update(m) {
                    self.screen = s;
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self.screen {
            Screen::AllProblems => self.all_problems.view(),
            Screen::NewProblem => self.new_problem.view(),
        }
    }
}
