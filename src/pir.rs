use anyhow::Result;
use chrono::offset::Utc;
use chrono::DateTime;
use std::{
    sync::mpsc::{self, sync_channel, Receiver, SyncSender},
    thread,
    time::{self, Duration, SystemTime},
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

        // starting detector in the background
        task::spawn_blocking(move || sensor_bedroom.start_detector(receiver));

        PIR {
            rcvr: detections_channel_receiver,
        }
    }

    pub fn receive(&self) -> Result<(String, SystemTime)> {
        Ok(&self.rcvr.try_recv())
    }
}
