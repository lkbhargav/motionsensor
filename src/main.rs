use chrono::offset::Utc;
use chrono::DateTime;
use motionsensor::pir::PIR;
use raspicam::image::{
    camera_operations,
    settings::{CameraSettings, ImageSettings},
};
use std::{fs, process::Command};

const GPIO_PIR: u8 = 21;

#[tokio::main]
async fn main() {
    let pir = PIR::new("BedroomSensor", GPIO_PIR);

    // Initialize camera settings with their default values.
    let mut camera_settings = CameraSettings::default();

    // Initialize image settings with their default values.
    let image_settings = ImageSettings::default();

    println!("1");

    match Command::new("/usr/bin/raspistill")
        .arg("-o")
        .arg("/var/log/images/abcd.jpg")
        .output()
    {
        Ok(_r) => println!("Ok"),
        Err(e) => println!("error trying to capture image: {e}"),
    }

    println!("2");

    // loop {
    //     if let Ok(detection_msg) = pir.receive() {
    //         let (detection_name, detection_time) = detection_msg;

    //         let datetime: DateTime<Utc> = detection_time.into();
    //         let datetime = format!("{}", datetime.format("%m/%d/%Y %T"));

    //         println!("detection happened, sensor: {detection_name}, time: {datetime:?} ");

    //         let datetime = datetime.replace(" ", "").replace(":", "").replace("/", "");
    //         let prefix = format!("/var/log/images/{datetime}");

    //         fs::create_dir(prefix.clone()).expect("trying to create a directory");

    //         for i in 0..30 {
    //             println!("In here");
    //             camera_settings.set_output(format!("{prefix}/{detection_name}-{i}.jpg"));

    //             match camera_operations::click_image(
    //                 camera_settings.clone(),
    //                 image_settings.clone(),
    //             ) {
    //                 Ok(_r) => continue,
    //                 Err(e) => println!("error trying to capture image: {e}"),
    //             }
    //         }
    //     }
    // }
}
