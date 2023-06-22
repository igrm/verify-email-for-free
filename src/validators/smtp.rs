use std::error::Error;
use std::time::Duration;
use lettre::SmtpTransport;

pub fn can_connect_remotely(hostname:&String) -> Result<bool, Box<dyn Error>> {
    let mailer:SmtpTransport= SmtpTransport::relay(hostname)?
                                 .timeout(Some(Duration::from_secs(10)))
                                 .build();
    let result = mailer.test_connection()?;
    Ok(result)
}