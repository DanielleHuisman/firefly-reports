mod app;
mod components;
mod views;

use dioxus::prelude::*;
use shield_bootstrap::BootstrapDioxusStyle;

use crate::app::App;

#[cfg(not(feature = "server"))]
fn main() {
    LaunchBuilder::new()
        .with_context(BootstrapDioxusStyle::default().context())
        .launch(App)
}

#[cfg(feature = "server")]
fn main() {
    dioxus::serve(|| async move {
        use std::{env, sync::Arc};

        use axum::{Extension, Router};
        use dioxus::{cli_config::fullstack_address_or_localhost, prelude::DioxusRouterExt};
        use shield::{Shield, ShieldOptions};
        use shield_dioxus_axum::{AuthRoutes, AxumDioxusIntegration, ShieldLayer};
        use shield_memory::{MemoryStorage, User};
        use shield_oauth::{OauthMethod, OauthProvider};
        use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration};

        let firefly_url = env::var("FIREFLY_URL")?;
        let firefly_client_id = env::var("FIREFLY_CLIENT_ID")?;
        let firefly_client_secret = env::var("FIREFLY_CLIENT_SECRET")?;
        let firefly_redirect_url = env::var("FIREFLY_REDIRECT_URL").ok();

        let addr = fullstack_address_or_localhost();

        // Initialize sessions
        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::minutes(10)));

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
                App,
            )
            .layer(Extension(
                AxumDioxusIntegration::<User>::default().context(),
            ))
            .layer(shield_layer)
            .layer(session_layer);

        Ok(router)
    });
}
