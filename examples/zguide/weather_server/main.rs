#![crate_name = "weather_server"]

//! Weather update server
//! Binds PUB socket to tcp://*:5556 and ipc://weather.ipc
//! Publishes random weather updates

use rand::Rng;

fn main() {
    let context = zmq2::Context::new();
    let publisher = context.socket(zmq2::PUB).unwrap();

    assert!(publisher.bind("tcp://*:5556").is_ok());
    assert!(publisher.bind("ipc://weather.ipc").is_ok());

    let mut rng = rand::thread_rng();

    loop {
        let zipcode = rng.gen_range(0..100_000);
        let temperature = rng.gen_range(-80..135);
        let relhumidity = rng.gen_range(10..60);

        // this is slower than C because the current format! implementation is
        // very, very slow. Several orders of magnitude slower than glibc's
        // sprintf
        let update = format!("{:05} {} {}", zipcode, temperature, relhumidity);
        publisher.send(&update, 0).unwrap();
    }

    // note: destructors mean no explicit cleanup necessary
}
