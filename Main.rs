use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            style: "font-family: Arial, sans-serif; padding: 1rem; background-color: #f8f9fa; height: 100vh; display: flex; flex-direction: column; justify-content: space-between; align-items: center;",

            // Header Section
            div {
                style: "width: 100%; display: flex; justify-content: space-between; align-items: center;",
                
                // Back Button
                button {
                    style: "background: none; border: none; font-size: 1.5rem;",
                    "<"
                }

                // Title
                h1 {
                    style: "font-size: 1.5rem; margin: 0;",
                    "Exchange"
                }

                // Placeholder for alignment
                span { " " }
            }

            // Exchange Panel
            div {
                style: "width: 100%; max-width: 400px; background-color: white; padding: 1.5rem; border-radius: 10px; box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1);",

                // SOL Section
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;",
                    
                    // Token Info
                    div {
                        style: "display: flex; align-items: center;",
                        img {
                            src: "https://path_to_sol_icon.com/sol.png",
                            alt: "SOL",
                            style: "width: 30px; height: 30px; margin-right: 0.5rem;"
                        }
                        span {
                            style: "font-weight: bold;",
                            "SOL"
                        }
                    }

                    // Balance and Send Button
                    div {
                        style: "display: flex; align-items: center;",
                        span { "Balance: 0.6948 ETH" }
                        button {
                            style: "margin-left: 1rem; padding: 0.5rem 1rem; background-color: #e9e3f5; color: #6a0dad; border: none; border-radius: 5px;",
                            "Send"
                        }
                    }
                }

                // Amount
                h2 {
                    style: "font-size: 2rem; margin: 0;",
                    "0,6948"
                }

                // Divider with Swap Icon
                div {
                    style: "display: flex; justify-content: center; align-items: center; margin: 1rem 0;",
                    hr {
                        style: "flex: 1; border: none; height: 1px; background-color: #ccc; margin: 0 1rem;"
                    }
                    img {
                        src: "https://path_to_swap_icon.com/swap.png",
                        alt: "Swap",
                        style: "width: 20px; height: 20px;"
                    }
                    hr {
                        style: "flex: 1; border: none; height: 1px; background-color: #ccc; margin: 0 1rem;"
                    }
                }

                // USD Section
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;",
                    
                    // Token Info
                    div {
                        style: "display: flex; align-items: center;",
                        img {
                            src: "https://path_to_usd_icon.com/usd.png",
                            alt: "USD",
                            style: "width: 30px; height: 30px; margin-right: 0.5rem;"
                        }
                        span {
                            style: "font-weight: bold;",
                            "USD"
                        }
                    }

                    // Balance and Receive Button
                    div {
                        style: "display: flex; align-items: center;",
                        span { "Balance: 100.95 USD" }
                        button {
                            style: "margin-left: 1rem; padding: 0.5rem 1rem; background-color: #e9e3f5; color: #6a0dad; border: none; border-radius: 5px;",
                            "Receive"
                        }
                    }
                }

                // Amount
                h2 {
                    style: "font-size: 2rem; margin: 0;",
                    "1,801.73"
                }
            }

            // Swap Button
            button {
                style: "width: 100%; max-width: 400px; padding: 1rem; background-color: #6a0dad; color: white; border: none; border-radius: 10px; font-size: 1.2rem; margin-top: 1rem;",
                "Swap"
            }

            // Rate and Fee Section
            div {
                style: "width: 100%; max-width: 400px; text-align: left; margin-top: 1rem;",
                p {
                    style: "margin: 0.5rem 0;",
                    "Rate: 1 SOL = 0.27 USD"
                }
                p {
                    style: "margin: 0.5rem 0;",
                    "Estimated Fee: 4.28 USD"
                }
                p {
                    style: "margin: 0.5rem 0;",
                    "You will Receive: 1,797.45 USD"
                }
            }
        }
    })
}
