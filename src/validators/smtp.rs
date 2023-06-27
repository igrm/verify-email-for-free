use std::error::Error;
use std::net::{SocketAddr, TcpStream};

pub get_sockets(host:String, ports:Vec<i32>) -> std::io::Result<Vec<SocketAddr>> {

}

pub fn do_smtp (host:String) -> std::io::Result<(bool, bool, bool, bool, bool, bool)> {
    
}

pub fn check_smtp_connection (host:String) -> Result<(bool, bool, bool, bool, bool, bool), Box<dyn Error>> {
    Ok(do_smtp(host)?)
}