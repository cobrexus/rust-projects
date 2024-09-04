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
    response: Response,
    error: bool,
    response_view_selected: ResponseView,
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
struct Response {
    headers: String,
    body: String,
    status: String,
}

impl Response {
    fn new() -> Self {
        Self {
            headers: String::new(),
            body: String::new(),
            status: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Display)]
enum ResponseView {
    Headers,
    Body,
}

#[derive(Debug, Clone)]
enum Message {
    HttpMethodSelected(HttpMethod),
    UrlInputChanged(String),
    RequestBodyEdited(text_editor::Action),
    UrlSubmitted,
    RequestSent(Option<Response>),
    ResponseViewSelected(ResponseView),
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
                response: Response::new(),
                error: false,
                response_view_selected: ResponseView::Body,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("HTTP Lens")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
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
                        self.response.headers = String::new();
                        self.response.body = String::new();
                    }
                }
                Command::none()
            }
            Message::ResponseViewSelected(view) => {
                self.response_view_selected = view;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let http_method_dropdown = pick_list(
            ALL_HTTP_METHODS,
            Some(&self.selected_http_method),
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

        let response_view_dropdown =
            if !self.response.headers.is_empty() || !self.response.body.is_empty() {
                Some(
                    container(
                        pick_list(
                            [ResponseView::Headers, ResponseView::Body],
                            Some(&self.response_view_selected),
                            Message::ResponseViewSelected,
                        )
                        .padding(10),
                    )
                    .padding([10, 0]),
                )
            } else {
                None
            };

        fn response_meta_data(content: &str, status: &str) -> Element<'static, Message> {
            Row::new()
                .push(text("Size: "))
                .push(text(content.as_bytes().len()))
                .push("B")
                .push(" • ")
                .push("Status: ")
                .push(text(status))
                .padding([10, 0])
                .into()
        }

        let response_text = if !self.response.headers.is_empty() || !self.response.body.is_empty() {
            Some(
                Column::new().push(match self.response_view_selected {
                    ResponseView::Headers => Column::new()
                        .push(response_meta_data(
                            &self.response.headers,
                            &self.response.status,
                        ))
                        .push(
                            scrollable(text(&self.response.headers))
                                .direction(Direction::Horizontal(Default::default())),
                        ),
                    ResponseView::Body => Column::new()
                        .push(response_meta_data(
                            &self.response.body,
                            &self.response.status,
                        ))
                        .push(
                            scrollable(text(&self.response.body))
                                .direction(Direction::Horizontal(Default::default())),
                        ),
                }),
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
                    .push_maybe(response_view_dropdown)
                    .push_maybe(response_text)
                    .width(1000)
                    .padding(20),
            )
            .center_x()
            .width(Length::Fill),
        )
        .into()
    }
}

async fn send_request(method: HttpMethod, url: String, body: String) -> Option<Response> {
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

                let status = response.status().as_str().to_owned();

                if let Ok(body) = response.text().await {
                    Some(Response {
                        headers,
                        body,
                        status,
                    })
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

                let status = response.status().as_str().to_owned();

                if let Ok(body) = response.text().await {
                    Some(Response {
                        headers,
                        body,
                        status,
                    })
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

                let status = response.status().as_str().to_owned();

                if let Ok(body) = response.text().await {
                    Some(Response {
                        headers,
                        body,
                        status,
                    })
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

                let status = response.status().as_str().to_owned();

                if let Ok(body) = response.text().await {
                    Some(Response {
                        headers,
                        body,
                        status,
                    })
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

                let status = response.status().as_str().to_owned();

                if let Ok(body) = response.text().await {
                    Some(Response {
                        headers,
                        body,
                        status,
                    })
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

                let status = response.status().as_str().to_owned();

                if let Ok(body) = response.text().await {
                    Some(Response {
                        headers,
                        body,
                        status,
                    })
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
