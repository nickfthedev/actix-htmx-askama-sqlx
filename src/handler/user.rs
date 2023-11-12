//
use crate::AppState;
use crate::view::user::RegisterForm;
use actix_web::{ get, post, web, HttpResponse, Responder};
use rand::Rng;
use crate::model::user::AddUser;

use crate::utils::view::{create_toast_header, ToastType};
use crate::utils::form::{verify_email, filter_input, validate_string_alpha_space, hash_password};
use askama_actix::TemplateToResponse;

///
/// Handler functions
/// 
#[get("/register")]
async fn show_register() -> impl Responder {
    RegisterForm{}.to_response()
}

//
// Add new user
//
#[post("/user")]
async fn add_user(
    data: web::Data<AppState>,
    web::Form(user): web::Form<AddUser>,
) -> impl Responder {
    // Verify Email address 
    if !verify_email(user.email.as_str()) {
        return HttpResponse::BadRequest()
            .insert_header((
                "hx-trigger",
                create_toast_header(ToastType::Error, "Missing or invalid email address"),
            ))
            .body("Missing or invalid email address");
    }
    
    if !validate_string_alpha_space(user.name.as_str(), Some(3), Some(60)) {
        return HttpResponse::BadRequest()
            .insert_header((
                "hx-trigger",
                create_toast_header(ToastType::Error, "Only alphabetical names allowed with mininimum 3 characters and max 60 characters"),
            ))
            .body("Only alphabetical names allowed with mininimum 3 characters and max 60 characters");
    }
    // Hash password
    let password = hash_password(&user.password);
    if let Err(false) = password {
        return HttpResponse::BadRequest()
            .insert_header((
                "hx-trigger",
                create_toast_header(ToastType::Error, "Invalid password"),
            ))
            .body("Invalid password");
    }
    let password = password.unwrap();

    // Generate Username
    let random_number = rand::thread_rng().gen_range(1000..9999);
    let username = format!("{}{}", filter_input(&user.name), random_number);

    // Query Insert
    let add_query = sqlx::query!(r#"INSERT INTO "user" (email,name,password,username) VALUES ($1,$2,$3,$4)"#, filter_input(&user.email), filter_input(&user.name), password,username);
    add_query.execute(&data.pool).await.unwrap();

    return HttpResponse::Accepted()
        .insert_header((
            "hx-trigger",
            create_toast_header(ToastType::Success, "User created"),
        ))
        .body("User added");
}
