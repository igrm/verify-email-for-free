use std::error::Error;
use std::time::Duration;
use lettre::SmtpTransport;
use crate::utils::constants::TIMEOUT;

pub fn check_tls_wrapped(hostname:&String) -> Result<bool, Box<dyn Error>> {
    let mailer:SmtpTransport= SmtpTransport::relay(hostname)
                                 .expect("build SmtpTransport::relay")
                                 .timeout(Some(Duration::from_secs(TIMEOUT)))
                                 .build();
    let result = mailer.test_connection()?;
    Ok(result)
}

pub fn check_upgrade_via_starttls(hostname:&String) -> Result<bool, Box<dyn Error>> {
    let mailer:SmtpTransport= SmtpTransport::starttls_relay(hostname)
                                 .expect("build SmtpTransport::starttls_relay")
                                 .timeout(Some(Duration::from_secs(TIMEOUT)))
                                 .build();
    let result = mailer.test_connection()?;
    Ok(result)
}

pub fn check_plaintext(hostname:&String) -> Result<bool, Box<dyn Error>> {
    let mailer:SmtpTransport= SmtpTransport::builder_dangerous(hostname)
                                 .timeout(Some(Duration::from_secs(TIMEOUT)))
                                 .build();
    let result = mailer.test_connection()?;
    Ok(result)
}