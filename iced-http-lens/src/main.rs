use iced::{
    border,
    daemon::Appearance,
    theme,
    widget::{button, container, pick_list, text, text_editor, text_input, Column, Row},
    Background, Border, Color, Element, Font, Length, Shadow, Task, Theme, Vector,
};
use reqwest;
use strum::VariantArray;
use strum_macros::{Display, VariantArray};

pub fn main() -> iced::Result {
    iced::application("HTTP Lens", update, view)
        .theme(|_| {
            Theme::custom(
                String::from("HTTP Lens Theme"),
                theme::Palette {
                    background: Color::from_rgb(0.0, 0.0, 0.0),
                    text: Color::from_rgb(0.9, 0.9, 0.9),
                    primary: Color::from_rgb(0.9, 0.9, 0.9),
                    success: Color::from_rgb(0.5, 1.0, 0.5),
                    danger: Color::from_rgb(1.0, 0.5, 0.5),
                },
            )
        })
        .style(|_, _| Appearance {
            background_color: Color::from_rgb(0.0, 0.0, 0.0),
            text_color: Color::from_rgb(0.9, 0.9, 0.9),
        })
        .run()
}

#[derive(Default)]
struct State {
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

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::HttpMethodSelected(method) => {
            state.selected_http_method = method;
            Task::none()
        }
        Message::UrlInputChanged(url) => {
            state.url_entered = url;
            Task::none()
        }
        Message::RequestBodyEdited(action) => {
            state.request_body.perform(action);
            Task::none()
        }
        Message::UrlSubmitted => {
            state.loading = true;

            Task::perform(
                send_request(
                    state.selected_http_method.clone(),
                    state.url_entered.clone(),
                    state.request_body.text(),
                ),
                Message::ResponseReceived,
            )
        }
        Message::ResponseReceived(result) => {
            state.loading = false;

            match result {
                Some(response) => {
                    state.error = false;
                    state.response = response;
                    state.response_headers_content =
                        text_editor::Content::with_text(&state.response.headers);
                    state.response_body_content =
                        text_editor::Content::with_text(&state.response.body);
                }
                None => {
                    state.error = true;
                    state.response.headers = String::new();
                    state.response.body = String::new();
                }
            }
            Task::none()
        }
        Message::ResponseViewSelected(view) => {
            state.response_view_selected = view;
            Task::none()
        }
        Message::ResponseHeadersAction(action) => match action {
            text_editor::Action::Edit(_) => Task::none(),
            _ => {
                state.response_headers_content.perform(action);
                Task::none()
            }
        },
        Message::ResponseBodyAction(action) => match action {
            text_editor::Action::Edit(_) => Task::none(),
            _ => {
                state.response_body_content.perform(action);
                Task::none()
            }
        },
    }
}

fn view(state: &State) -> Element<Message> {
    let http_method_dropdown = pick_list(
        HttpMethod::VARIANTS,
        Some(&state.selected_http_method),
        Message::HttpMethodSelected,
    )
    .style(|_, _| pick_list::Style {
        border: Border {
            radius: border::Radius {
                top_left: 10.0,
                top_right: 0.0,
                bottom_right: 0.0,
                bottom_left: 10.0,
            },
            color: Color::TRANSPARENT,
            width: 0.0,
        },
        text_color: Color::from_rgb(0.0, 0.0, 0.0),
        placeholder_color: Color::from_rgb(0.9, 0.9, 0.9),
        handle_color: Color::from_rgb(0.0, 0.0, 0.0),
        background: Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
    })
    .padding(10);

    let url_input = text_input("Enter URL", &state.url_entered)
        .on_input(Message::UrlInputChanged)
        .on_submit(Message::UrlSubmitted)
        .padding(10)
        .style(|_, _| text_input::Style {
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                color: Color::from_rgb(0.9, 0.9, 0.9),
                width: 1.0,
                radius: border::Radius {
                    top_left: 0.0,
                    top_right: 0.0,
                    bottom_right: 0.0,
                    bottom_left: 0.0,
                },
            },
            icon: Default::default(),
            placeholder: Color::from_rgb(0.5, 0.5, 0.5),
            value: Color::from_rgb(0.9, 0.9, 0.9),
            selection: Color::from_rgb(0.5, 0.5, 0.5),
        });

    let submit_btn: Element<Message> = if state.loading {
        button("Loading...")
            .padding(10)
            .on_press_maybe(None)
            .style(|_, _| button::Style {
                border: Border {
                    radius: border::Radius {
                        top_left: 0.0,
                        top_right: 10.0,
                        bottom_right: 10.0,
                        bottom_left: 0.0,
                    },
                    color: Color::TRANSPARENT,
                    width: 0.0,
                },
                text_color: Color::from_rgb(0.0, 0.0, 0.0),
                background: Some(Background::Color(Color::from_rgb(0.5, 0.5, 0.5))),
                shadow: Shadow {
                    color: Color::TRANSPARENT,
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 0.0,
                },
            })
            .into()
    } else {
        button("Send Request")
            .padding(10)
            .on_press(Message::UrlSubmitted)
            .style(|_, _| button::Style {
                border: Border {
                    radius: border::Radius {
                        top_left: 0.0,
                        top_right: 10.0,
                        bottom_right: 10.0,
                        bottom_left: 0.0,
                    },
                    color: Color::TRANSPARENT,
                    width: 0.0,
                },
                text_color: Color::from_rgb(0.0, 0.0, 0.0),
                background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
                shadow: Shadow {
                    color: Color::TRANSPARENT,
                    offset: Vector::new(0.0, 0.0),
                    blur_radius: 0.0,
                },
            })
            .into()
    };

    let request_body = Column::new()
        .push(
            text_editor(&state.request_body)
                .placeholder("Request body")
                .on_action(Message::RequestBodyEdited)
                .font(Font::MONOSPACE)
                .style(|_, _| text_editor::Style {
                    border: Border {
                        radius: border::Radius {
                            top_left: 10.0,
                            top_right: 10.0,
                            bottom_right: 10.0,
                            bottom_left: 10.0,
                        },
                        color: Color::from_rgb(0.9, 0.9, 0.9),
                        width: 1.0,
                    },
                    background: Background::Color(Color::TRANSPARENT),
                    icon: Default::default(),
                    placeholder: Color::from_rgb(0.5, 0.5, 0.5),
                    value: Color::from_rgb(0.9, 0.9, 0.9),
                    selection: Color::from_rgb(0.5, 0.5, 0.5),
                })
                .padding(10),
        )
        .padding([10, 0]);

    let error_msg = if state.error {
        Some(container(
            text("Whoops, something went wrong").color(Color::from_rgb(1.0, 0.5, 0.5)),
        ))
    } else {
        None
    };

    let response_view_dropdown =
        if !state.response.headers.is_empty() || !state.response.body.is_empty() {
            Some(
                container(
                    pick_list(
                        ResponseView::VARIANTS,
                        Some(&state.response_view_selected),
                        Message::ResponseViewSelected,
                    )
                    .style(|_, _| pick_list::Style {
                        border: Border {
                            radius: border::Radius {
                                top_left: 10.0,
                                top_right: 10.0,
                                bottom_right: 10.0,
                                bottom_left: 10.0,
                            },
                            color: Color::TRANSPARENT,
                            width: 0.0,
                        },
                        text_color: Color::from_rgb(0.0, 0.0, 0.0),
                        placeholder_color: Color::from_rgb(0.9, 0.9, 0.9),
                        handle_color: Color::from_rgb(0.0, 0.0, 0.0),
                        background: Background::Color(Color::from_rgb(0.9, 0.9, 0.9)),
                    })
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
            .padding([20, 0])
            .into()
    }

    let response_text = if !state.response.headers.is_empty() || !state.response.body.is_empty() {
        Some(
            Column::new().push(match state.response_view_selected {
                ResponseView::Headers => Column::new()
                    .push(response_metadata(
                        &state.response.headers,
                        state.response.status,
                    ))
                    .push(
                        text_editor(&state.response_headers_content)
                            .on_action(Message::ResponseHeadersAction)
                            .font(Font::MONOSPACE)
                            .style(|_, _| text_editor::Style {
                                border: Border {
                                    radius: border::Radius {
                                        top_left: 10.0,
                                        top_right: 10.0,
                                        bottom_right: 10.0,
                                        bottom_left: 10.0,
                                    },
                                    color: Color::from_rgb(0.5, 0.5, 0.5),
                                    width: 1.0,
                                },
                                background: Background::Color(Color::TRANSPARENT),
                                icon: Default::default(),
                                placeholder: Color::from_rgb(0.5, 0.5, 0.5),
                                value: Color::from_rgb(0.9, 0.9, 0.9),
                                selection: Color::from_rgb(0.5, 0.5, 0.5),
                            })
                            .padding(10),
                    ),
                ResponseView::Body => Column::new()
                    .push(response_metadata(
                        &state.response.body,
                        state.response.status,
                    ))
                    .push(
                        text_editor(&state.response_body_content)
                            .on_action(Message::ResponseBodyAction)
                            .font(Font::MONOSPACE)
                            .style(|_, _| text_editor::Style {
                                border: Border {
                                    radius: border::Radius {
                                        top_left: 10.0,
                                        top_right: 10.0,
                                        bottom_right: 10.0,
                                        bottom_left: 10.0,
                                    },
                                    color: Color::from_rgb(0.5, 0.5, 0.5),
                                    width: 1.0,
                                },
                                background: Background::Color(Color::TRANSPARENT),
                                icon: Default::default(),
                                placeholder: Color::from_rgb(0.5, 0.5, 0.5),
                                value: Color::from_rgb(0.9, 0.9, 0.9),
                                selection: Color::from_rgb(0.5, 0.5, 0.5),
                            })
                            .padding(10),
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
                    .push(submit_btn),
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
