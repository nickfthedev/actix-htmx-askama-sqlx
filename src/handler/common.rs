use actix_web::Responder;

// 404 not found handler
pub async fn not_found() -> impl Responder {
    format!("Actix -  404: Not found.")
}

