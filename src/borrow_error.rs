use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub struct BorrowError;

impl Debug for BorrowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BorrowError").finish()
    }
}

impl Display for BorrowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Borrow error")
    }
}

impl Error for BorrowError {}