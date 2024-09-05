use iced::{
    font::{Family, Stretch, Style, Weight},
    widget::{button, container, pick_list, text, text_editor, text_input, Column, Row},
    Color, Element, Font, Length, Task, Theme,
};
use reqwest;
use strum::VariantArray;
use strum_macros::{Display, VariantArray};

pub fn main() -> iced::Result {
    iced::application("HTTP Lens", HttpLens::update, HttpLens::view)
        .theme(|_| Theme::Dark)
        .default_font(Font {
            family: Family::Monospace,
            weight: Weight::Normal,
            stretch: Stretch::Normal,
            style: Style::Normal,
        })
        .run()
}

#[derive(Default)]
struct HttpLens {
    selected_http_method: HttpMethod,
    url_entered: String,
    request_body: text_editor::Content,
    loading: bool,
    response: Response,
    error: bool,
    response_view_selected: ResponseView,
    response_headers_content: text_editor::Content,
    response_body_content: text_editor::Content,
}

#[derive(Default, Debug, Clone, PartialEq, Display, VariantArray)]
enum HttpMethod {
    #[default]
    Get,
    Post,
    Put,
    Delete,
    Head,
    Patch,
}

#[derive(Default, Debug, Clone)]
struct Response {
    headers: String,
    body: String,
    status: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Display, VariantArray)]
enum ResponseView {
    Headers,
    #[default]
    Body,
}

#[derive(Debug, Clone)]
enum Message {
    HttpMethodSelected(HttpMethod),
    UrlInputChanged(String),
    RequestBodyEdited(text_editor::Action),
    UrlSubmitted,
    ResponseReceived(Option<Response>),
    ResponseViewSelected(ResponseView),
    ResponseHeadersAction(text_editor::Action),
    ResponseBodyAction(text_editor::Action),
}

impl HttpLens {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::HttpMethodSelected(method) => {
                self.selected_http_method = method;
                Task::none()
            }
            Message::UrlInputChanged(url) => {
                self.url_entered = url;
                Task::none()
            }
            Message::RequestBodyEdited(action) => {
                self.request_body.perform(action);
                Task::none()
            }
            Message::UrlSubmitted => {
                self.loading = true;

                Task::perform(
                    send_request(
                        self.selected_http_method.clone(),
                        self.url_entered.clone(),
                        self.request_body.text(),
                    ),
                    Message::ResponseReceived,
                )
            }
            Message::ResponseReceived(result) => {
                self.loading = false;

                match result {
                    Some(response) => {
                        self.error = false;
                        self.response = response;
                        self.response_headers_content =
                            text_editor::Content::with_text(&self.response.headers);
                        self.response_body_content =
                            text_editor::Content::with_text(&self.response.body);
                    }
                    None => {
                        self.error = true;
                        self.response.headers = String::new();
                        self.response.body = String::new();
                    }
                }
                Task::none()
            }
            Message::ResponseViewSelected(view) => {
                self.response_view_selected = view;
                Task::none()
            }
            Message::ResponseHeadersAction(action) => match action {
                text_editor::Action::Edit(_) => Task::none(),
                _ => {
                    self.response_headers_content.perform(action);
                    Task::none()
                }
            },
            Message::ResponseBodyAction(action) => match action {
                text_editor::Action::Edit(_) => Task::none(),
                _ => {
                    self.response_body_content.perform(action);
                    Task::none()
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let http_method_dropdown = pick_list(
            HttpMethod::VARIANTS,
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
                text("Whoops, something went wrong").color(Color::from_rgb(1.0, 0.5, 0.5)),
            ))
        } else {
            None
        };

        let response_view_dropdown =
            if !self.response.headers.is_empty() || !self.response.body.is_empty() {
                Some(
                    container(
                        pick_list(
                            ResponseView::VARIANTS,
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

        fn response_metadata(content: &str, status: u16) -> Element<'static, Message> {
            Row::new()
                .push(
                    text(format!(
                        "Status: {} • Size: {}B",
                        status,
                        content.as_bytes().len()
                    ))
                    .color(if status == 200 {
                        Color::from_rgb(0.5, 1.0, 0.5)
                    } else {
                        Color::from_rgb(1.0, 0.5, 0.5)
                    }),
                )
                .padding([10, 0])
                .into()
        }

        let response_text = if !self.response.headers.is_empty() || !self.response.body.is_empty() {
            Some(
                Column::new().push(match self.response_view_selected {
                    ResponseView::Headers => Column::new()
                        .push(response_metadata(
                            &self.response.headers,
                            self.response.status,
                        ))
                        .push(
                            text_editor(&self.response_headers_content)
                                .on_action(Message::ResponseHeadersAction),
                        ),
                    ResponseView::Body => Column::new()
                        .push(response_metadata(&self.response.body, self.response.status))
                        .push(
                            text_editor(&self.response_body_content)
                                .on_action(Message::ResponseBodyAction),
                        ),
                }),
            )
        } else {
            None
        };

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
        .center_x(Length::Fill)
        .into()
    }
}

async fn send_request(method: HttpMethod, url: String, body: String) -> Option<Response> {
    let client = reqwest::Client::new();

    let response = match method {
        HttpMethod::Get => client.get(url),
        HttpMethod::Post => client.post(url),
        HttpMethod::Put => client.put(url),
        HttpMethod::Delete => client.delete(url),
        HttpMethod::Head => client.head(url),
        HttpMethod::Patch => client.patch(url),
    };

    if let Ok(r) = response.body(body).send().await {
        let headers = r
            .headers()
            .iter()
            .map(|header| format!("{}: {}\n", header.0.as_str(), header.1.to_str().unwrap()))
            .collect::<String>();

        let status = r.status().as_u16().to_owned();

        if let Ok(body) = r.text().await {
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
