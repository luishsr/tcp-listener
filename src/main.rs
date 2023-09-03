use std::net::{UdpSocket};
use std::thread;

fn main() -> std::io::Result<()> {
    // Create a UDP socket to send and receive discovery messages
    let socket = UdpSocket::bind("0.0.0.0:12345")?;
    socket.set_broadcast(true)?;

    println!("UDP Discovery Server is listening on 0.0.0.0:12345");

    // Spawn a thread to handle incoming discovery messages
    thread::spawn(move || {
        handle_discovery_messages(socket);
    });

    // Main thread for sending discovery messages
    send_discovery_messages();

    Ok(())
}

fn send_discovery_messages() {
    // Replace with your own custom discovery message
    let discovery_message = "Hello from the discovery server!";
    let broadcast_addr = "255.255.255.255:12345";

    // Send the discovery message to the broadcast address
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        socket.set_broadcast(true).ok();
        socket.send_to(discovery_message.as_bytes(), broadcast_addr).ok();
    }
}

fn handle_discovery_messages(socket: UdpSocket) {
    let mut buffer = [0u8; 1024];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, src)) => {
                let message = String::from_utf8_lossy(&buffer[0..size]);
                println!("Received discovery message from {}: {}", src, message);
            },
            Err(_) => {
                println!("Error receiving discovery message");
            },
        }
    }
}
