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
    response_headers: String,
    response_body: String,
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
    RequestSent(Option<(ResponseHeaders, ResponseBody)>),
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
                response_headers: String::new(),
                response_body: String::new(),
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
                        self.response_headers = response.0 .0;
                        self.response_body = response.1 .0;
                    }
                    None => {
                        self.error = true;
                        self.response_headers = String::new();
                        self.response_body = String::new();
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let http_method_dropdown = pick_list(
            ALL_HTTP_METHODS,
            Some(self.selected_http_method.clone()),
            Message::HttpMethodSelected,
        )
        .padding(10);

        let url_input = text_input("Enter URL", &self.url_entered)
            .on_input(Message::UrlInputChanged)
            .on_submit(Message::UrlSubmitted)
            .padding(10);

        let submit_btn: Element<Message> = if self.loading {
            button("Loading...").padding(10).on_press_maybe(None).into()
        } else {
            button("Send Request")
                .padding(10)
                .on_press(Message::UrlSubmitted)
                .into()
        };

        let request_body = Column::new()
            .push(container("Request body").padding([10, 0]))
            .push(text_editor(&self.request_body).on_action(Message::RequestBodyEdited))
            .padding([10, 0]);

        let error_msg = if self.error {
            Some(container(
                text("Whoops, something went wrong").style(Color::from_rgb(1.0, 0.5, 0.5)),
            ))
        } else {
            None
        };

        let scrollable_response_text =
            if !self.response_headers.is_empty() || !self.response_body.is_empty() {
                Some(
                    Column::new()
                        .push(container(text("Response Headers").size(25)).padding([10, 0]))
                        .push(
                            scrollable(text(&self.response_headers))
                                .direction(Direction::Horizontal(Default::default())),
                        )
                        .push(container(text("Response Body").size(25)).padding([10, 0]))
                        .push(
                            scrollable(text(&self.response_body))
                                .direction(Direction::Horizontal(Default::default())),
                        ),
                )
            } else {
                None
            };

        scrollable(
            container(
                Column::new()
                    .push(
                        Row::new()
                            .push(http_method_dropdown)
                            .push(url_input)
                            .push(submit_btn)
                            .spacing(10),
                    )
                    .push(request_body)
                    .push_maybe(error_msg)
                    .push_maybe(scrollable_response_text)
                    .width(1000)
                    .padding(20),
            )
            .center_x()
            .width(Length::Fill),
        )
        .into()
    }
}

#[derive(Debug, Clone)]
struct ResponseBody(String);

#[derive(Debug, Clone)]
struct ResponseHeaders(String);

async fn send_request(
    method: HttpMethod,
    url: String,
    body: String,
) -> Option<(ResponseHeaders, ResponseBody)> {
    match method {
        HttpMethod::Get => {
            let client = reqwest::Client::new();
            if let Ok(response) = client.get(url).body(body).send().await {
                let headers = response
                    .headers()
                    .iter()
                    .map(|header| {
                        format!("{}: {}\n", header.0.as_str(), header.1.to_str().unwrap())
                    })
                    .collect::<String>();

                if let Ok(body) = response.text().await {
                    Some((ResponseHeaders(headers), ResponseBody(body)))
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
                let headers = response
                    .headers()
                    .iter()
                    .map(|header| format!("{}: {}", header.0.as_str(), header.1.to_str().unwrap()))
                    .collect::<String>();

                if let Ok(body) = response.text().await {
                    Some((ResponseHeaders(headers), ResponseBody(body)))
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
                let headers = response
                    .headers()
                    .iter()
                    .map(|header| format!("{}: {}", header.0.as_str(), header.1.to_str().unwrap()))
                    .collect::<String>();

                if let Ok(body) = response.text().await {
                    Some((ResponseHeaders(headers), ResponseBody(body)))
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
                let headers = response
                    .headers()
                    .iter()
                    .map(|header| format!("{}: {}", header.0.as_str(), header.1.to_str().unwrap()))
                    .collect::<String>();

                if let Ok(body) = response.text().await {
                    Some((ResponseHeaders(headers), ResponseBody(body)))
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
                let headers = response
                    .headers()
                    .iter()
                    .map(|header| format!("{}: {}", header.0.as_str(), header.1.to_str().unwrap()))
                    .collect::<String>();

                if let Ok(body) = response.text().await {
                    Some((ResponseHeaders(headers), ResponseBody(body)))
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
                let headers = response
                    .headers()
                    .iter()
                    .map(|header| format!("{}: {}", header.0.as_str(), header.1.to_str().unwrap()))
                    .collect::<String>();

                if let Ok(body) = response.text().await {
                    Some((ResponseHeaders(headers), ResponseBody(body)))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
