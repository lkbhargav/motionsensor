use anyhow::Result;
use std::env;

#[derive(Debug, Clone)]
pub struct Gmail {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct EnvironmentVariables {
    pub gmail: Gmail,
    pub email_alert: bool,
    pub db_path: String,
    pub images_path: String,
    pub number_of_images_to_capture: u16,
    pub app_name: String,
    pub capture_image: bool,
}

macro_rules! parse_env {
    ($key:expr, $mess:expr) => {
        env::var($key).expect($mess)
    };
}

impl EnvironmentVariables {
    pub fn init() -> Result<Self> {
        let username = parse_env!("MS_USERNAME", "MS_USERNAME is required but not found");
        let password = parse_env!(
            "MS_APP_PASSWORD",
            "MS_APP_PASSWORD is required but not found"
        );
        let email_alert = parse_env!("MS_EMAIL_ALERT", "MS_EMAIL_ALERT is required but not found");
        let email_alert = email_alert.parse::<bool>()?;
        let db_path = parse_env!("MS_DB_PATH", "MS_DB_PATH is required but not found");
        let images_path = parse_env!("MS_IMAGES_PATH", "MS_IMAGES_PATH is required but not found");
        let number_of_images_to_capture = parse_env!(
            "MS_NUMBER_OF_IMAGES_TO_CAPTURE",
            "MS_NUMBER_OF_IMAGES_TO_CAPTURE is required but not found"
        );
        let number_of_images_to_capture = number_of_images_to_capture.parse::<u16>()?;
        let app_name = parse_env!("MS_APP_NAME", "MS_APP_NAME is required but not found");
        let capture_image = parse_env!(
            "MS_CAPTURE_IMAGE",
            "MS_CAPTURE_IMAGE is required but not found"
        );
        let capture_image = capture_image.parse::<bool>()?;

        Ok(EnvironmentVariables {
            gmail: Gmail { username, password },
            email_alert,
            db_path,
            images_path,
            number_of_images_to_capture,
            app_name,
            capture_image,
        })
    }
}
