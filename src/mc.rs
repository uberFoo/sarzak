//! Model Compiler Primitives
//!
//!
use std::{any::Any, path::PathBuf};

use snafu::prelude::*;

use crate::domain::Domain;

#[derive(Debug, Snafu)]
pub enum ModelCompilerError {
    #[snafu(display("ModelError: {}", description))]
    ModelError { description: String },
    #[snafu(display("I/O Error: {}, caused by {}", path.display(), source))]
    IOError {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("Compiler Error: {}", description))]
    CompilerError { description: String },
}

pub trait ModelCompilerOptions {
    fn as_any(&self) -> &dyn Any;
}

pub trait SarzakModelCompiler {
    fn compile(
        &self,
        model: &Domain,
        package: &str,
        src_path: &PathBuf,
        options: Box<&dyn ModelCompilerOptions>,
        test: bool,
    ) -> Result<(), ModelCompilerError>;
}
