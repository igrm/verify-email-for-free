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
            let mut buffer=[0; 1000000];
            stream.read(&mut buffer)?;
            print!("    buffer read: {}\n", String::from_utf8(buffer.to_vec()).unwrap());
            result.1 = String::from_utf8(buffer.to_vec()).unwrap().contains("220");
            break;
        }
    }
    Ok(result)
}

pub fn check_smtp_connection (host:String) -> Result<(bool, bool, bool, bool, bool, bool), Box<dyn Error>> {
    Ok(do_smtp(host)?)
}