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
    let request_id = Uuid::new_v4();

    // Spans, like logs, have an associated level
    // 'info_span' creates a span at the info level
    let request_span = tracing::info_span!(
     "\n
         Adding a new subscriber
         \n",
     %request_id, 
     subscriber_email = %form.email,
     subscriber_name = %form.name
    );

    // Using 'enter' in async function is a recipe for disaster
    // bear with me for now, but dont do this at home.
    // See the following section on 'Instrumenting Futures'
    let _request_span_guard = request_span.enter();

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
            tracing::error!(
                "\n
                Request ID: {} \n
                Failed to execute query: {e:?}\n",
                request_id
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
