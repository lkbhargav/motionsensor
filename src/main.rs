use pir_motion_sensor::sensor::motion::MotionSensor;
use rascam::*;
use std::{
    sync::mpsc::{self, sync_channel, Receiver, SyncSender},
    time::SystemTime,
};
use tokio::task;

const GPIO_PIR: u8 = 21;

#[tokio::main]
async fn main() {
    // channel for sensor data
    #[allow(clippy::type_complexity)]
    let (detections_channel_sender, detections_channel_receiver): (
        SyncSender<(String, SystemTime)>,
        Receiver<(String, SystemTime)>,
    ) = sync_channel(0);

    // sensor initialization - check README for more details about sensor parameters
    let mut sensor_bedroom = MotionSensor::new(
        String::from("SensorBedroom"), // name
        GPIO_PIR,                      // gpio PIN number
        100,                           // sensor refresh rate in miliseconds
        300,                           // sensor motion time period in miliseconds
        2,                             // sensor minimal triggering number
        detections_channel_sender,     // channel where sensor thread will be sending detections
        None,                          // None for real GPIO usage, Some(Vec<u128>) for unit tests
    );

    // this is for sending stop requests for motion sensor thread
    let (_stop_command, receiver) = mpsc::channel();

    // starting detector in the background
    task::spawn_blocking(move || sensor_bedroom.start_detector(receiver));

    let info = info().unwrap();
    if info.cameras.len() < 1 {
        error!("Found 0 cameras. Exiting");
        // note that this doesn't run destructors
        ::std::process::exit(1);
    }
    info!("{}", info);

    simple_sync(&info.cameras[0]);

    loop {
        if let Ok(detection_msg) = detections_channel_receiver.try_recv() {
            // detection received
            // each "valid" detection constains sensor name and time of detection as SystemTime()
            let (detection_name, detection_time) = detection_msg;

            let datetime: DateTime<Utc> = detection_time.into();
            let datetime = format!("{}", datetime.format("%m/%d/%Y %T"));

            println!("detection happened, sensor: {detection_name}, time: {datetime:?} ");

            // TODO: trigger camera to take picture
        }
    }
}

fn simple_sync(info: &CameraInfo) {
    let mut camera = SimpleCamera::new(info.clone()).unwrap();
    camera.activate().unwrap();

    let sleep_duration = time::Duration::from_millis(2000);
    thread::sleep(sleep_duration);

    let b = camera.take_one().unwrap();
    File::create("image.jpg").unwrap().write_all(&b).unwrap();

    info!("Saved image as image.jpg");
}
