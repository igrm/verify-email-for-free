use crate::utils::common::read_dns_data;
use regex::Regex;
use std::error::Error;
use substring::Substring;

pub fn get_mx_records(hostname:String, dns_server:&String) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result:Vec<String> = Vec::<String>::new();
    let raw = read_dns_data(hostname, dns_server, rustdns::types::Type::MX)?;
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