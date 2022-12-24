use crate::{
    common_ports::MOST_COMMON_PORTS_100,
    model::{Port, Subdomain},
};
use rayon::prelude::*;
use std::net::{SocketAddr, ToSocketAddrs};
use std::{net::TcpStream, time::Duration};

// Scans a list of ports for the given subdomain and returns a Subdomain object with the list of open ports
pub fn scan_ports(mut subdomain: Subdomain) -> Subdomain {
    // Create a list of SocketAddr objects for the given subdomain
    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain)
        .to_socket_addrs()
        .expect("port scanner: Creating socket address")
        .collect();

    // Return the original subdomain object if there are no SocketAddr objects
    if socket_addresses.is_empty() {
        return subdomain;
    }

    // Scan the most common ports in parallel and collect the open ones into a list
    subdomain.open_ports = MOST_COMMON_PORTS_100
        .into_par_iter()
        .map(|port| scan_port(socket_addresses[0], *port))
        .filter(|port| port.is_open) // filter closed ports
        .collect();
    subdomain
}

// Scans a single port and returns a Port object indicating whether it is open or closed
fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    // Set a timeout of 3 seconds for the connection attempt
    let timeout = Duration::from_secs(3);
    // Update the port number of the SocketAddr object
    socket_address.set_port(port);

    // Check if a TcpStream can be established with the given SocketAddr within the timeout
    let is_open = TcpStream::connect_timeout(&socket_address, timeout).is_ok();

    Port { port, is_open }
}
