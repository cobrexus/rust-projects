use iced::{
    widget::{button, text, Column, Row},
    Element,
};

fn main() -> iced::Result {
    iced::run("Tabs - Iced", Tabs::update, Tabs::view)
}

#[derive(Default)]
struct Tabs {
    tabs: Vec<Tab>,
    active_tab: usize,
}

#[derive(Debug, Clone)]
enum Message {
    NewTab,
    OpenTab(usize),
    CloseTab(usize),
}

impl Tabs {
    fn update(&mut self, message: Message) {
        match message {
            Message::NewTab => {
                self.tabs.push(Tab {});
            }
            Message::OpenTab(idx) => {
                self.active_tab = idx;
            }
            Message::CloseTab(idx) => {
                if self.active_tab == idx {
                    if idx > 0 {
                        self.active_tab = idx - 1;
                    } else if self.tabs.len() > 1 {
                        self.active_tab = 0;
                    } else {
                        self.tabs.push(Tab {});
                        self.active_tab = 0;
                    }
                }
                self.tabs.remove(idx);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        fn open_tab_btn(idx: usize) -> Element<'static, Message> {
            button(text(format!("Tab {}", idx)))
                .on_press(Message::OpenTab(idx))
                .into()
        }

        if self.tabs.is_empty() {
            button("New Tab").on_press(Message::NewTab).into()
        } else {
            Column::new()
                .push(button("New Tab").on_press(Message::NewTab))
                .push(Row::from_iter(self.tabs.iter().enumerate().map(
                    |(idx, _)| {
                        Row::new()
                            .push(open_tab_btn(idx))
                            .push(button("Close Tab").on_press(Message::CloseTab(idx)))
                            .into()
                    },
                )))
                .push(self.tabs[self.active_tab].view(self.active_tab))
                .into()
        }
    }
}

struct Tab;

impl Tab {
    fn update(&mut self, message: Message) {}

    fn view(&self, idx: usize) -> Element<Message> {
        text(format!("Tab {}", idx)).into()
    }
}
