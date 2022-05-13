# Rust ZeroMQ bindings

[![Build Status](https://github.com/Traverse-Research/zmq2/workflows/Continuous%20integration/badge.svg)](https://github.com/Traverse-Research/zmq2/actions)
[![Apache 2.0 licensed](https://img.shields.io/badge/license-Apache2.0-blue.svg)](./LICENSE-APACHE)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![crates.io](https://img.shields.io/crates/v/zmq2.svg)](https://crates.io/crates/zmq2)
[![docs](https://docs.rs/zmq2/badge.svg)](https://docs.rs/zmq2)

[Release Notes](https://github.com/Traverse-Research/zmq2/releases)

## About

The `zmq2` crate provides bindings for the `libzmq` library from the
[ZeroMQ](https://zeromq.org/) project. This project is a fork of the
[https://github.com/erickt/rust-zmq](rust-zmq) project, with the intent
of keeping it actively maintained.

This project removes the `cmake` dependency of this project, as well as
update the dependencies. It has also removed the `pkgconfig` build in
favor of always building a vendored version of this library.

## Compatibility

The goal of this fork is to track latest zmq releases as close as possible,
while in the beginning aiming to be a drop-in replacement of the original
`zmq` library. Though over time we'll most likely abandon that, in favor
of our own library features.

## Usage

`zmq2` is a pretty straightforward port of the C API into Rust:

```rust
fn main() {
    let ctx = zmq2::Context::new();

    let socket = ctx.socket(zmq2::REQ).unwrap();
    socket.connect("tcp://127.0.0.1:1234").unwrap();
    socket.send("hello world!", 0).unwrap();
}
```

You can find more usage examples in
https://github.com/Traverse-Research/zmq2/tree/master/examples.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed under the terms of both the
Apache License, Version 2.0 and the MIT license without any additional
terms or conditions.

See the [contribution guidelines] for what to watch out for when
submitting a pull request.

[contribution guidelines]: ./CONTRIBUTING.md
