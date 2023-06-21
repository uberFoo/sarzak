//! Version 1 Sarzak Domain
//!
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v1::{
    drawing::store::ObjectStore as DrawingStore, sarzak::store::ObjectStore as SarzakStore,
};
use crate::VERSION;
use nut::sarzak::SarzakModel;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Domain {
    version: String,
    domain: String,
    id: Uuid,
    description: String,
    sarzak: SarzakStore,
    drawing: DrawingStore,
    extents: [u16; 2],
    view: [i32; 2],
    path: PathBuf,
}

impl Domain {
    /// Create a new Domain
    ///
    /// This is used by the [`DomainBuilder`] to initialize a domain. It's not
    /// a generally useful means of creating a domain.
    pub(crate) fn new(
        path: PathBuf,
        model: SarzakModel,
        sarzak: SarzakStore,
        drawing: DrawingStore,
    ) -> Self {
        

        Domain {
            version: VERSION.to_owned(),
            domain: model.domain,
            id: model.id,
            description: model.description,
            sarzak,
            drawing,
            extents: model.extents,
            view: model.view,
            path,
        }
    }

    /// Return the name of the domain
    ///
    pub fn domain(&self) -> &str {
        &self.domain
    }

    /// Return the UUID of the domain.
    ///
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Return the name of the domain
    ///
    /// This is an alias for [`Domain::domain`]. I should pick one probably.
    ///
    pub fn name(&self) -> &str {
        &self.domain
    }

    /// Return the domain description
    ///
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Return a reference to the sarzak store
    ///
    /// This returns a reference to the [`ObjectStore`] that contains the domain
    /// model instances.
    pub fn sarzak(&self) -> &SarzakStore {
        &self.sarzak
    }

    /// Return a reference to the drawing store
    ///
    /// This returns a reference to the [`ObjectStore`] that contains the domain
    /// model UI instances.
    pub fn drawing(&self) -> &DrawingStore {
        &self.drawing
    }

    /// Return the domain extents
    ///
    pub fn extents(&self) -> &[u16; 2] {
        &self.extents
    }

    /// Return the domain view
    ///
    pub fn view(&self) -> &[i32; 2] {
        &self.view
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
