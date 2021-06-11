use std::{
    sync::{mpsc::channel, Arc, Mutex},
    time::Duration,
};

use mio::{net::TcpListener, Events, Interest, Poll, Token};

mod connection;
mod pool;

use connection::Connection;
use pool::ThreadPool;

fn main() {
    // Bind and register the TcpListener in MIO to detect when is readable (new
    // connection).
    let address = "0.0.0.0:9000";
    let mut server = TcpListener::bind(address.parse().unwrap()).unwrap();

    let mut poll = Poll::new().unwrap();
    poll.registry()
        .register(&mut server, Token(0), Interest::READABLE)
        .unwrap();

    // Create some threads, and make them wait for work, but also they should be
    // able to communicate when a connections needs to be register again
    let (job_tx, job_rx) = channel::<Connection>();
    let job_rx = Arc::new(Mutex::new(job_rx));

    let (registry_tx, registry_rx) = channel::<Connection>();

    let mut pool = ThreadPool::new(4);
    for _ in 0..pool.size() {
        let job_rx = job_rx.clone();
        let registry_tx = registry_tx.clone();

        pool.submit(move || loop {
            let mut connection = job_rx.lock().unwrap().recv().unwrap();

            connection.read();

            // Process
            let msg = connection.to_send.to_owned();
            connection.set(&msg);

            connection.write();
            registry_tx.send(connection).unwrap();
        });
    }

    // Let

    let mut events = Events::with_capacity(1024);
    loop {
        poll.poll(&mut events, Some(Duration::new(1, 0))).unwrap();
        for event in events.iter() {
            match event.token() {
                Token(0) => loop {
                    match server.accept() {
                        Ok((socket, addr)) => {}
                        Err(_) => break,
                    }
                },
                token if event.is_readable() => {}
                token if event.is_writable() => {}
                _ => unreachable!(),
            }
        }
    }
}
