use dioxus::prelude::*;

use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use shield_dioxus_axum::UserRequired;

#[derive(Deserialize, PartialEq, Serialize)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub email_addresses: Vec<String>,
}

#[get("/api/user", user: UserRequired<shield_memory::User>)]
pub async fn user() -> Result<User, ServerFnError> {
    use shield::User as _;

    let UserRequired(user) = user;

    Ok(User {
        id: user.id(),
        name: user.name(),
        email_addresses: user
            .email_addresses()
            .await
            .context("failed to get user email addresses")?
            .into_iter()
            .map(|email_address| email_address.email)
            .collect(),
    })
}
