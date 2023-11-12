use askama_actix::Template;

///
/// Template Structs
///

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "register_form.html")]
pub struct RegisterForm {}
