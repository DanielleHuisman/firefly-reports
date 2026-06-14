use std::env;

use anyhow::{Context, Result, anyhow};
use firefly_iii::apis::{ApiClient, configuration::Configuration};
use secrecy::ExposeSecret;
use shield_dioxus_axum::{ExtractShield, UserRequired};
use shield_memory::User;
use shield_oauth::{OAUTH_METHOD_ID, OauthConnection};

pub async fn firefly_client(
    ExtractShield(shield): ExtractShield<User>,
    UserRequired(user): UserRequired<User>,
) -> Result<ApiClient> {
    let connections = shield
        .user_connections::<OauthConnection>(&user, OAUTH_METHOD_ID, Some("firefly"))
        .await
        .context("failed to get user connections")?;
    if connections.len() > 1 {
        return Err(anyhow!("multiple connections"));
    }
    let Some(connection) = connections.first() else {
        return Err(anyhow!("no connections"));
    };

    let client = ApiClient::new(
        Configuration {
            base_path: format!("{}/api", env::var("FIREFLY_URL")?),
            oauth_access_token: Some(connection.access_token.expose_secret().to_owned()),
            user_agent: Some("Firefly III Reports".to_owned()),
            ..Configuration::default()
        }
        .into(),
    );

    Ok(client)
}
