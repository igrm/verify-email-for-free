pub struct EmailVerifier {
    dns_server: String
}

pub struct VerificationResult {
    mx: MxResult
}

struct MxResult {
    accepts_mail: bool,
    mx_records: Vec<String>
}

impl EmailVerifier {
    pub fn new(dns_server:&'static str) -> Self {
        Self {dns_server:String::from(dns_server)}
    }
    pub fn verify_static(email:&'static str) -> VerificationResult{
        Self::verify(String::from(email))
    }
    pub fn verify(email:String)-> VerificationResult{
        VerificationResult{mx:MxResult{accepts_mail:false, mx_records:Vec::<String>::new()}}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dns_is_in_place() {
        let result = EmailVerifier::new("8.8.8.8:53");
        assert_eq!(result.dns_server, String::from("8.8.8.8:53"));
    }
}
