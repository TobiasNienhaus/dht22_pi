extern crate dht22_pi;

use dht22_pi::{Sensor, read};

pub fn main() {
    let result = read(Sensor::Dht22, 14);

    println!("{:?}", result);
}
