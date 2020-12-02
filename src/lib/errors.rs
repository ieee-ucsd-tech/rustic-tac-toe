use std::error::Error;
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug)]
pub enum MoveError {
    ParseInt(ParseIntError),
    OOB,
    Occupied,
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // The wrapped error contains additional information and is available
            // via the source() method.
            MoveError::ParseInt(_) => write!(f, "The input could not be parsed as a point, try again."),
            MoveError::OOB => write!(f, "The input is out of bounds, try again."),
            MoveError::Occupied => write!(f, "The input is already occupied, try again."),
        }
    }
}

impl Error for MoveError {
    // the source of the error
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            // The cause is the underlying implementation error type. Is implicitly
            // cast to the trait object `&error::Error`. This works because the
            // underlying type already implements the `Error` trait.
            MoveError::ParseInt(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<ParseIntError> for MoveError {
    fn from(err: ParseIntError) -> MoveError {
        MoveError::ParseInt(err)
    }
}