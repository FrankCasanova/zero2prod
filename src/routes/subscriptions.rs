use actix_web::{web, HttpResponse};

// To learn more about serde:
// https://www.joshmcguigan.com/blog/understanding-serde/
#[derive(serde::Deserialize)]
pub struct Formdata {
    name: String,
    email: String,
}

// Let's start simple: we always return a 200 OK
pub async fn subscribe(_form: web::Form<Formdata>) -> HttpResponse {
    HttpResponse::Ok().finish()
}