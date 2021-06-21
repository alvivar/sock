# sock

Simple multi-thread websocket echo server in Rust.

Built over MIO's TCP example.

My goal was to learn [how to make
websockets](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers)
from a TcpListener and use [MIO](https://github.com/tokio-rs/mio) as
non-blocking I/O event manager.

# how

Just run "cargo run"

You can connect to the server using 'nc' on Linux:
_\$ nc 127.0.0.1 1984_
Send a message to receive the same message.

# working on

This way I can use what I learn here to make
[Bite](https://github.com/alvivar/bite) faster and web compatible.
