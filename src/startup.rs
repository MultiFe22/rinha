use crate::configuration::DatabaseSettings;
use crate::configuration::Settings;
use crate::routes::get_extrato;
use crate::routes::health_check;
use crate::routes::register_transacao;
// use actix_session::storage::RedisSessionStore;
// use actix_session::SessionMiddleware;
// use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
// use actix_web_flash_messages::storage::CookieMessageStore;
// use actix_web_flash_messages::FlashMessagesFramework;
// use actix_web_lab::middleware::from_fn;
// use secrecy::ExposeSecret;
// use secrecy::Secret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
// use tracing_actix_web::TracingLogger;

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async // without having to use any proc-macro incantation.
async fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, anyhow::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        // Wrap the connection in a smart pointer
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route(
                "/clientes/{id}/transacoes",
                web::post().to(register_transacao),
            )
            .route(
                "/clientes/{id}/extrato",
                web::get().to(get_extrato))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

// We need to define a wrapper type in order to retrieve the URL
// in the `subscribe` handler.
// Retrieval from the context, in actix-web, is type-based: using
// a raw `String` would expose us to conflicts.
pub struct ApplicationBaseUrl(pub String);

pub struct Application {
    pub port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        // Now the port is read from the configuration file.
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            connection_pool,
        )
        .await?;
        // We "save" the bound port in one of `Application`'s fields
        Ok(Self { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .max_connections(400)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}
