use motionsensor::pir::PIR;
use std::{
    sync::mpsc::{self, sync_channel, Receiver, SyncSender},
    thread,
    time::{self, Duration, SystemTime},
};
use tokio::task;

const GPIO_PIR: u8 = 21;

#[tokio::main]
async fn main() {
    // channel for sensor data
    // #[allow(clippy::type_complexity)]
    // let (detections_channel_sender, detections_channel_receiver): (
    //     SyncSender<(String, SystemTime)>,
    //     Receiver<(String, SystemTime)>,
    // ) = sync_channel(0);

    // // this is for sending stop requests for motion sensor thread
    // let (_stop_command, receiver) = mpsc::channel();

    // // starting detector in the background
    // task::spawn_blocking(move || sensor_bedroom.start_detector(receiver));

    // loop {
    //     if let Ok(detection_msg) = detections_channel_receiver.try_recv() {
    //         // detection received
    //         // each "valid" detection constains sensor name and time of detection as SystemTime()
    //         let (detection_name, detection_time) = detection_msg;

    //         let datetime: DateTime<Utc> = detection_time.into();
    //         let datetime = format!("{}", datetime.format("%m/%d/%Y %T"));

    //         println!("detection happened, sensor: {detection_name}, time: {datetime:?} ");

    //         // TODO: trigger camera to take picture

    //         thread::sleep(Duration::from_secs(1));
    //     }
    // }

    let pir = PIR::new("BedroomSensor", GPIO_PIR);

    loop {
        if let Ok(detection_msg) = pir.receive() {
            let (detection_name, detection_time) = detection_msg;

            let datetime: DateTime<Utc> = detection_time.into();
            let datetime = format!("{}", datetime.format("%m/%d/%Y %T"));

            println!("detection happened, sensor: {detection_name}, time: {datetime:?} ");

            // TODO: trigger camera to take picture

            thread::sleep(Duration::from_secs(1));
        }
    }
}
