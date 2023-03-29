use anyhow::Result;
use pir_motion_sensor::sensor::motion::MotionSensor;
use std::{
    sync::mpsc::{self, sync_channel, Receiver, SyncSender},
    time::SystemTime,
};
use tokio::task;

pub struct PIR {
    rcvr: Receiver<(String, SystemTime)>,
}

impl PIR {
    pub fn new(name: &str, gpio_pin: u8) -> PIR {
        // channel for sensor data
        #[allow(clippy::type_complexity)]
        let (detections_channel_sender, detections_channel_receiver): (
            SyncSender<(String, SystemTime)>,
            Receiver<(String, SystemTime)>,
        ) = sync_channel(0);

        // sensor initialization - check README for more details about sensor parameters
        let mut sensor_bedroom = MotionSensor::new(
            String::from(name),        // name
            gpio_pin,                  // gpio PIN number
            100,                       // sensor refresh rate in miliseconds
            300,                       // sensor motion time period in miliseconds
            2,                         // sensor minimal triggering number
            detections_channel_sender, // channel where sensor thread will be sending detections
            None,                      // None for real GPIO usage, Some(Vec<u128>) for unit tests
        );

        let (_stop_command, receiver) = mpsc::channel();

        // starting detector in the background
        task::spawn_blocking(move || sensor_bedroom.start_detector(receiver));

        PIR {
            rcvr: detections_channel_receiver,
        }
    }

    pub fn receive(&self) -> Result<(String, SystemTime)> {
        Ok(self.rcvr.try_recv().clone()?)
    }
}
