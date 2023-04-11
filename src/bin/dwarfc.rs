use std::{env, fs, path::PathBuf};

use clap::{ArgAction, Parser, Subcommand};

use sarzak::{
    domain::DomainBuilder,
    dwarf::{parse, populate_lu_dog},
    sarzak::store::ObjectStore as SarzakStore,
    v2::domain::Domain,
};

const EXTENSIONS: [&str; 2] = ["tao", "道"];

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// Dwarf Source File
    ///
    /// Source file to compile.
    source: PathBuf,
    /// Model File
    ///
    /// Include the model, corresponding to the source file, to build the
    /// Lu-Dog domain.
    model: PathBuf,
}
fn main() {
    let args = Args::parse();

    let model = DomainBuilder::new()
        .cuckoo_model(&args.model)
        .unwrap()
        .build_v2()
        .unwrap();

    let src = fs::read_to_string(&args.source).expect("Failed to read file");

    let ast = parse(&src).expect("Failed to parse file");

    let lu_dog = populate_lu_dog(&ast, model.sarzak()).expect("Failed to populate lu_dog");
}
