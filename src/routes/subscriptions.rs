use actix_web::{HttpResponse, web};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

// To learn more about serde:
// https://www.joshmcguigan.com/blog/understanding-serde/
#[derive(serde::Deserialize)]
pub struct Formdata {
    name: String,
    email: String,
}

// Let's start simple: we always return a 200 OK
pub async fn subscribe(form: web::Form<Formdata>, pool: web::Data<PgPool>) -> HttpResponse {
    // logs are so important, it improve one foundamental aspect of backend field.
    // OBSERVABILITY AND MONITORING AND LOGGING
    // this is just our first log
    log::info!("Saving new subscriber details in the database");
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We use 'get_ref' to get an inmmutable reference to the 'PgConnection'
    // Wrapped by 'Web::Data'
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            // This is just our second log
            log::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // This is just our third log, is flaged as error.
            log::error!("Failed to execute query: {e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
