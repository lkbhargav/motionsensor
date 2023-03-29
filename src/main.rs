use chrono::offset::Utc;
use chrono::DateTime;
use motionsensor::pir::PIR;
use raspicam::image::{
    camera_operations,
    settings::{CameraSettings, ImageSettings},
};
use std::fs;
use std::{process::Command, thread, time::Duration};

const GPIO_PIR: u8 = 21;

#[tokio::main]
async fn main() {
    let pir = PIR::new("BedroomSensor", GPIO_PIR);

    // Initialize camera settings with their default values.
    let camera_settings = CameraSettings::default();

    // Initialize image settings with their default values.
    let image_settings = ImageSettings::default();

    loop {
        if let Ok(detection_msg) = pir.receive() {
            let (detection_name, detection_time) = detection_msg;

            let datetime: DateTime<Utc> = detection_time.into();
            let datetime = format!("{}", datetime.format("%m/%d/%Y %T"));

            println!("detection happened, sensor: {detection_name}, time: {datetime:?} ");

            let datetime = datetime.replace(" ", "");
            let prefix = format!("~/{datetime}");

            fs::create_dir(prefix.clone()).expect("trying to create a directory");

            for i in 0..30 {
                let file_path_and_name = format!("{prefix}/{detection_name}-{i}.jpg");
                camera_settings.set_output(file_path_and_name.as_str());
                camera_operations::click_image(camera_settings.clone(), image_settings.clone());
            }
        }
    }
}
