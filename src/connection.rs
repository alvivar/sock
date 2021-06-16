use std::net::SocketAddr;
// io::{self, Read},
// str::from_utf8,

use mio::{net::TcpStream, Token};

pub struct Connection {
    pub token: Token,
    pub socket: TcpStream,
    pub address: SocketAddr,
    pub open: bool,
    // pub received: Vec<u8>,
    pub to_send: Vec<u8>,
}

impl Connection {
    pub fn new(token: Token, socket: TcpStream, address: SocketAddr) -> Connection {
        Connection {
            token,
            socket,
            address,
            open: true,
            // received: Vec::with_capacity(4096),
            to_send: Vec::new(),
        }
    }

    // pub fn read(&mut self) -> io::Result<bool> {
    //     let mut received_data = vec![0; 256];
    //     let mut bytes_read = 0;

    //     // We can (maybe) read from the connection.
    //     loop {
    //         match self.socket.read(&mut received_data[bytes_read..]) {
    //             Ok(0) => {
    //                 // Reading 0 bytes means the other side has closed the
    //                 // connection or is done writing, then so are we.
    //                 self.open = false;
    //                 break;
    //             }
    //             Ok(n) => {
    //                 bytes_read += n;
    //                 if bytes_read == received_data.len() {
    //                     received_data.resize(received_data.len() + 256, 0);
    //                 }
    //             }
    //             // Would block "errors" are the OS's way of saying that the
    //             // connection is not actually ready to perform this I/O operation.
    //             Err(ref err) if would_block(err) => break,
    //             Err(ref err) if interrupted(err) => continue,
    //             // Other errors we'll consider fatal.
    //             Err(err) => return Err(err),
    //         }
    //     }

    //     if bytes_read != 0 {
    //         let received_data = &received_data[..bytes_read];
    //         if let Ok(str_buf) = from_utf8(received_data) {
    //             println!("Received data: {}", str_buf.trim_end());
    //         } else {
    //             println!("Received (none UTF-8) data: {:?}", received_data);
    //         }
    //     }

    //     if !self.open {
    //         println!("Connection closed");
    //         Ok(true)
    //     } else {
    //         Ok(false)
    //     }

    //     // else {
    //     //     // Let's prepare for writing again.
    //     //     registry.reregister(&mut connection.socket, event.token(), Interest::WRITABLE)?;
    //     // }
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

// fn would_block(err: &io::Error) -> bool {
//     err.kind() == io::ErrorKind::WouldBlock
// }

// fn interrupted(err: &io::Error) -> bool {
//     err.kind() == io::ErrorKind::Interrupted
// }
