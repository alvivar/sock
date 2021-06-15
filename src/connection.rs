use std::net::SocketAddr;

use mio::{net::TcpStream, Token};

pub struct Connection {
    pub token: Token,
    pub socket: TcpStream,
    pub address: SocketAddr,
    pub open: bool,
    // pub received: Vec<u8>,
    // pub to_send: Vec<u8>,
}

impl Connection {
    pub fn new(token: Token, socket: TcpStream, address: SocketAddr) -> Connection {
        Connection {
            token,
            socket,
            address,
            open: true,
            // received: Vec::with_capacity(4096),
            // to_send: Vec::with_capacity(4096),
        }
    }

    // pub fn read(&mut self) {
    //     loop {
    //         let read = self.socket.read_to_string(&mut self.received);

    //         match read {
    //             Ok(0) => {
    //                 println!("Connection closed. 0 bytes received.");
    //                 self.is_open = false;
    //             }
    //             Ok(n) => {
    //                 println!("Data received. {} bytes received.", n);
    //             }
    //             // @todo What's ref doing? Maybe a temp var?
    //             Err(ref e) if would_block(e) => {
    //                 println!("Failed to read. Would block: {}", e);
    //                 break;
    //             }
    //             Err(e) => {
    //                 println!("Failed to read: {}", e);
    //                 break;
    //             }
    //         }
    //     }
    // }

    // pub fn write(&mut self) {
    //     match self.socket.write_all(self.to_send.as_bytes()) {
    //         Ok(_) => (),
    //         Err(_) => {
    //             self.is_open = false;
    //             return;
    //         }
    //     }

    //     self.to_send.clear();
    // }

    // pub fn set(&mut self, value: &str) {
    //     self.to_send.push_str(value);
    // }
}

// fn would_block(e: &std::io::Error) -> bool {
//     e.kind() == std::io::ErrorKind::WouldBlock
// }
