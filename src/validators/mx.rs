use rustdns::Message;
use rustdns::types::*;
use std::net::UdpSocket;
use std::time::Duration;
use regex::Regex;
use std::error::Error;
use substring::Substring;

fn read_dns_data(hostname:String, dns_server:&String) -> std::io::Result<String> {
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
    Ok(answer.to_string())
}

pub fn get_mx_records(hostname:String, dns_server:&String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result:Vec<String> = Vec::<String>::new();
    let raw = read_dns_data(hostname, dns_server)?;
    let re = Regex::new(r"MX\s+\d+\s.+\.")?;
    for cap in re.captures_iter(&raw) {
        let mut temp:String = cap[0].to_string();
        temp = temp.replace("MX ", "").trim().to_string();
        temp = temp.trim_end_matches(".").to_string();
        temp = temp.replace(temp.substring(0, temp.find(" ").unwrap_or(0)), "").to_string();
        result.push(temp.trim().to_string());
    }
    Ok(result)

}