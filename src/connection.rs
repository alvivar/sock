use std::{
    io::{Read, Write},
    net::TcpStream,
};

use mio::Token;

struct Connection {
    token: Token,
    socket: TcpStream,
    is_open: bool,
    received: String,
    to_send: String,
}

impl Connection {
    fn new(token: Token, socket: TcpStream) -> Connection {
        Connection {
            token,
            socket,
            is_open: true,
            received: String::new(),
            to_send: String::new(),
        }
    }

    fn read(&mut self) {
        loop {
            let read = self.socket.read_to_string(&mut self.received);

            match read {
                Ok(0) => {
                    println!("Connection closed. 0 bytes received.");
                    self.is_open = false;
                }
                Ok(n) => {
                    println!("Data received. {} bytes received.", n);
                }
                // @todo What's ref doing? Maybe a temp var?
                Err(ref e) if would_block(e) => {
                    println!("Failed to read. Would block: {}", e);
                    break;
                }
                Err(e) => {
                    println!("Failed to read: {}", e);
                    break;
                }
            }
        }
    }

    fn write(&mut self) {
        match self.socket.write_all(self.to_send.as_bytes()) {
            Ok(_) => (),
            Err(_) => {
                self.is_open = false;
                return;
            }
        }

        self.to_send.clear();
    }

    fn to_send(&mut self, value: &str) {
        self.to_send.push_str(value);
    }
}

fn would_block(e: &std::io::Error) -> bool {
    e.kind() == std::io::ErrorKind::WouldBlock
}
