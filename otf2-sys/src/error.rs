use std::ffi;
use std::ops::{FromResidual, Try};

use crate::c;

/// Wrapper around the low-level OTF2 error code with name and description.
///
/// Note: by implementing `Try` this excludes `OTF2_SUCCESS` which means this only represents
/// error states.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct StatusCode(c::OTF2_ErrorCode);

pub type Status<T> = std::result::Result<T, StatusCode>;

impl StatusCode {
    pub fn ok() -> Self {
        StatusCode(c::OTF2_ErrorCode::OTF2_SUCCESS)
    }

    pub fn is_ok(&self) -> bool {
        self.0 == c::OTF2_ErrorCode::OTF2_SUCCESS
    }

    pub fn code(&self) -> c::OTF2_ErrorCode {
        self.0
    }

    pub fn name(&self) -> &str {
        // SAFETY: trust the otf2 library to give valid, null-terminated, const utf8 strings
        unsafe {
            str::from_utf8_unchecked(ffi::CStr::from_ptr(c::OTF2_Error_GetName(self.0)).to_bytes())
        }
    }

    pub fn description(&self) -> &str {
        // SAFETY: trust the otf2 library to give valid, null-terminated, const utf8 strings
        unsafe {
            str::from_utf8_unchecked(
                ffi::CStr::from_ptr(c::OTF2_Error_GetDescription(self.0)).to_bytes(),
            )
        }
    }

    /// This is `pub(crate)` so that consumers of this crate can't create invalid values by passing
    /// OTF2_SUCCESS.
    pub(crate) fn from_raw(code: c::OTF2_ErrorCode) -> Self {
        assert!(code != c::OTF2_ErrorCode::OTF2_SUCCESS, "Cannot create StatusCode from OTF2_SUCCESS");
        StatusCode(code)
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name(), self.description())
    }
}

impl FromResidual<StatusCode> for StatusCode {
    fn from_residual(residual: StatusCode) -> Self {
        residual
    }
}

impl<T> FromResidual<StatusCode> for Status<T> {
    fn from_residual(residual: StatusCode) -> Self {
        Err(residual)
    }
}

impl Try for StatusCode {
    type Output = ();
    type Residual = StatusCode;

    fn from_output(_output: Self::Output) -> Self {
        Self(c::OTF2_ErrorCode::OTF2_SUCCESS)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        if self.is_ok() {
            std::ops::ControlFlow::Continue(())
        } else {
            std::ops::ControlFlow::Break(self)
        }
    }
}

impl From<StatusCode> for Status<()> {
    fn from(value: StatusCode) -> Self {
        if value.is_ok() {
            Ok(())
        } else {
            Err(value)
        }
    }
}

impl From<c::OTF2_ErrorCode> for Status<()> {
    fn from(code: c::OTF2_ErrorCode) -> Self {
        StatusCode(code).into()
    }
}

impl FromResidual for c::OTF2_ErrorCode {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual.code()
    }
}

impl FromResidual<c::OTF2_ErrorCode> for Status<()> {
    fn from_residual(residual: c::OTF2_ErrorCode) -> Self {
        Err(StatusCode(residual))
    }
}

impl Try for c::OTF2_ErrorCode {
    type Output = ();
    type Residual = StatusCode;

    fn from_output(_: Self::Output) -> Self {
        c::OTF2_ErrorCode::OTF2_SUCCESS
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        if self == c::OTF2_ErrorCode::OTF2_SUCCESS {
            std::ops::ControlFlow::Continue(())
        } else {
            std::ops::ControlFlow::Break(StatusCode(self))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_otf2_error_success() {
        let result: Status<()> = c::OTF2_ErrorCode::OTF2_SUCCESS.into();
        dbg!(&result);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_otf2_error_eacces() {
        let error = StatusCode(c::OTF2_ErrorCode::OTF2_ERROR_EACCES);
        dbg!(&error);
        dbg!(error.name());
        dbg!(error.description());
        assert_eq!(error.code(), c::OTF2_ErrorCode::OTF2_ERROR_EACCES);
    }

    #[test]
    fn can_use_question_operator_on_status() {
        let fails = || -> StatusCode { StatusCode(c::OTF2_ErrorCode::OTF2_ERROR_EACCES) };
        let succeeds = || -> StatusCode { StatusCode(c::OTF2_ErrorCode::OTF2_SUCCESS) };
        let fallible = |f: fn() -> StatusCode| -> Status<()> {
            f()?;
            Ok(())
        };
        let expect_error = fallible(fails);
        let expect_success = fallible(succeeds);
        dbg!(&expect_error);
        dbg!(&expect_success);
        assert!(expect_error.is_err());
        assert!(expect_success.is_ok());
    }

    #[test]
    fn can_use_question_operator_on_code() {
        let fails = || -> c::OTF2_ErrorCode { c::OTF2_ErrorCode::OTF2_ERROR_EACCES };
        let succeeds = || -> c::OTF2_ErrorCode { c::OTF2_ErrorCode::OTF2_SUCCESS };
        let fallible = |f: fn() -> c::OTF2_ErrorCode| -> Status<()> {
            f()?;
            Ok(())
        };
        let expect_error = fallible(fails);
        let expect_success = fallible(succeeds);
        dbg!(&expect_error);
        dbg!(&expect_success);
        assert!(expect_error.is_err());
        assert!(expect_success.is_ok());
        assert_eq!(
            expect_error,
            Err(StatusCode(c::OTF2_ErrorCode::OTF2_ERROR_EACCES))
        );
        assert_eq!(expect_success, Ok(()));
    }
}
