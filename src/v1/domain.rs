//! Version 1 Sarzak Domain
//!
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v1::{
    drawing::store::ObjectStore as DrawingStore, sarzak::store::ObjectStore as SarzakStore,
};
use crate::VERSION;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Domain {
    version: String,
    domain: String,
    id: Uuid,
    description: String,
    sarzak: SarzakStore,
    drawing: DrawingStore,
}

impl Domain {
    /// Create a new Domain
    ///
    /// This is used by the [`DomainBuilder`] to initialize a domain. It's not
    /// a generally useful means of creating a domain.
    pub(crate) fn new(
        domain: String,
        id: Uuid,
        description: String,
        sarzak: SarzakStore,
        drawing: DrawingStore,
    ) -> Self {
        let domain = Domain {
            version: VERSION.to_owned(),
            domain,
            id,
            description,
            sarzak,
            drawing,
        };

        domain
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
}
