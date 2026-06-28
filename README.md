# stream-mutator

A personal challenge to learn Rust streams by hand.

## What

Take an incoming HTTP request as a byte stream, parse and modify it on the fly, then stream it back out — without ever buffering the full request in memory.

## Why

Why not. Just want to stay in touch with writing code on my own. also trying to understand how Rusts stream model works by actually building something.

## Constraints

- No buffering the full request in memory at any point.
- No AI writing code for me.
- References: docs, google, stackoverflow if im feeling whimsical. 
- (Is just asking AI questions cheating? I'll avoid it as much as possible just in case)

## Observations

It's a streaming HTTP proxy at its core, even if that was not the original intention.

## Demo modes:
Update the 'mode' variable in main.rs to one of the values from 'pipe_mode.rs':
1. Default Mode: the most basic proxy 'pipe'. Does nothing, just passes the input stream to the output, no logging even.
2. Default Metric Mode: logs the number of bytes read, no other modifications
3. Raw Response logging mode: Slightly misnamed, logs the read buffer as a UTF-8 string chunk. Useful to see what the data passing through the pipe actually is.
4. Header Injection Mode: Uses a state machine to inject a header into the request passing through the pipe. Never buffers more than 1 byte. Achieves the main goal mentioned here, modification with no/minimal buffering.
5. Fast Header Injection Mode: Same as header injection mode, but uses a 256 byte buffer, to avoid the overhead of a syscall for each byte passing through. (Speedup is from buffer processing time >>> syscall time).

## References

- [RFC 9110 — HTTP Semantics](https://www.rfc-editor.org/rfc/rfc9110): HTTP 1.0 Message format and older
- [RFC 9112 — HTTP/1.1](https://www.rfc-editor.org/rfc/rfc9112): HTTP 1.1 Message formats
- [tokio::io::copy](https://dtantsur.github.io/rust-openstack/tokio/io/fn.copy.html): how tokio moves bytes between streams transparently today — baseline for writing my own interceptor that modifies in flight
- [tokio::net::TcpListener](https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html): accept incoming TCP connections
- [tokio_stream::wrappers::TcpListenerStream](https://docs.rs/tokio-stream/0.1.18/tokio_stream/wrappers/struct.TcpListenerStream.html): wrap listener as stream of incoming connections
- [dyn-compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility): rules for when trait can be made into `dyn Trait` object
- [std::sync::mpsc](https://doc.rust-lang.org/std/sync/mpsc/index.html): mpsc channel module docs
- [std::sync::mpsc::channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html): channel constructor — bounded backpressure for sink between two `copy()` calls
- [State machines](https://hoverbear.org/blog/rust-state-machine-pattern/): Intro to state machine patterns in rust
