use chrono::offset::Utc;
use chrono::DateTime;
use motionsensor::pir::PIR;
use std::{fs, process::Command};

const GPIO_PIR: u8 = 21;

#[tokio::main]
async fn main() {
    let pir = PIR::new("BedroomSensor", GPIO_PIR);

    loop {
        if let Ok(detection_msg) = pir.receive() {
            let (detection_name, detection_time) = detection_msg;

            let datetime: DateTime<Utc> = detection_time.into();
            let datetime = format!("{}", datetime.format("%m/%d/%Y %T"));

            println!("detection happened, sensor: {detection_name}, time: {datetime:?} ");

            let datetime = datetime.replace(" ", "").replace(":", "").replace("/", "");
            let prefix = format!("/var/log/images/{datetime}");

            fs::create_dir(prefix.clone()).expect("trying to create a directory");

            for i in 0..30 {
                match Command::new("/usr/bin/raspistill")
                    .arg("-o")
                    .arg(&format!("{prefix}/{detection_name}-{i}.jpg"))
                    .output()
                {
                    Ok(_r) => println!("Ok"),
                    Err(e) => println!("error trying to capture image: {e}"),
                }
            }
        }
    }
}
