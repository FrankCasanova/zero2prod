use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use env_logger::Env;
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // 'init' does call 'set_logger', so this is all we need to do.
    // We are falling back to printing all logs at info-level or above
    // if the RUST_LOG enviroment variable has not been set.
    // Something useful mentioned in the bookk is that
    // using:
    // RUST_LOG=info cargo run
    // RUST_LOG=debug cargo run
    // RUST_LOG=trace cargo run
    // RUST_LOG=error cargo run
    // RUST_LOG=warn cargo run
    // we can get different levels of logs
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Wrap the connection in a smart pointer cos the connection
    // is not clonable, we need an Atomic Reference Counter
    // in order to allow al threads use that connection carefully
    let db_pool = web::Data::new(db_pool);

    // HttpServer does not take App as argument - it wanats a clousure
    // that returns an App struct.
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            // Register the connection as part of the applications state
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
