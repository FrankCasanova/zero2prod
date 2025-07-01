use actix_web::HttpResponse;

// We were returning 'impl Responder' at the very beginning.
// we are no spelling out the type explicitly given that we have
// become more familiar with 'actix-web'
// There is no perfomrance difference! just a stylistic choice :)
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}