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

## References

- [RFC 9110 — HTTP Semantics](https://www.rfc-editor.org/rfc/rfc9110): HTTP 1.0 Message format and older
- [RFC 9112 — HTTP/1.1](https://www.rfc-editor.org/rfc/rfc9112): HTTP 1.1 Message formats
- [tokio::io::copy](https://dtantsur.github.io/rust-openstack/tokio/io/fn.copy.html): how tokio moves bytes between streams transparently today — baseline for writing my own interceptor that modifies in flight
