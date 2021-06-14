use std::{
    collections::HashMap,
    sync::{mpsc::channel, Arc, Mutex},
    time::Duration,
};

use mio::{net::TcpListener, Events, Interest, Poll, Token};

mod connection;
mod pool;

use connection::Connection;
use pool::ThreadPool;

fn main() {
    // Bind and register the TcpListener in MIO to detect when is readable, a
    // new connection.
    let address = "0.0.0.0:1984";
    let mut server = TcpListener::bind(address.parse().unwrap()).unwrap();

    let mut poll = Poll::new().unwrap();
    poll.registry()
        .register(&mut server, Token(0), Interest::READABLE)
        .unwrap();

    // We are gonna use channels to communicate with the thread pool and to
    // re-register connections in Mio.
    let (work_tx, work_rx) = channel::<Connection>();
    let work_rx = Arc::new(Mutex::new(work_rx));
    let (poll_tx, poll_rx) = channel::<Connection>();

    // The thread pool is gonna handle connections reading and writting.
    let mut pool = ThreadPool::new(4);
    for _ in 0..pool.size() {
        let job_rx = work_rx.clone();
        let poll_tx = poll_tx.clone();

        pool.submit(move || loop {
            let mut connection = job_rx.lock().unwrap().recv().unwrap();

            connection.read();

            // Process
            let msg = connection.to_send.to_owned();
            connection.set(&msg);

            connection.write();

            poll_tx.send(connection).unwrap();
        });
    }

    // Mio event detection.
    let mut counter: usize = 0;
    let mut connections: HashMap<Token, Connection> = HashMap::new();
    let mut events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, Some(Duration::new(1, 0))).unwrap();

        // New connection? Readable? Writeable? Send
        for event in events.iter() {
            match event.token() {
                Token(0) => loop {
                    match server.accept() {
                        Ok((mut socket, _)) => {
                            counter += 1;
                            let token = Token(counter);

                            poll.registry()
                                .register(&mut socket, token, Interest::READABLE)
                                .unwrap();

                            connections.insert(token, Connection::new(token, socket));
                        }
                        Err(_) => break,
                    }
                },

                token if event.is_readable() => {
                    if let Some(connection) = connections.remove(&token) {
                        work_tx.send(connection).unwrap();
                    }
                }

                token if event.is_writable() => {
                    if let Some(connection) = connections.remove(&token) {
                        work_tx.send(connection).unwrap();
                    }
                }
                _ => unreachable!(),
            }
        }

        // When the thread pool is done processing the connection, we need to
        // reregister it with Mio.
        loop {
            match poll_rx.try_recv() {
                Ok(connection) if !connection.is_open => {}
                Ok(mut connection) => {
                    if connection.to_send.len() > 0 {
                        poll.registry()
                            .reregister(
                                &mut connection.socket,
                                connection.token,
                                Interest::WRITABLE,
                            )
                            .unwrap();
                    } else {
                        poll.registry()
                            .reregister(
                                &mut connection.socket,
                                connection.token,
                                Interest::READABLE,
                            )
                            .unwrap();
                    }
                }
                _ => break,
            }
        }
    }
}
