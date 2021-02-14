use snafu::Snafu;

/// Main error type for model errors
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Main error type
#[derive(Debug, Snafu)]
#[snafu(visibility="pub(crate)")]
pub enum Error {
    #[snafu(display("Unknown error occurred"))]
    Unknown,

    #[snafu(display("Invalid magic in file: expected {:#X} got {:#X}", expected_magic, got_magic))]
    InvalidMagic { expected_magic: u32, got_magic: u32 },
    
    #[snafu(display("A general I/O error occurred: {}", source))]
    IOError {
        source: std::io::Error
    },

    #[snafu(display("Slice failed to convert to array: {}", source))]
    FailedSliceError {
        source: std::array::TryFromSliceError
    }
}
