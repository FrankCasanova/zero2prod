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
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            eprintln!("Failed to execute query: {e:?}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
