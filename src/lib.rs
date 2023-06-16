mod utils;
mod validators;
use utils::constants::GOOGLE_DNS;
use validators::mx::get_mx_records;

pub struct EmailVerifier {
    dns_server: String
}

pub struct VerificationResult {
    pub mx: MxResult
}

pub struct MxResult {
    pub accepts_mail: bool,
    pub mx_records: Vec<String>
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
        VerificationResult{mx:MxResult{accepts_mail:false, mx_records:get_mx_records(String::from("gmail.com"),&self.dns_server).unwrap()}}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dns_is_in_place() {
        let result = EmailVerifier::new("8.8.8.8:53");
        print!("checking test!!!");
        print!("{}",result.verify_static("23@ss.ss").mx.mx_records[0]);
        assert_eq!(result.dns_server, String::from("8.8.8.8:53"));
    }

    #[test]
    fn default_usable() {
        let result =  EmailVerifier::default();
        assert_eq!(result.dns_server, String::from("8.8.8.8:53"));
    }
}
