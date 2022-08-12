use std::any::Any;

use crate::status_code::StatusCode;

pub trait ErrorExt: std::error::Error {
    fn status_code(&self) -> StatusCode {
        StatusCode::Unknown
    }

    fn backtrace_opt(&self) -> Option<crate::snafu::Backtrace>;

    fn as_any(&self) -> &dyn Any;
}
