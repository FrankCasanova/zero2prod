use std::{fmt::format, net::TcpListener};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::startup::run;
use zero2prod::configuration::{get_configuration, DatabaseSettings};


pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(
        &config.connection_string_without_db()
    )
        .await
        .expect("Failed to connect to posgres in configure database test function");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("failed to create database, on configure database function");


    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("failed to connect to posgres while migratin database on configure database function");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("failed to migrate database on configure database function");
    
    connection_pool
}

async fn spawn_app() -> TestApp {
    // Port 0 is special-cased at the OS leve: trying to bind
    // port 0 will trigger an OS scan for an available port
    // which will then be bound to the application.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let address =format!("http://127.0.0.1:{}", port);
    
    let mut configuration = get_configuration().expect("failed reading the configuration. it must be a file called configuration.yaml, check it out!");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;

    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    // Return the struct to the caller.
    TestApp {
        address,
        db_pool: connection_pool
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = "name=Frank%20Casanova&email=frankcasanova.test%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("failded to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
    let saved = sqlx::query("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    // assert_eq!(saved.email, "frankcasanova.test@gmail.com");
    // assert_eq!(saved.name, "Frank Casanova");

}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = [
        ("name=le%20guin", "missing the email"),
        ("email=loco_loco%40gmail.com", "missing the name"),
        ("", "missing both, name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to exxecute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail  with 400 Bad Request when the payload was {}",
            error_message
        );
    }
}
