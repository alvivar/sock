# Sock

Simple multi-thread echo server in Rust.

I wanted to learn how to use [MIO](https://github.com/tokio-rs/mio) as
non-blocking I/O in a multi-thread way, so I can improve the way
[Bite](https://github.com/alvivar/bite) handles connections.

Based on the [MIO TCP
example](https://github.com/tokio-rs/mio/blob/master/examples/tcp_server.rs).

## Try it

_"cargo run"_ to start the server.

You can connect with **nc** on unix consoles:

    nc 127.0.0.1 1984

Send a message, receive the same message.

## For later

- [Writing WebSocket servers](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers)
