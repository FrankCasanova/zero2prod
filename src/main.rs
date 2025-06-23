// src/main.rs

use actix_web::{
    web,
    App,
    HttpRequest,
    HttpServer,
    Responder
};

// Handler function for incoming HTTP GET requests.
// It takes an HttpRequest as input and returns something that implements the Responder trait.
// This allows Actix Web to convert the return value into an HTTP response.
async fn greet(req: HttpRequest) -> impl Responder {
    // Attempt to extract the "name" parameter from the request's path.
    // If the parameter is not present (e.g., the path is just "/"), default to "world".
    let name = req.match_info().get("name").unwrap_or("world");
    // Format a greeting message using the extracted or default name.
    // The resulting String will be sent as the HTTP response body.
    format!("Hello {}", &name)
}


// The main entry point for the Actix Web application.
// The #[actix_web::main] macro sets up the async runtime required by Actix.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create and configure the HTTP server.
    // HttpServer::new takes a closure that returns an App instance for each worker thread.
    HttpServer::new(|| {
        // Construct a new Actix Web application.
        App::new()
            // Register a route for GET requests to the root path "/".
            // This will call the `greet` handler function.
            .route("/", web::get().to(greet))
            // Register a route for GET requests to "/{name}".
            // The `{name}` part is a path parameter, which will be passed to the `greet` handler.
            .route("/{name}", web::get().to(greet))
    })
    // Bind the server to the local address 127.0.0.1:8080.
    .bind(("127.0.0.1", 8080))?
    // Start the server and wait for it to complete.
    .run()
    .await
}