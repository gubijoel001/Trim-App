use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh; font-family: Arial, sans-serif; background-color: #f8f9fa;",
            
            div {
                style: "margin-bottom: 2rem; text-align: center;",
                img {
                    src: "https://path_to_logo.com/logo.png",
                    alt: "Trim Logo",
                    style: "width: 50px; height: 50px; margin-bottom: 1rem;"
                }
                h1 { "Trim" }
                p { "Your Gateway to Financial Freedom" }
            }

            form {
                style: "width: 80%; max-width: 400px;",

                label {
                    style: "font-size: 0.9rem; margin-bottom: 0.5rem; display: block;",
                    "Your email address"
                }
                input {
                    style: "width: 100%; padding: 0.75rem; margin-bottom: 1rem; border: 1px solid #ccc; border-radius: 5px;",
                    placeholder: "Enter your email",
                    r#type: "email"
                }

                label {
                    style: "font-size: 0.9rem; margin-bottom: 0.5rem; display: block;",
                    "Choose a password"
                }
                div {
                    style: "display: flex; align-items: center; border: 1px solid #ccc; border-radius: 5px; margin-bottom: 1rem; overflow: hidden;",
                    input {
                        style: "flex: 1; padding: 0.75rem; border: none; outline: none;",
                        placeholder: "Enter your password",
                        r#type: "password"
                    }
                    button {
                        style: "padding: 0 1rem; background: none; border: none; cursor: pointer;",
                        "👁" // Add password visibility toggle logic here
                    }
                }

                button {
                    style: "width: 100%; padding: 0.75rem; background-color: #6a0dad; color: white; border: none; border-radius: 5px; font-size: 1rem; cursor: pointer;",
                    "Continue"
                }
            }

            div {
                style: "margin: 1.5rem 0; text-align: center; color: #6c757d;",
                "or"
            }

            div {
                style: "width: 80%; max-width: 400px;",
                button {
                    style: "width: 100%; padding: 0.75rem; margin-bottom: 0.5rem; background-color: white; border: 1px solid #ccc; border-radius: 5px; display: flex; align-items: center; justify-content: center; font-size: 0.9rem; cursor: pointer;",
                    img {
                        src: "https://path_to_google_icon.com/google.png",
                        alt: "Google Icon",
                        style: "width: 20px; height: 20px; margin-right: 0.5rem;"
                    }
                    "Sign up with Google"
                }
                button {
                    style: "width: 100%; padding: 0.75rem; background-color: white; border: 1px solid #ccc; border-radius: 5px; display: flex; align-items: center; justify-content: center; font-size: 0.9rem; cursor: pointer;",
                    img {
                        src: "https://path_to_apple_icon.com/apple.png",
                        alt: "Apple Icon",
                        style: "width: 20px; height: 20px; margin-right: 0.5rem;"
                    }
                    "Sign up with Apple"
                }
            }
        }
    })
}
