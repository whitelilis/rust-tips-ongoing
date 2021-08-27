use std::fmt::Debug;
use std::io::Error;
use std::net::UdpSocket;

///
/// A trick to get current node's outer ip.
///
///

pub fn get_outer_ip() -> Result<String, Error> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("114.114.114.114:80")?;

    match socket.local_addr() {
        Ok(addr) => Ok(addr.ip().to_string()),
        Err(_) => Err(Error::from_raw_os_error(2)),
    }
}

///
/// A trick to check if some ip is belongs to current node
///
///

pub fn is_my_ip(ip: &str) -> bool {
    UdpSocket::bind(format!("{}:0", ip)).is_ok()
}
