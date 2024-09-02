use iced::{
    executor,
    widget::{
        button, container, pick_list, scrollable, scrollable::Direction, text, text_editor,
        text_input, Column, Row,
    },
    Application, Color, Command, Element, Length, Settings, Theme,
};
use reqwest;
use strum_macros::Display;

pub fn main() -> iced::Result {
    HttpLens::run(Settings::default())
}

struct HttpLens {
    selected_http_method: HttpMethod,
    url_entered: String,
    request_body: text_editor::Content,
    loading: bool,
    response: String,
    error: bool,
}

#[derive(Debug, Clone, PartialEq, Display)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Patch,
}

const ALL_HTTP_METHODS: [HttpMethod; 6] = [
    HttpMethod::Get,
    HttpMethod::Post,
    HttpMethod::Put,
    HttpMethod::Delete,
    HttpMethod::Head,
    HttpMethod::Patch,
];

#[derive(Debug, Clone)]
enum Message {
    HttpMethodSelected(HttpMethod),
    UrlInputChanged(String),
    RequestBodyEdited(text_editor::Action),
    UrlSubmitted,
    RequestSent(Option<String>),
}

impl Application for HttpLens {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (HttpLens, Command<Message>) {
        (
            HttpLens {
                selected_http_method: HttpMethod::Get,
                url_entered: String::new(),
                request_body: text_editor::Content::new(),
                loading: false,
                response: String::new(),
                error: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("HTTP Lens")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::HttpMethodSelected(method) => {
                self.selected_http_method = method;
                Command::none()
            }
            Message::UrlInputChanged(url) => {
                self.url_entered = url;
                Command::none()
            }
            Message::RequestBodyEdited(action) => {
                self.request_body.perform(action);
                Command::none()
            }
            Message::UrlSubmitted => {
                self.loading = true;
                Command::perform(
                    send_request(
                        self.selected_http_method.clone(),
                        self.url_entered.clone(),
                        self.request_body.text(),
                    ),
                    Message::RequestSent,
                )
            }
            Message::RequestSent(result) => {
                self.loading = false;
                match result {
                    Some(response) => {
                        self.error = false;
                        self.response = response;
                    }
                    None => {
                        self.error = true;
                        self.response = String::new();
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        container(
            Column::new()
                .push(
                    Row::new()
                        .push(
                            pick_list(
                                ALL_HTTP_METHODS,
                                Some(self.selected_http_method.clone()),
                                Message::HttpMethodSelected,
                            )
                            .padding(7),
                        )
                        .push(
                            text_input("Enter URL", &self.url_entered)
                                .on_input(Message::UrlInputChanged)
                                .on_submit(Message::UrlSubmitted)
                                .padding(7),
                        )
                        .push(
                            button(if self.loading {
                                "Loading..."
                            } else {
                                "Send Request"
                            })
                            .padding(7)
                            .on_press(Message::UrlSubmitted),
                        )
                        .spacing(10),
                )
                .push(
                    container(
                        text_editor(&self.request_body)
                            .on_action(Message::RequestBodyEdited)
                            .padding(7),
                    )
                    .padding([10, 0]),
                )
                .push_maybe(if self.error {
                    Some(text("Whoops, something went wrong").style(Color::from_rgb(1.0, 0.5, 0.5)))
                } else {
                    None
                })
                .push(scrollable(text(&self.response)).direction(Direction::Both {
                    vertical: Default::default(),
                    horizontal: Default::default(),
                }))
                .width(1000)
                .padding(20),
        )
        .center_x()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

async fn send_request(method: HttpMethod, url: String, body: String) -> Option<String> {
    match method {
        HttpMethod::Get => {
            let client = reqwest::Client::new();
            if let Ok(response) = client.get(url).body(body).send().await {
                if let Ok(text) = response.text().await {
                    Some(text)
                } else {
                    None
                }
            } else {
                None
            }
        }
        HttpMethod::Post => {
            let client = reqwest::Client::new();
            if let Ok(response) = client.post(url).body(body).send().await {
                if let Ok(text) = response.text().await {
                    Some(text)
                } else {
                    None
                }
            } else {
                None
            }
        }
        HttpMethod::Put => {
            let client = reqwest::Client::new();
            if let Ok(response) = client.put(url).body(body).send().await {
                if let Ok(text) = response.text().await {
                    Some(text)
                } else {
                    None
                }
            } else {
                None
            }
        }
        HttpMethod::Delete => {
            let client = reqwest::Client::new();
            if let Ok(response) = client.delete(url).body(body).send().await {
                if let Ok(text) = response.text().await {
                    Some(text)
                } else {
                    None
                }
            } else {
                None
            }
        }
        HttpMethod::Head => {
            let client = reqwest::Client::new();
            if let Ok(response) = client.head(url).body(body).send().await {
                if let Ok(text) = response.text().await {
                    Some(text)
                } else {
                    None
                }
            } else {
                None
            }
        }
        HttpMethod::Patch => {
            let client = reqwest::Client::new();
            if let Ok(response) = client.patch(url).body(body).send().await {
                if let Ok(text) = response.text().await {
                    Some(text)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
