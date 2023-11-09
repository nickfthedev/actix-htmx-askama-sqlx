use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum ToastType {
    info,
    warning,
    error,
    success,
}

impl ToastType {
    fn as_str(&self) -> &'static str {
        match self {
            ToastType::info => "info",
            ToastType::error => "error",
            ToastType::warning => "warning",
            ToastType::success => "success",
        }
    }
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