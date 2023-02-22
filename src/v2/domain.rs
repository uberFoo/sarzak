//! Version 2 Sarzak Domain
//!
use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v1::domain::Domain as DomainV1;
use crate::v2::{
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
    #[allow(dead_code)]
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

    /// Return a mutable reference to the sarzak store
    ///
    /// This returns a reference to the [`ObjectStore`] that contains the domain
    /// model instances.
    pub fn sarzak_mut(&mut self) -> &mut SarzakStore {
        &mut self.sarzak
    }

    /// Return a reference to the drawing store
    ///
    /// This returns a reference to the [`ObjectStore`] that contains the domain
    /// model UI instances.
    pub fn drawing(&self) -> &DrawingStore {
        &self.drawing
    }

    /// Return a mutable reference to the drawing store
    ///
    /// This returns a reference to the [`ObjectStore`] that contains the domain
    /// model UI instances.
    pub fn drawing_mut(&mut self) -> &mut DrawingStore {
        &mut self.drawing
    }

    pub fn persist<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let path = path.join(format!("{}.json", self.domain));
        fs::create_dir_all(path.parent().unwrap())?;

        self.sarzak.persist(&path)?;
        self.drawing.persist(&path)?;

        // let path = path.join("metadata.json");
        // let file = fs::File::create(path)?;
        // let writer = io::BufWriter::new(file);
        // serde_json::to_writer_pretty(writer, &self)?;

        Ok(())
    }
}

impl From<DomainV1> for Domain {
    fn from(domain: DomainV1) -> Self {
        let domain = Domain {
            version: VERSION.to_owned(),
            domain: domain.domain().to_owned(),
            id: domain.id().to_owned(),
            description: domain.description().to_owned(),
            sarzak: domain.sarzak().into(),
            drawing: domain.drawing().into(),
        };

        domain
    }
}
