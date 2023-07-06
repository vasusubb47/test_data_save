// use serde_json;
use serde::{Deserialize, Serialize};
// use std::fs::File;

use chrono::{offset::Utc, Timelike};
use std::{
    fs::File,
    io::{Error, ErrorKind, Result, Write},
    thread,
    time::Duration,
};

mod utils;
use utils::{arrays, get_nanosec, get_rand, Nanosec};

trait MPU60X0Data {
    fn save_data(&self, file_name: &str) -> Result<()>;

    fn load_data(&self, file_name: &str) -> Result<()>;
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Default)]
struct Acceleration {
    t: Nanosec, //
    x: f32,
    y: f32,
    z: f32,
}

impl Acceleration {
    fn new() -> Acceleration {
        Acceleration {
            t: get_nanosec(),
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    fn save_data(&mut self, t: u32, x: f32, y: f32, z: f32) {
        self.t = t;
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Default)]
struct Gyroscope {
    t: Nanosec,
    x: f32,
    y: f32,
    z: f32,
}

impl Gyroscope {
    fn new() -> Gyroscope {
        Gyroscope {
            t: get_nanosec(),
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    fn save_data(&mut self, t: u32, x: f32, y: f32, z: f32) {
        self.t = t;
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

const NUMBER_OF_ENTRIES_PER_SECOND: u32 = 30;
// 5 entries every second for 1 minute
const NUMBER_OF_ENTRIES_PER_MIN: usize = (NUMBER_OF_ENTRIES_PER_SECOND * 60) as usize;
const SLEEP_MILISEC: u32 = 1000 / NUMBER_OF_ENTRIES_PER_SECOND;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct Data {
    // nanoseconds since EPOCH
    start_time: u32,
    #[serde(with = "arrays")]
    accelerations: [Acceleration; NUMBER_OF_ENTRIES_PER_MIN],
    #[serde(with = "arrays")]
    gyroscope_angles: [Gyroscope; NUMBER_OF_ENTRIES_PER_MIN],
}

impl MPU60X0Data for Data {
    fn save_data(&self, file_name: &str) -> Result<()> {
        if utils::path_exists(&file_name) {
            Error::new(
                ErrorKind::AlreadyExists,
                "The specified file exist and cannot be mofiled.",
            );
        }
        let serilized_data = serde_json::to_string(self);
        match serilized_data {
            Ok(data) => {
                // println!("data: {}", data);

                let mut file = File::create(file_name)?;
                file.write_all(data.as_bytes())
            }
            Err(err) => Err(Error::new(err.io_error_kind().unwrap(), err.to_string())),
        }
    }

    fn load_data(&self, file_name: &str) -> Result<()> {
        todo!()
    }
}

impl Data {
    fn simulate(&mut self) {
        for ind in 0..NUMBER_OF_ENTRIES_PER_MIN {
            let t = get_nanosec();

            self.accelerations[ind].save_data(t, get_rand(), get_rand(), get_rand());
            self.gyroscope_angles[ind].save_data(t, get_rand(), get_rand(), get_rand());

            thread::sleep(Duration::from_millis(SLEEP_MILISEC as u64));
            // println!("hello : {}", ind);
        }
    }
}

fn main() {
    let mut data = Data {
        start_time: Utc::now().time().nanosecond(),
        accelerations: [Acceleration::new(); NUMBER_OF_ENTRIES_PER_MIN],
        gyroscope_angles: [Gyroscope::new(); NUMBER_OF_ENTRIES_PER_MIN],
    };

    data.simulate();

    data.save_data(&format!("data/test{}.json", data.start_time))
        .unwrap();
}
