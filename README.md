# Sock

Simple multi-thread echo server in Rust.

I wanted to learn [how to make
websockets](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers),
how to use [MIO](https://github.com/tokio-rs/mio) as non-blocking I/O in a
multi-thread way, and how to improve the way [Bite](github.com/alvivar/bite)
handles connections.

## Try it

_"cargo run"_ to start the server.

You can connect with **nc** on console:

    nc 127.0.0.1 1984

Send a message, receive the same message.
