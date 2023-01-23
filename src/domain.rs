//! Sarzak Domain
//!
//! A Domain is a container for items that all participate in the same abstraction.
//! Currently that means a model.
use std::path::Path;

use serde::{Deserialize, Serialize};
use snafu::prelude::*;

use nut::{
    codegen::{
        DrawingObjectStore as FromDrawingObjectStore, SarzakObjectStore as FromSarzakObjectStore,
    },
    sarzak::SarzakModel as FromModel,
};

use crate::{
    drawing::{store::ObjectStore as DrawingObjectStore, types::Context as DrawingContext},
    error::{FileOpenSnafu, Result},
    sarzak::{store::ObjectStore as SarzakObjectStore, types::Context as SarzakContext},
    VERSION,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Domain {
    pub version: String,
    pub domain: String,
    pub description: String,
    pub sarzak: SarzakObjectStore,
    pub drawing: DrawingObjectStore,
}

impl Domain {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Domain> {
        let mut nut_model = FromModel::load_cuckoo_model(&path).context(FileOpenSnafu {
            path: path.as_ref(),
        })?;

        let domain = Domain {
            version: VERSION.to_owned(),
            domain: nut_model.domain,
            description: nut_model.description,
            sarzak: nut_model.sarzak.into(),
            drawing: nut_model.drawing.into(),
        };

        Ok(domain)
    }
}

impl From<FromSarzakObjectStore> for SarzakObjectStore {
    fn from(from: FromSarzakObjectStore) -> Self {
        let mut store = SarzakObjectStore::new();

        let mut context = SarzakContext {
            from: &from,
            to: &mut store,
        };

        Self::new()
    }
}

impl From<FromDrawingObjectStore> for DrawingObjectStore {
    fn from(from: FromDrawingObjectStore) -> Self {
        Self::new()
    }
}
