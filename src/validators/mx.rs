use rustdns::Message;
use rustdns::types::*;
use std::net::UdpSocket;
use std::time::Duration;

pub fn get_mx_records(hostname:String, dns_server:&String) -> std::io::Result<Vec<String>> {
    let mut message = Message::default();
    message.add_question(&hostname, Type::MX, Class::Internet);
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::new(5, 0)))?;
    socket.connect(dns_server)?;
    let question = message.to_vec()?;
    socket.send(&question)?;
    let mut resp = [0; 4096];
    let len = socket.recv(&mut resp)?;
    let answer = Message::from_slice(&resp[0..len])?;
    let mut result = Vec::<String>::new();
    result.push(answer.to_string());
    Ok(result)
}