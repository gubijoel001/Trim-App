use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        style { include_str!("style.css") } // Load CSS for styling
        div {
            class: "container",
            // Logo Section
            img {
                src: "assets/logo.png", // Replace with your logo
                alt: "Trim Logo",
                class: "logo",
            }
            // App Name
            h1 { class: "app-name", "Trim" }
            // Tagline
            p { class: "tagline", "Your Gateway to Financial Freedom" }
            // Button
            button {
                class: "get-started",
                onclick: |_| {
                    // Handle button click
                    web_sys::console::log_1(&"Get Started Clicked".into());
                },
                "Get Started for Free"
            }
        }
    })
}
