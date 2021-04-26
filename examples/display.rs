extern crate dht11_22_pi;

use dht11_22_pi::{Sensor, read};

pub fn main() {
    let result = read(Sensor::Dht22, 14);

    println!("{:?}", result);
}
