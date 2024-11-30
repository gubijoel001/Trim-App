use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; font-family: Arial, sans-serif; background-color: #f8f9fa; padding: 1rem;",

            // Top Section with Profile and Icons
            div {
                style: "display: flex; justify-content: space-between; align-items: center; width: 100%; max-width: 600px; margin-bottom: 1rem;",
                
                // Profile Picture
                div {
                    style: "display: flex; align-items: center;",
                    img {
                        src: "https://path_to_profile_image.com/profile.jpg",
                        alt: "Profile",
                        style: "width: 50px; height: 50px; border-radius: 50%; margin-right: 1rem;"
                    }
                    input {
                        style: "padding: 0.5rem; width: 250px; border: 1px solid #ccc; border-radius: 20px; font-size: 1rem;",
                        placeholder: "Search"
                    }
                }

                // Notifications
                div {
                    img {
                        src: "https://path_to_notification_icon.com/bell.png",
                        alt: "Notifications",
                        style: "width: 30px; height: 30px;"
                    }
                }
            }

            // Total Balance Section
            div {
                style: "background-color: white; width: 100%; max-width: 600px; padding: 1.5rem; border-radius: 10px; box-shadow: 0 4px 10px rgba(0, 0, 0, 0.1); margin-bottom: 1rem; text-align: center;",
                p { "Total Balance" }
                h1 {
                    style: "margin: 0.5rem 0;",
                    "$TRM"
                }
                h2 {
                    style: "font-size: 2.5rem; margin: 0;",
                    "72,829.62"
                }
                div {
                    style: "display: flex; justify-content: space-between; margin-top: 1rem;",
                    button {
                        style: "flex: 1; margin: 0 0.5rem; padding: 0.5rem; background-color: #6a0dad; color: white; border: none; border-radius: 5px;",
                        "+ Top Up"
                    }
                    button {
                        style: "flex: 1; margin: 0 0.5rem; padding: 0.5rem; background-color: #6a0dad; color: white; border: none; border-radius: 5px;",
                        "- Withdraw"
                    }
                    button {
                        style: "flex: 1; margin: 0 0.5rem; padding: 0.5rem; background-color: #6a0dad; color: white; border: none; border-radius: 5px;",
                        "↔ Exchange"
                    }
                }
            }

            // Quick Actions
            div {
                style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem; width: 100%; max-width: 600px; margin-bottom: 1rem;",
                button {
                    style: "padding: 1rem; text-align: center; background-color: white; border: 1px solid #ccc; border-radius: 10px; font-size: 0.9rem;",
                    "Payments\nGet Paid/Pay for Service"
                }
                button {
                    style: "padding: 1rem; text-align: center; background-color: white; border: 1px solid #ccc; border-radius: 10px; font-size: 0.9rem;",
                    "Services\nReach Service Providers"
                }
                button {
                    style: "padding: 1rem; text-align: center; background-color: white; border: 1px solid #ccc; border-radius: 10px; font-size: 0.9rem;",
                    "Savings\nStake your Savings"
                }
                button {
                    style: "padding: 1rem; text-align: center; background-color: white; border: 1px solid #ccc; border-radius: 10px; font-size: 0.9rem;",
                    "Loans\nApply for a Loan"
                }
            }

            // Recent Transactions
            div {
                style: "width: 100%; max-width: 600px; margin-bottom: 1rem;",
                h3 { "Recent Transactions" }
                div {
                    style: "display: flex; justify-content: space-between; padding: 1rem; background-color: white; border: 1px solid #ccc; border-radius: 10px; margin-bottom: 0.5rem;",
                    span { "TRM to SOL" }
                    span { "+0.0116 TRM" }
                }
                div {
                    style: "display: flex; justify-content: space-between; padding: 1rem; background-color: white; border: 1px solid #ccc; border-radius: 10px;",
                    span { "SOL to USDC" }
                    span { "+0.0217 SOL" }
                }
            }

            // Navigation Bar
            div {
                style: "display: flex; justify-content: space-around; width: 100%; max-width: 600px; padding: 1rem 0; background-color: white; border-radius: 20px; position: fixed; bottom: 0;",
                button {
                    style: "background: none; border: none; text-align: center;",
                    img {
                        src: "https://path_to_dashboard_icon.com/home.png",
                        style: "width: 30px; height: 30px;"
                    }
                    p { "Dashboard" }
                }
                button {
                    style: "background: none; border: none; text-align: center;",
                    img {
                        src: "https://path_to_savings_icon.com/savings.png",
                        style: "width: 30px; height: 30px;"
                    }
                    p { "Savings" }
                }
                button {
                    style: "background: none; border: none; text-align: center;",
                    img {
                        src: "https://path_to_payments_icon.com/payments.png",
                        style: "width: 30px; height: 30px;"
                    }
                    p { "Payments" }
                }
                button {
                    style: "background: none; border: none; text-align: center;",
                    img {
                        src: "https://path_to_loans_icon.com/loans.png",
                        style: "width: 30px; height: 30px;"
                    }
                    p { "Loans" }
                }
            }
        }
    })
}
