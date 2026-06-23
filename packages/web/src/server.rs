use std::{env, sync::Arc};

use anyhow::{Result, anyhow};
use axum::{Extension, Router};
use base64::{Engine, prelude::BASE64_STANDARD};
use dioxus::{
    cli_config::fullstack_address_or_localhost,
    prelude::{DioxusRouterExt, Element, ServeConfig},
};
use shield::{Shield, ShieldOptions};
use shield_bootstrap::BootstrapDioxusStyle;
use shield_dioxus_axum::{AuthRoutes, AxumDioxusIntegration, ShieldLayer};
use shield_memory::{MemoryStorage, User};
use shield_oauth::{OauthMethod, OauthProvider};
use tower_sessions::{
    Expiry, MemoryStore, SessionManagerLayer,
    cookie::{Key, SameSite, time::Duration},
};

pub async fn server(app: fn() -> Element) -> Result<Router> {
    let encoded_cookie_key = env::var("COOKIE_KEY").map_err(|_| {
            anyhow!("Missing `COOKIE_KEY`. Run `cargo run -p scripts --bin generate-cookie-key` to generate one.")
        })?;
    let cookie_key = match BASE64_STANDARD.decode(&encoded_cookie_key) {
        Ok(cookie_key) => cookie_key,
        Err(err) => {
            return Err(anyhow!(
                "Invalid `COOKIE_KEY` value `{encoded_cookie_key}`: {err} Run `cargo run -p scripts --bin generate-cookie-key` to generate one."
            ));
        }
    };
    let cookie_key = Key::try_from(cookie_key.as_slice())?;

    let firefly_url = env::var("FIREFLY_URL")?;
    let firefly_client_id = env::var("FIREFLY_CLIENT_ID")?;
    let firefly_client_secret = env::var("FIREFLY_CLIENT_SECRET")?;
    let firefly_redirect_url = env::var("FIREFLY_REDIRECT_URL").ok();

    let addr = fullstack_address_or_localhost();

    // Initialize sessions
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(60)))
        .with_private(cookie_key)
        .with_same_site(SameSite::Lax)
        .with_secure(
            firefly_redirect_url
                .as_ref()
                .is_some_and(|redirect_url| redirect_url.starts_with("https://")),
        );

    // Initialize Shield
    let storage = MemoryStorage::new();
    let shield = Shield::new(
        storage.clone(),
        vec![Arc::new(
            OauthMethod::new(storage).with_providers([OauthProvider::builder()
                .id("firefly")
                .name("Firefly III")
                .client_id(firefly_client_id)
                .client_secret(firefly_client_secret)
                .authorization_url(format!("{firefly_url}/oauth/authorize"))
                .token_url(format!("{firefly_url}/oauth/token"))
                .user_url(format!("{firefly_url}/api/v1/about/user"))
                .user_path("data")
                .user_id_path("id")
                .user_email_path("attributes.email")
                .user_name_path("attributes.name")
                .redirect_url(firefly_redirect_url.unwrap_or_else(|| {
                    format!(
                        "http://localhost:{}/api/auth/sign-in-callback/oauth/firefly",
                        dioxus::cli_config::devserver_raw_addr()
                            .map(|addr| addr.port())
                            .unwrap_or_else(|| addr.port())
                    )
                }))
                .icon_url("https://docs.firefly-iii.org/images/logo.png")
                .build()]),
        )],
        ShieldOptions::default(),
    );
    let shield_layer = ShieldLayer::new(shield.clone());

    // Initialize router
    let router = Router::new()
        .nest("/api/auth", AuthRoutes::new(shield).router())
        .serve_dioxus_application(
            ServeConfig::new().context(BootstrapDioxusStyle::default().context()),
            app,
        )
        .layer(Extension(
            AxumDioxusIntegration::<User>::default().context(),
        ))
        .layer(shield_layer)
        .layer(session_layer);

    Ok(router)
}
