use api::{error, AppState, Database};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use open_ai_api::OpenAiClient;
use pbkdf2::password_hash::rand_core::OsRng;
use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};
use secrecy::{ExposeSecret, Secret};
use std::convert::From;
use std::net::Ipv4Addr;
use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use surrealdb::Surreal;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
// static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub struct Config {
    pub open_api_key: Secret<String>,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            open_api_key: Secret::new(
                std::env::var("OPEN_AI_KEY").expect("Failed to get OpenAI key."),
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), error::ApiError> {
    let Config { open_api_key } = Config::new();

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // axum logs rejections from built-in extractors with the `axum::rejection`
        // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
        "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace,parelthon_server=debug,error,info".into()
    });

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port: u16 = 1690;

    // 0.0.0.0: This IP address is a way to specify that the socket should bind to all available network interfaces on
    // the host machine. It's a common choice when you want your service to be reachable from outside networks.
    // let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let addr = SocketAddr::from((Ipv4Addr::LOCALHOST, port));

    tracing::debug!("listening on {}", addr);

    let db = Database::init().await.expect("Database not started");

    let random = ChaCha8Rng::seed_from_u64(OsRng.next_u64());

    let open_ai_client = OpenAiClient::new(open_api_key);

    let router = AppState::new(db, Arc::new(Mutex::new(random)), open_ai_client)
        .router()
        .await?;

    let server = axum::Server::bind(&addr)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    Ok(())
}
