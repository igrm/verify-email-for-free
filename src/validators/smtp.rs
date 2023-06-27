use std::error::Error;
use std::io::Read;
use std::net::{SocketAddr, TcpStream};
use dns_lookup::lookup_host;
use crate::utils::constants::TIMEOUT;
use std::time::Duration;

pub fn get_sockets(host:String, ports:&Vec<u16>) -> std::io::Result<Vec<SocketAddr>> {
    let mut result:Vec<SocketAddr> = Vec::<SocketAddr>::new();
    let ips: Vec<std::net::IpAddr> = lookup_host(&host)?;
    for port in ports.iter() {
        for ip in ips.iter(){
            result.push(SocketAddr::new(*ip, *port));
        }   
    }
    
    Ok(result)
}

pub fn do_smtp (host:String) -> std::io::Result<(bool, bool, bool, bool, bool, bool)> {
    let mut smtp_ports:Vec<u16> = Vec::<u16>::new();
    smtp_ports.push(25);
    smtp_ports.push(465);
    smtp_ports.push(587);
    smtp_ports.push(2525);
    let sockets = get_sockets(host, &smtp_ports)?;
    print!("sockets detected: {}\n",sockets.len());
    let mut result: (bool, bool, bool, bool, bool, bool) = (false, false, false, false, false, false);
    for socket in sockets.iter() {
        print!("     trying {}\n", socket.to_string());
        if let Ok(mut stream) = TcpStream::connect_timeout(socket, Duration::from_secs(TIMEOUT)) {
            result.0 = true;
            print!("     connected {}\n", socket.to_string());
            let mut string_buffer = String::new();
            stream.read_to_string(&mut string_buffer)?;
            print!("    buffer read: {}\n", string_buffer);
            result.1 = string_buffer.contains("220");
            break;
        }
    }
    Ok(result)
}

pub fn check_smtp_connection (host:String) -> Result<(bool, bool, bool, bool, bool, bool), Box<dyn Error>> {
    Ok(do_smtp(host)?)
}