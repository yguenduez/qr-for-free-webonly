#![allow(non_snake_case)]
use std::io::{BufWriter, Cursor};

// import the prelude to get access to the `rsx!` macro and the `Element` type
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use image::{ImageFormat, Luma};
use qrcode::QrCode;
use url::Url;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    const ENTRY_VALUE: &str = "https://example.com";
    let text = use_state(cx, || ENTRY_VALUE.to_string());
    let query_url = use_state(cx, || qr_url(ENTRY_VALUE));

    cx.render(rsx! {
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
                    class: "flex items-center rounded-3xl shadow-md",
                    input {
                        class: "input input-bordered w-full m-8",
                        placeholder: "Your URL",
                        oninput: move |evt| text.set(evt.value.clone()),
                        onkeydown: move |evt| { if evt.key() == Key::Enter || evt.key() == Key::Accept {
                            query_url.set(qr_url(text.get()));
                        }}
                    }
                    div {
                        class: "flex items-center justify-center w-full m-8",
                        img {
                            class: "rounded-3xl",
                            src: "{query_url}"
                        }
                    }
                }
            }
        }
    })
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
