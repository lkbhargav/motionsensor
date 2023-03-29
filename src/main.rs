use chrono::offset::Utc;
use chrono::DateTime;
use email::{Email, Relay};
use motionsensor::db::DB;
use motionsensor::environment::EnvironmentVariables;
use motionsensor::pir::PIR;
use std::thread;
use std::time::{Duration, SystemTime};
use std::{fs, process::Command};

const GPIO_PIR: u8 = 21;
const EMAIL_FROM: &str = "MotionSensor <bhargav.lakkur@gmail.com>";
const TO_ADDRESS: &str = "Bhargav Lakkur <lkbhargav9@gmail.com>";

#[tokio::main]
async fn main() {
    let vars = EnvironmentVariables::init().expect("error initializing env vars");
    let mut db = DB::init(&vars.db_path).expect("error initializing DB instance");
    let pir = PIR::new(&vars.app_name, GPIO_PIR);
    let gmail = Email::new(
        EMAIL_FROM,
        EMAIL_FROM,
        &vars.gmail.username,
        &vars.gmail.password,
        Relay::Gmail,
    )
    .expect("error initializing email service");

    let mut last_image_capture_time = SystemTime::now();

    loop {
        if let Ok(detection_msg) = pir.receive() {
            let (detection_name, detection_time) = detection_msg;

            // only consider it as a valid motion if detected_time is greater than last_image_capture_time
            if detection_time <= last_image_capture_time {
                continue;
            }

            let mut prefix = String::new();

            if vars.capture_image {
                let datetime: DateTime<Utc> = detection_time.into();
                let datetime = format!("{}", datetime.format("%m/%d/%Y %T"));

                let datetime_instance_folder = datetime
                    .replace(" ", "T")
                    .replace(":", "-")
                    .replace("/", "-");

                prefix = format!("{}/{datetime_instance_folder}", vars.images_path);

                fs::create_dir(prefix.clone()).expect("trying to create a directory");
            }

            let res = db.log(&prefix);

            if res.is_err() {
                println!("error trying to log a record in DB");
            }

            if vars.email_alert {
                let tmp = format!(
                    "Motion detected and {} images are being collected to {}",
                    vars.number_of_images_to_capture, prefix
                );

                let mut message = tmp.as_str();

                if !vars.capture_image {
                    message = "Motion detected in the room!";
                }

                let res = gmail.send(TO_ADDRESS, "Motion detected", message);

                if res.is_err() {
                    println!("error sending email: {}", res.err().unwrap());
                }
            }

            if vars.capture_image {
                for i in 0..vars.number_of_images_to_capture {
                    let file_name = format!("{prefix}/{detection_name}-{i}.jpg");

                    match Command::new("/usr/bin/raspistill")
                        .arg("-rot")
                        .arg("180")
                        .arg("-o")
                        .arg(&file_name)
                        .output()
                    {
                        Ok(_r) => continue,
                        Err(e) => println!("error trying to capture image: {e}"),
                    }
                }
            } else {
                thread::sleep(Duration::from_secs(5));
            }

            last_image_capture_time = SystemTime::now();
        }
    }
}
