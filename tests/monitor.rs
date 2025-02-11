#[macro_use]
mod common;

use std::str;
use std::u16;

fn version_ge_4_3() -> bool {
    let (major, minor, _) = zmq2::version();
    (major > 4) || (major == 4 && minor >= 3)
}

/// Read one event off the monitor socket; return the SocketEvent value.
fn get_monitor_event(monitor: &mut zmq2::Socket) -> zmq2::Result<zmq2::SocketEvent> {
    let msg = monitor.recv_msg(0)?;
    // TODO: could be simplified by using `TryInto` (since 1.34)
    let event = u16::from_ne_bytes([msg[0], msg[1]]);

    assert!(
        monitor.get_rcvmore()?,
        "Monitor socket should have two messages per event"
    );

    // the address, we'll ignore it
    let _ = monitor.recv_msg(0)?;

    Ok(zmq2::SocketEvent::from_raw(event))
}

fn expect_event(mon: &mut zmq2::Socket, expected: zmq2::SocketEvent) {
    let event = get_monitor_event(mon).unwrap();
    assert_eq!(expected, event);
}

/// Send a series of pings between the client and the server.
/// The messages should round trip from the client to the server
/// and back again.
fn bounce(client: &mut zmq2::Socket, server: &mut zmq2::Socket) {
    let data = "12345678ABCDEFGH12345678abcdefgh";

    //  Send message from client to server
    client.send(data.as_bytes(), zmq2::SNDMORE).unwrap();
    client.send(data.as_bytes(), 0).unwrap();

    //  Receive message at server side
    let mut recv_data = server.recv_bytes(0).unwrap();
    assert_eq!(str::from_utf8(&recv_data).unwrap(), data);
    assert!(server.get_rcvmore().unwrap());

    recv_data = server.recv_bytes(0).unwrap();
    assert_eq!(str::from_utf8(&recv_data).unwrap(), data);
    assert!(!server.get_rcvmore().unwrap());

    //  Send message from client to server
    server.send(&recv_data, zmq2::SNDMORE).unwrap();
    server.send(&recv_data, 0).unwrap();

    //  Receive the two parts at the client side
    recv_data = client.recv_bytes(0).unwrap();
    assert_eq!(str::from_utf8(&recv_data).unwrap(), data);
    assert!(client.get_rcvmore().unwrap());

    recv_data = client.recv_bytes(0).unwrap();
    assert_eq!(str::from_utf8(&recv_data).unwrap(), data);
    assert!(!client.get_rcvmore().unwrap());
}

/// Close the given socket with LINGER set to 0
fn close_zero_linger(socket: zmq2::Socket) {
    socket.set_linger(0).unwrap();
    drop(socket);
}

test!(test_monitor_events, {
    let ctx = zmq2::Context::new();

    let mut client = ctx.socket(zmq2::DEALER).unwrap();
    let mut server = ctx.socket(zmq2::DEALER).unwrap();

    let err = client
        .monitor("tcp://127.0.0.1:9999", 0)
        .expect_err("Socket monitoring only works over inproc://");
    assert_eq!(zmq2::Error::EPROTONOSUPPORT, err);

    assert!(client
        .monitor("inproc://monitor-client", zmq2::SocketEvent::ALL as i32)
        .is_ok());
    assert!(server
        .monitor("inproc://monitor-server", zmq2::SocketEvent::ALL as i32)
        .is_ok());

    let mut client_mon = ctx.socket(zmq2::PAIR).unwrap();
    let mut server_mon = ctx.socket(zmq2::PAIR).unwrap();

    // Connect these to the inproc endpoints so they'll get events
    client_mon.connect("inproc://monitor-client").unwrap();
    server_mon.connect("inproc://monitor-server").unwrap();

    // Now do a basic ping test
    server.bind("tcp://127.0.0.1:9998").unwrap();
    client.connect("tcp://127.0.0.1:9998").unwrap();
    bounce(&mut client, &mut server);

    close_zero_linger(client);

    // Now collect and check events from both sockets
    let mut event = get_monitor_event(&mut client_mon).unwrap();
    if event == zmq2::SocketEvent::CONNECT_DELAYED {
        event = get_monitor_event(&mut client_mon).unwrap();
    }
    assert_eq!(zmq2::SocketEvent::CONNECTED, event);

    if version_ge_4_3() {
        expect_event(&mut client_mon, zmq2::SocketEvent::HANDSHAKE_SUCCEEDED);
    }
    expect_event(&mut client_mon, zmq2::SocketEvent::MONITOR_STOPPED);

    // This is the flow of server events
    expect_event(&mut server_mon, zmq2::SocketEvent::LISTENING);
    expect_event(&mut server_mon, zmq2::SocketEvent::ACCEPTED);

    if version_ge_4_3() {
        expect_event(&mut server_mon, zmq2::SocketEvent::HANDSHAKE_SUCCEEDED);
    }
    expect_event(&mut server_mon, zmq2::SocketEvent::DISCONNECTED);

    close_zero_linger(server);

    expect_event(&mut server_mon, zmq2::SocketEvent::CLOSED);
    expect_event(&mut server_mon, zmq2::SocketEvent::MONITOR_STOPPED);

    // Close down the sockets
    close_zero_linger(client_mon);
    close_zero_linger(server_mon);
});
