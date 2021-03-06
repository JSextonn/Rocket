mod parser;
mod error;
pub(crate) mod tables;

#[cfg(test)] mod tests;

use crate::uri::{Uri, Origin, Absolute, Authority};

use self::parser::{uri, origin, authority_only, absolute_only};

pub use self::error::Error;

type RawInput<'a> = pear::input::Pear<pear::input::Cursor<&'a [u8]>>;

#[inline]
pub fn from_str(s: &str) -> Result<Uri<'_>, Error<'_>> {
    Ok(parse!(uri: RawInput::new(s.as_bytes()))?)
}

#[inline]
pub fn origin_from_str(s: &str) -> Result<Origin<'_>, Error<'_>> {
    Ok(parse!(origin: RawInput::new(s.as_bytes()))?)
}

#[inline]
pub fn authority_from_str(s: &str) -> Result<Authority<'_>, Error<'_>> {
    Ok(parse!(authority_only: RawInput::new(s.as_bytes()))?)
}

#[inline]
pub fn absolute_from_str(s: &str) -> Result<Absolute<'_>, Error<'_>> {
    Ok(parse!(absolute_only: RawInput::new(s.as_bytes()))?)
}
