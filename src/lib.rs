mod utils;
mod validators;
use utils::constants::GOOGLE_DNS;
use validators::mx::get_mx_records;
use validators::smtp::*;
use std::collections::HashMap;

pub struct EmailVerifier {
    dns_server: String
}

pub struct VerificationResult {
    pub mx: MxResult,
    pub smtp: SmtpResult
}

pub struct MxResult {
    pub accepts_email: bool,
    pub mx_records: Vec<String>
}

pub struct SmtpResult {
    pub accepts_email: HashMap<String, SmtpResultConnection>,
    pub inbox_is_full : bool,
    pub disabled_address: bool,
    pub email_deliverable : bool,
    pub catch_all_address : bool,
}

pub struct SmtpResultConnection {
    pub tls_wrapped: bool,
    pub upgrade_via_starttls : bool,
    pub plain_text : bool,
    pub can_connect: bool
}

impl Default for EmailVerifier {
    fn default() -> Self {
        Self {dns_server:String::from(GOOGLE_DNS)}
    }
}

impl EmailVerifier {
    pub fn new(dns_server:&'static str) -> Self {
        Self {dns_server:String::from(dns_server)}
    }
    pub fn verify_static(&self, email:&'static str) -> VerificationResult{
        self.verify(String::from(email))
    }
    pub fn verify(&self, _email:String)-> VerificationResult{
        let mx_validation_result:Vec<String>= match get_mx_records(String::from("gmail.com"), &self.dns_server ){
            Ok(items) => items,
            Err(error) => panic!("{}",error)
        };

        let mut connection_result = HashMap::<String, SmtpResultConnection>::new();
        let mut tls_wrapped:bool = true;
        let mut upgrade_via_starttls:bool = true;
        let mut plain_text:bool = true;

        for host in mx_validation_result.iter() {
            print!("      checking host:{}\n", host);
            tls_wrapped &= match check_tls_wrapped(host) 
            {
                Ok(value) => value,
                Err(_) => false
            };
            upgrade_via_starttls &= match check_upgrade_via_starttls(host) 
            {
                Ok(value) => value,
                Err(_) => false
            };
            plain_text &= match check_plaintext(host)
            {
                Ok(value) => value,
                Err(_) => false
            };
            connection_result.insert(host.to_string(), SmtpResultConnection { 
                tls_wrapped: tls_wrapped, 
                upgrade_via_starttls: upgrade_via_starttls,
                plain_text: plain_text,
                can_connect: tls_wrapped||upgrade_via_starttls||plain_text
            });
        }
        VerificationResult{
            mx:MxResult{accepts_email : mx_validation_result.len() > 0, mx_records : mx_validation_result},
            smtp:SmtpResult { 
                accepts_email: connection_result,
                inbox_is_full: false, 
                disabled_address: false, 
                email_deliverable: false, 
                catch_all_address: false 
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dns_is_in_place() {
        let verifier = EmailVerifier::new("8.8.8.8:53");
        let result = verifier.verify_static("bill.gates@microsoft.com");
        for (key, val) in result.smtp.accepts_email.iter()        {
            print!("{}:{} {} {}\n", key,val.tls_wrapped, val.upgrade_via_starttls, val.plain_text);
            assert_eq!(val.can_connect, true);
        }
    }

    #[test]
    fn default_usable() {
        let result =  EmailVerifier::default();
        assert_eq!(result.dns_server, String::from("8.8.8.8:53"));
    }
}
