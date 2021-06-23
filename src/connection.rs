use mio::{net::TcpStream, Token};

use crossbeam_channel::{unbounded, Receiver, Sender};
use std::net::SocketAddr;

pub struct Connection {
    pub token: Token,
    pub socket: TcpStream,
    pub address: SocketAddr,
    pub open: bool,
    pub send_tx: Sender<Vec<u8>>,
    pub send_rx: Receiver<Vec<u8>>,
}

impl Connection {
    pub fn new(token: Token, socket: TcpStream, address: SocketAddr) -> Connection {
        let (send_tx, send_rx) = unbounded::<Vec<u8>>();

        Connection {
            token,
            socket,
            address,
            open: true,
            send_rx,
            send_tx,
        }
    }
}
