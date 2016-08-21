extern crate futures;
extern crate tokio;
extern crate tokio_line as line;
extern crate env_logger;
extern crate mio;

use futures::Future;
use mio::timer::Builder as MioTimerBuilder;
use std::borrow::Cow;
use std::io;
use std::time::Duration;
use tokio::Service;
use tokio::io::{Transport, Readiness};
use tokio::proto::pipeline;
use tokio::util::timer::{Timer, Timeout};


pub fn main() {
    env_logger::init().unwrap();

    let addr = "127.0.0.1:12345".parse().unwrap();

    let server = line::Server::new()
        .bind(addr)
        .serve(tokio::simple_service(|msg| {
            println!("GOT: {:?}", msg);
            Ok(msg)
        }))
        .unwrap();
}

// Why this isn't in futures-rs, I do not know...
fn await<T: Future>(f: T) -> Result<T::Item, T::Error> {
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();

    f.then(move |res| {
        tx.send(res).unwrap();
        Ok::<(), ()>(())
    }).forget();

    rx.recv().unwrap()
}
