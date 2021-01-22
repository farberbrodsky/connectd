use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

pub const ADDR: &str = "239.255.255.250:5000";
/// Currently only supports discovery using ipv4 multicast
pub fn discover(broadcast_socket: UdpSocket) -> Result<Vec<(SocketAddr, String)>, std::io::Error> {
    broadcast_socket.set_broadcast(true)?;
    broadcast_socket.set_multicast_loop_v4(false)?;
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_millis(100)))?;

    let port = socket.local_addr()?.port();
    broadcast_socket.send_to(&port.to_be_bytes(), ADDR)?;
    let mut msg = [0u8; 1024];
    let mut responses = vec![];
    while let Ok((n, peer_addr)) = socket.recv_from(&mut msg) {
        let peer_hostname = String::from_utf8(msg[..n].to_vec()).unwrap_or("anonymous".to_string());
        responses.push((peer_addr, peer_hostname));
    }
    Ok(responses)
}

/// Makes this device discoverable
pub fn discoverable(broadcast_socket: UdpSocket, name: &str) -> Result<(), std::io::Error> {
    broadcast_socket.set_broadcast(true)?;
    broadcast_socket.set_multicast_loop_v4(false)?;

    let mut msg = [0u8; 2];
    while let Ok((n, mut peer_addr)) = broadcast_socket.recv_from(&mut msg) {
        if n == 2 {
            let port = u16::from_be_bytes(msg);
            let my_msg = name.as_bytes();
            peer_addr.set_port(port);
            broadcast_socket.send_to(&my_msg, peer_addr)?;
        }
    }
    Ok(())
}
