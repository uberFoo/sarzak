//! Sarzak Errors
//!
use std::path::PathBuf;

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub struct Error(pub(crate) SarzakError);

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum SarzakError {
    #[snafu(display("error opening file {}", path.display()))]
    FileOpen {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("error building domain: {}", message))]
    DomainBuilder { message: String },
}
