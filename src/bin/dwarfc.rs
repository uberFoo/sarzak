use std::{env, ffi::OsString, fs, os::unix::ffi::OsStringExt, path::PathBuf, process};

use clap::{ArgAction, Parser, Subcommand};
use log;
use sarzak::mc::{CompilerSnafu, FileSnafu, IOSnafu, ModelCompilerError, Result};
use snafu::prelude::*;

use sarzak::{
    domain::DomainBuilder,
    dwarf::{parse_dwarf, populate_lu_dog},
};

const TARGET_DIR: &str = "target";
const BUILD_DIR: &str = "sarzak";
const EXTENSIONS: [&str; 2] = ["tao", "道"];

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// Dwarf Source File
    ///
    /// Path to the source file to compile.
    source: PathBuf,
    /// Model File
    ///
    /// Path to the model, corresponding to the source file, to build the
    /// Lu-Dog domain.
    model: PathBuf,
    /// Meta-Model File
    ///
    /// Path to the meta-model, sarzak.
    sarzak: PathBuf,
    /// Path to package
    ///
    /// If included, `sarzak` will create a new domain in the specified
    /// location. It must exist, and must be part of a Rust package.
    #[arg(long, short)]
    package_dir: Option<PathBuf>,
}

fn find_package_dir(start_dir: &Option<PathBuf>) -> Result<PathBuf> {
    if let Some(dir) = start_dir {
        std::env::set_current_dir(&dir).context(IOSnafu {
            description: "Failed to set current dir".to_owned(),
        })?;
    }

    // Figure out where Cargo.toml is located.
    //
    let output = process::Command::new("cargo")
        .arg("locate-project")
        .arg("--message-format")
        .arg("plain")
        .output()
        .context(IOSnafu {
            description: "Failed to run cargo locate-project".to_owned(),
        })?;

    ensure!(
        output.status.success(),
        CompilerSnafu {
            description: "cargo locate-project failed".to_owned()
        }
    );

    let mut stdout = output.stdout;

    // I don't know if it's kosher, but this does nicely to get rid of
    // that newline character.
    stdout.pop();
    let os_string = OsString::from_vec(stdout);
    let mut package_root = PathBuf::from(os_string);
    // Get rid of Cargo.toml
    package_root.pop();

    log::debug!("Found root 🦀 at {:?}!", package_root);

    Ok(package_root)
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = Args::parse();

    let model = DomainBuilder::new()
        .cuckoo_model(&args.model)
        .unwrap()
        .build_v2()
        .unwrap();

    let sarzak = DomainBuilder::new()
        .cuckoo_model(&args.sarzak)
        .unwrap()
        .build_v2()
        .unwrap();

    let package_dir = find_package_dir(&args.package_dir)?;

    // This is where we output the file. I think this is stupid. We should just write
    // it to current working directory. We do however want the package_dir, because
    // we need to feed it to the lu dog populator.
    let mut path = PathBuf::from(package_dir);
    path.push(TARGET_DIR);
    path.push(BUILD_DIR);
    path.push(model.name());

    fs::create_dir_all(&path).context(FileSnafu {
        description: "creating type directory".to_owned(),
        path: &path,
    })?;

    let src = fs::read_to_string(&args.source).expect("Failed to read file");

    let ast = parse_dwarf(&src).expect("Failed to parse file");

    let lu_dog =
        populate_lu_dog(&ast, model.sarzak(), sarzak.sarzak()).expect("Failed to populate lu_dog");

    lu_dog.persist(&path).expect("Failed to persist lu_dog");

    println!("Lu-Dog domain created at {:?}", path);

    Ok(())
}
