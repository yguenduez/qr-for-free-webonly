#![allow(non_snake_case)]
use std::io::{BufWriter, Cursor};

// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::prelude::*;
use image::{ImageFormat, Luma};
use qrcode::QrCode;
use url::Url;

fn main() {
    launch(App);
}

fn App() -> Element {
    const ENTRY_VALUE: &str = "https://example.com";
    let mut text = use_signal(|| ENTRY_VALUE.to_string());
    let mut query_url = use_signal(|| qr_url(ENTRY_VALUE));

    rsx! {
        div {
            class: "bg-base-500",
            div{
                class: "flex justify-center navbar bg-base-100",
                h1 {
                    class: "text-2xl text-bold p-8",
                    "Free QR"
                }
            }
            div {
                class: "flex justify-center items-center h-full",
                div {
                    div {
                        class: "flex items-center justify-center w-full",
                        img {
                            class: "rounded-3xl m-8",
                            src: "{query_url}"
                        }
                    }
                    div {
                        div {
                            class: "flex items-center justify-center",
                            input {
                                class: "input input-bordered w-full m-8",
                                placeholder: "Your URL",
                                oninput: move |evt| text.set(evt.value().clone()),
                                onkeydown: move |evt| { if evt.key() == Key::Enter || evt.key() == Key::Accept {
                                    query_url.set(qr_url(&text.read()));
                                }}
                            }
                        }
                        div {
                            class: "flex items-center justify-center",
                            button {
                                class: "btn btn-primary m-8 p-4",
                                onclick: move |_| {query_url.set(qr_url(&text.read()))},
                                p {
                                    class: "text-xl justify-center",
                                    "Generate"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

use base64::prelude::*;
fn qr_url(url: &str) -> String {
    let bytes = generate_qr_code(url);
    let encoded = BASE64_STANDARD.encode(&bytes);
    let data_url = Url::parse(&format!("data:image/png;base64,{}", encoded)).unwrap();
    data_url.to_string()
}

fn generate_qr_code(url: &str) -> Vec<u8> {
    let code = QrCode::new(url.as_bytes()).unwrap();
    let image = code.render::<Luma<u8>>().build();

    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
    image.write_to(&mut buffer, ImageFormat::Png).unwrap();
    buffer.into_inner().unwrap().into_inner()
}
