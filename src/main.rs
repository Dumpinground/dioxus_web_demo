#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

pub fn About(cx: Scope) -> Element {
    cx.render(rsx!(
        p {
            b { "Dioxus Labs" }
            br {}
            " An Open Source project dedicated to making Rust UI wonderful."
        }
    ))
}

#[derive(PartialEq, Props)]
struct LikesProps {
    score: i32,
}

fn Likes(cx: Scope<LikesProps>) -> Element {
    cx.render(rsx!(
        div {
            "This post has "
            b { "{cx.props.score}" }
            " likes"
        }
    ))
}

pub fn Hint(cx: Scope) -> Element {
    cx.render(rsx!(
        img {
            src: "no source"
        }
        header {
            id: 1902,
            class: "reader"
        }
    ))
}

fn App(cx: Scope) -> Element {
    let coordinates = (42, 0);
    let country = "es";

    cx.render(rsx!(
        div {
            "Hello, world! 123"
        }

        About {}
        About {}
        Likes {
            score: 42,
        }

        div {
            class: "country-{country}",
            "position": "{coordinates:?}",

            div {
                "{country.to_uppercase()}"
            },
            div {
                "{7 * 6 - 16}"
            },
            div {
                "{{}}"
            },
            div {
                "read"
            },
        }

        a {
            href: "https://baidu.com",
            class: "primary_button",
            color: "red",
        }
    ))
}
