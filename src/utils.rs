//! Shared functions and objects

use crate::prelude::*;
use std::fs;
use std::path::Path;

/// Wrapper over [`std::fs::read_to_string`]
///
/// ## Error kinds
///
/// - [`crate::error::ErrorKind::OpenError`] if file does not exist
/// - [`crate::error::ErrorKind::ReadError`] if an error occured while
///   reading with `std::fs::read_to_string()` function
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    if !path.as_ref().exists() {
        return Err(
            Error::new(format!("file {} does not exist", &path.as_ref().display()))
                .set_kind(ErrorKind::OpenError),
        );
    }

    fs::read_to_string(&path).map_err(|err| Error::new(err).set_kind(ErrorKind::ReadError))
}

/// Wrapper over [`std::fs::read`]
///
/// ## Error kinds
///
/// - [`crate::error::ErrorKind::OpenError`] if file does not exist
/// - [`crate::error::ErrorKind::ReadError`] if an error occured while
///   reading with `std::fs::read()` function
pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    if !path.as_ref().exists() {
        return Err(
            Error::new(format!("file {} does not exist", &path.as_ref().display()))
                .set_kind(ErrorKind::OpenError),
        );
    }

    fs::read(&path).map_err(|err| Error::new(err).set_kind(ErrorKind::ReadError))
}

/// Wrapper over [`std::fs::write`]
///
/// ## Error kinds
///
/// - [`crate::error::ErrorKind::WriteError`] if an error occured while
///   writing with `std::fs::write` function
pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()> {
    fs::write(&path, &contents).map_err(|err| Error::new(err).set_kind(ErrorKind::WriteError))
}
