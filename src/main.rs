#![feature(generators, generator_trait)]

extern crate osaka;
extern crate carrier;

use carrier::{error::Error, conduit::Conduit};
use osaka::{osaka};
use std::env;
use std::time::{Duration};


#[osaka]
pub fn main() -> Result<(), Error> {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let poll = osaka::Poll::new();
    let mut conduit = Conduit::new(poll,"<YOUR SHADOW ADDRESS>".parse().unwrap(),);
    let mut conduit = osaka::sync!(conduit)?;

    conduit.schedule(
        Duration::from_secs(10),
        carrier::headers::Headers::with_path("/v1/sysinfo"),
        |identity: &carrier::identity::Identity, msg: carrier::proto::Sysinfo| {
            println!("{} => {:#?}", identity, msg);
        }
    );

    osaka::sync!(conduit)?;

    Ok(())
}

