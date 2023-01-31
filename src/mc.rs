//! Model Compiler Primitives
//!
//!
use std::{
    any::Any,
    path::{Path, PathBuf},
};

use snafu::prelude::*;

use crate::domain::Domain;

pub type Result<T, E = ModelCompilerError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ModelCompilerError {
    #[snafu(display("ModelError: {}", description))]
    Model { description: String },
    #[snafu(display("I/O Error caused by {}", source))]
    IO { source: std::io::Error },
    #[snafu(display("Format Error caused by {}", source))]
    Format { source: std::fmt::Error },
    #[snafu(display("File Error: {}, caused by {}", path.display(), source))]
    File {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Compiler Error: {}", description))]
    Compiler { description: String },
}

pub trait ModelCompilerOptions: std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}

pub trait SarzakModelCompiler {
    fn compile<P: AsRef<Path>>(
        &self,
        domain: &Domain,
        module: &str,
        src_path: P,
        options: Box<&dyn ModelCompilerOptions>,
        test: bool,
    ) -> Result<(), ModelCompilerError>;
}
