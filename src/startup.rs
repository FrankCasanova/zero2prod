use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use sqlx::PgConnection;
use std::net::TcpListener;


pub fn run(
    listener: TcpListener,
    connection: PgConnection
) -> Result<Server, std::io::Error> {

    // Wrap the connection in a smart pointer cos the connection
    // is not clonable, we need an Atomic Reference Counter 
    // in order to allow al threads use that connection carefully
    let connection = web::Data::new(connection);

    // HttpServer does not take App as argument - it wanats a clousure
    // that returns an App struct.
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))

            // Register the connection as part of the applications state
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}