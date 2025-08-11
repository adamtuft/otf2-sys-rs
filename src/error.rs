use std::ffi;
use std::ops::{FromResidual, Try};

use crate::c;

/// Wrapper around the low-level OTF2 error code with name and description.
///
/// Note: by implementing `Try` this excludes `OTF2_SUCCESS` which means this only represents
/// error states.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Status(c::OTF2_ErrorCode);

impl Status {
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
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name(), self.description())
    }
}

impl From<c::OTF2_ErrorCode> for Status {
    fn from(code: c::OTF2_ErrorCode) -> Self {
        Status(code)
    }
}

impl FromResidual<Status> for Status {
    fn from_residual(residual: Status) -> Self {
        residual
    }
}

impl FromResidual<Status> for Result<(), Status> {
    fn from_residual(residual: Status) -> Self {
        Err(residual)
    }
}

impl Try for Status {
    type Output = ();
    type Residual = Status;

    fn from_output(_output: Self::Output) -> Self {
        Self(c::OTF2_ErrorCode::OTF2_SUCCESS)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        if self.0 == c::OTF2_ErrorCode::OTF2_SUCCESS {
            std::ops::ControlFlow::Continue(())
        } else {
            std::ops::ControlFlow::Break(self)
        }
    }
}

pub type StatusResult<T> = std::result::Result<T, Status>;

impl From<Status> for StatusResult<()> {
    fn from(value: Status) -> Self {
        if value.0 == c::OTF2_ErrorCode::OTF2_SUCCESS {
            Ok(())
        } else {
            Err(value)
        }
    }
}

impl From<c::OTF2_ErrorCode> for StatusResult<()> {
    fn from(code: c::OTF2_ErrorCode) -> Self {
        Status(code).into()
    }
}

// pub trait IntoStatusResult {
//     fn into_result(self) -> StatusResult<()>;
// }

// impl<T> IntoStatusResult for Option<T>
// where
//     T: Into<StatusResult<()>>,
// {
//     fn into_result(self) -> StatusResult<()> {
//         match self {
//             Some(code) => code.into(),
//             None => Ok(()),
//         }
//     }
// }

impl FromResidual for c::OTF2_ErrorCode {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        residual.code()
    }
}

impl FromResidual<c::OTF2_ErrorCode> for Result<(), Status> {
    fn from_residual(residual: c::OTF2_ErrorCode) -> Self {
        Err(Status(residual))
    }
}

impl Try for c::OTF2_ErrorCode {
    type Output = ();
    type Residual = Status;

    fn from_output(output: Self::Output) -> Self {
        c::OTF2_ErrorCode::OTF2_SUCCESS
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        if self == c::OTF2_ErrorCode::OTF2_SUCCESS {
            std::ops::ControlFlow::Continue(())
        } else {
            std::ops::ControlFlow::Break(Status(self))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_otf2_error_success() {
        let result: StatusResult<()> = c::OTF2_ErrorCode::OTF2_SUCCESS.into();
        dbg!(&result);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_otf2_error_eacces() {
        let error = Status(c::OTF2_ErrorCode::OTF2_ERROR_EACCES);
        dbg!(&error);
        dbg!(error.name());
        dbg!(error.description());
        assert_eq!(error.code(), c::OTF2_ErrorCode::OTF2_ERROR_EACCES);
    }

    #[test]
    fn can_use_question_operator_on_status() {
        let fails = || -> Status { Status(c::OTF2_ErrorCode::OTF2_ERROR_EACCES) };
        let succeeds = || -> Status { Status(c::OTF2_ErrorCode::OTF2_SUCCESS) };
        let fallible = |f: fn() -> Status| -> Result<(), Status> {
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
        let fallible = |f: fn() -> c::OTF2_ErrorCode| -> Result<(), Status> {
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
            Err(Status(c::OTF2_ErrorCode::OTF2_ERROR_EACCES))
        );
        assert_eq!(expect_success, Ok(()));
    }
}
