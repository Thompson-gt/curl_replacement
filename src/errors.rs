// modual for internal errors

use std::fmt;

// expected reasons for the program to error
#[derive(Debug)]
pub enum ErrorReason {
    FormatError,
    BuildError,
    NetworkError,
    UnexpectedError, // last,
}

#[derive(Debug)]
pub struct ValidError {
    pub reason: ErrorReason,
    pub message: String,
}
impl ValidError {
    /// will construct the error
    pub fn build(r: ErrorReason, m: String) -> Self {
        Self {
            reason: r,
            message: m,
        }
    }
}

impl fmt::Display for ValidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = format!("reason:{:?}\n, message:{}", self.reason, self.message);
        write!(f, "{}", err_msg)
    }
}
