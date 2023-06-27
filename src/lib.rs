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
    pub connection_result: HashMap<String, SmtpResultConnection>,
    pub inbox_is_full : bool,
    pub disabled_address: bool,
    pub email_deliverable : bool,
    pub catch_all_address : bool,
}

pub struct SmtpResultConnection {
    pub server_answered : bool,
    pub allowed_to_connect: bool,
    pub can_use_this_server: bool,
    pub tls_option_awailable: bool,
    pub valid_certificates: bool,
    pub connection_secure: bool
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

        for host in mx_validation_result.iter() {
           
           print!("---------- checking host {}\n", host);
            let smtp_connection_details:(bool, bool, bool, bool, bool, bool) = match check_smtp_connection(host.to_string() ) {
                Ok(val) => val,
                Err(error) => {print!("error: {}", error);(false, false, false, false, false, false)}
            };
            
            connection_result.insert(host.to_string(), SmtpResultConnection { 
               server_answered: smtp_connection_details.0,
               allowed_to_connect: smtp_connection_details.1,
               can_use_this_server: smtp_connection_details.2,
               tls_option_awailable: smtp_connection_details.3,
               valid_certificates: smtp_connection_details.4,
               connection_secure: smtp_connection_details.5
            });
        }
        VerificationResult{
            mx:MxResult{accepts_email : mx_validation_result.len() > 0, mx_records : mx_validation_result},
            smtp:SmtpResult { 
                connection_result: connection_result,
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
        for (key, val) in result.smtp.connection_result.iter()        {
            if val.server_answered { assert_eq!(val.server_answered, true); return;}
        }
        assert_eq!(false, true);
    }

    #[test]
    fn default_usable() {
        let result =  EmailVerifier::default();
        assert_eq!(result.dns_server, String::from("8.8.8.8:53"));
    }
}
