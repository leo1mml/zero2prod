use actix_web::HttpResponse;
use actix_web::web;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
