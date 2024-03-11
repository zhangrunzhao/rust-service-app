use std::fmt::Display;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {}

impl Display for Error {
    fn fmt(
        &self,
        fmt: &mut core::fmt::Formatter<'_>,
    ) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
