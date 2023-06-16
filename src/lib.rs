pub fn add(left: usize, right: usize) -> usize {
    left + right
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
