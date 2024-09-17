#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use reqwest::RequestBuilder;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut http_method = use_signal(|| String::from(""));
    let mut request_body = use_signal(|| String::from(""));

    let mut response_headers = use_signal(|| String::from(""));
    let mut response_body = use_signal(|| String::from(""));
    let mut response_status = use_signal(|| 0u16);

    let mut error = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let mut url = use_signal(|| String::from(""));

    let send_req = move |_| {
        spawn(async move {
            loading.set(true);

            let client = reqwest::Client::new();

            let response: RequestBuilder;

            if &*http_method.read() == "GET" {
                response = client.get(url());
            } else if &*http_method.read() == "POST" {
                response = client.post(url());
            } else if &*http_method.read() == "PUT" {
                response = client.put(url());
            } else if &*http_method.read() == "DELETE" {
                response = client.delete(url());
            } else if &*http_method.read() == "HEAD" {
                response = client.head(url());
            } else {
                response = client.patch(url());
            }

            if let Ok(res) = response.body(request_body()).send().await {
                let headers = res
                    .headers()
                    .iter()
                    .map(|header| {
                        format!("{}: {}\n", header.0.as_str(), header.1.to_str().unwrap())
                    })
                    .collect::<String>();

                response_status.set(res.status().as_u16().to_owned());

                if let Ok(body) = res.text().await {
                    response_body.set(body);
                    error.set(false);
                    loading.set(false);
                    response_headers.set(headers);
                } else {
                    error.set(true);
                    loading.set(false);
                }
            } else {
                error.set(true);
                loading.set(false);
            }
        });
    };

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        form {
            onsubmit: send_req,
            prevent_default: "onsubmit",
            class: "control-bar",
            select {
                onchange: move |evt| http_method.set(evt.data.value()),
                option {
                    value: "GET",
                    "GET"
                }
                option {
                    value: "POST",
                    "POST"
                }
                option {
                    value: "PUT",
                    "PUT"
                }
                option {
                    value: "DELETE",
                    "DELETE"
                }
                option {
                    value: "HEAD",
                    "HEAD"
                }
                option {
                    value: "PATCH",
                    "PATCH"
                }
            }
            input {
                r#type: "url",
                placeholder: "Enter a URL",
                autofocus: true,
                value: "{url}",
                oninput: move |evt| url.set(evt.value())
            }
            button {
                "Send Request"
            }
            br {}
            if error() {
                p {
                    class: "error",
                    "Whoops, something went wrong"
                }
            }
            if loading() {
                p {
                    class: "loading",
                    "Loading..."
                }
            }
            textarea {
                placeholder: "Request body",
                value: "{request_body}",
                oninput: move |evt| request_body.set(evt.value())
            }
        }
        if !response_body().is_empty() {
            p {
                margin_top: "30px",
                "Status: {response_status}"
            }
            textarea {
                readonly: true,
                value: "{response_headers}",
            }
            textarea {
                readonly: true,
                value: "{response_body}"
            }
        }
    }
}
