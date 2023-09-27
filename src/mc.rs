//! Model Compiler Primitives
//!
//!
use std::{
    any::Any,
    path::{Path, PathBuf},
};

use ansi_term::Colour;
use snafu::{prelude::*, Backtrace, Location};

use crate::v2::domain::Domain;

pub type Result<T, E = ModelCompilerError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ModelCompilerError {
    #[snafu(display("ModelError: {}", description))]
    Model { description: String },
    #[snafu(display("\n{}: {description}: {}:{}:{}\n  --> {source}", Colour::Red.bold().paint("error"), location.file, location.line, location.column))]
    IO {
        source: std::io::Error,
        description: String,
        location: Location,
    },
    #[snafu(display("Format Error caused by {}", source))]
    Format { source: std::fmt::Error },
    #[snafu(display("\n{backtrace}\n{}: {description}: {}:{}:{}\n  --> {source} ({})", Colour::Red.bold().paint("error"), location.file, location.line, location.column, path.display()))]
    File {
        backtrace: Backtrace,
        location: Location,
        description: String,
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display(
        "\n{backtrace}\n{}: {description}\n  --> {}:{}:{}", Colour::Red.bold().paint("error"), location.file, location.line, location.column
        // "\n{}: {description}\n  --> {}:{}:{}", Colour::Red.bold().paint("error"), location.file, location.line, location.column
    ))]
    Compiler {
        backtrace: Backtrace,
        location: Location,
        description: String,
    },
}

pub trait ModelCompilerOptions: std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
}

pub trait SarzakModelCompiler {
    fn compile<P: AsRef<Path>>(
        &self,
        domain: Domain,
        package: &str,
        module: &str,
        src_path: P,
        options: Box<&dyn ModelCompilerOptions>,
        test: bool,
        verbosity: u8,
    ) -> Result<usize, ModelCompilerError>;
}
