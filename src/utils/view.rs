use serde_json::json;
use serde::Serialize;

#[derive(Serialize)]
pub enum ToastType {
    Info,
    Warning,
    Error,
    Success,
}

pub fn create_toast_header(toast_type: ToastType, msg: &str) -> String {

        let header_data = json!({
            "AppToast": {
                "level": toast_type,
                "message": msg
            }
        });

        return header_data.to_string();
}