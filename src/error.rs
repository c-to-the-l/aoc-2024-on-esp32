/// Crate-scoped error type that allows converging error types through ?
#[derive(Debug, thiserror::Error)]
pub enum AerError {
    #[error("Format error (format body probably too long)")]
    FmtError(core::fmt::Error),
    #[error("Bad day: {day} ")]
    BadDay { day: u32 },
    #[error("Picoserve IO Error: {0:?}")]
    PicoserveIo(picoserve::io::ErrorKind),
    #[error("Input size error: {message}. Expected {expected}, got {got}")]
    InputSize{expected: usize, got: usize, message: &'static str},
    #[error("Missing expected message= prefix of post request")]
    MissingMessage,
    #[error("Malformed or invalid utf-8 in input: {0}")]
    Utf8(#[from] core::str::Utf8Error),
    #[error("Malformed or invalid utf-8 in input: {0}")]
    AllocUtf8(#[from] alloc::string::FromUtf8Error),
    #[error("Integer parse error: {0}")]
    IntParse(#[from] core::num::ParseIntError),
    #[error("Exact read error")]
    ExactRead,
}

/// Some constraints on trait scope means that some error types need manual conversion
/// Unfortunately this trait is required relatively regularly for `picoserve::io::Error`
pub trait IntoAer<T> {
    fn into_aer(self) -> Result<T>
    where
        T: Sized;
}

impl<T, E> IntoAer<T> for core::result::Result<T, E>
where
    E: picoserve::io::Error,
{
    fn into_aer(self) -> Result<T> {
        self.map_err(|e| AerError::PicoserveIo(e.kind()))
    }
}

/// Can't use the thiserror #[from] marker for this, have to do it manually
impl From<core::fmt::Error> for AerError {
    fn from(value: core::fmt::Error) -> Self {
        Self::FmtError(value)
    }
}

/// Allows us to log errors straight to the serial port.
impl defmt::Format for AerError {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{:?}", defmt::Debug2Format(self));
    }
}

/// Crate-level result type.
pub type Result<T> = core::result::Result<T, AerError>;
