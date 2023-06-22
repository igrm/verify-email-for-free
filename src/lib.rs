mod utils;
mod validators;
use utils::constants::GOOGLE_DNS;
use validators::mx::get_mx_records;
use validators::smtp::can_connect_remotely;

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
    pub accepts_smtp_connection: bool,
    pub inbox_is_full : bool,
    pub disabled_address: bool,
    pub email_deliverable : bool,
    pub catch_all_address : bool,
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
    pub fn verify(&self, email:String)-> VerificationResult{
        let mx_validation_result:Vec<String>= match get_mx_records(String::from("gmail.com"), &self.dns_server ){
            Ok(items) => items,
            Err(error) => panic!("{}",error)
        };

        let mut accepts_smtp_connection:bool = true;
        for host in mx_validation_result.iter() {
            print!("-----------checking host:{}\n", host.trim());
            accepts_smtp_connection &= can_connect_remotely(host).unwrap();
        }
        VerificationResult{
            mx:MxResult{accepts_email : mx_validation_result.len() > 0, mx_records : mx_validation_result},
            smtp:SmtpResult { accepts_smtp_connection: accepts_smtp_connection, inbox_is_full: false, disabled_address: false, email_deliverable: false, catch_all_address: false }
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
        assert_eq!(result.mx.mx_records.len() > 0, true);
    }

    #[test]
    fn default_usable() {
        let result =  EmailVerifier::default();
        assert_eq!(result.dns_server, String::from("8.8.8.8:53"));
    }
}
