//! Version 2 Sarzak Domain
//!
use std::path::PathBuf;
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v1::domain::Domain as DomainV1;
use crate::v2::{
    drawing::store::ObjectStore as DrawingStore, merlin::store::ObjectStore as MerlinStore,
};

#[cfg(feature = "sarzak_multi")]
use crate::v2::sarzak::store::ObjectStore as SarzakStore;

#[cfg(feature = "sarzak_single")]
use crate::v2::sarzak_single::store::ObjectStore as SarzakStore;

use crate::VERSION;

#[derive(Clone, Debug, Deserialize, Serialize)]
struct MetaData {
    version: String,
    domain: String,
    id: Uuid,
    description: String,
    extents: [u16; 2],
    view: [i32; 2],
    path: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Domain {
    meta: MetaData,
    sarzak: SarzakStore,
    merlin: MerlinStore,
}

impl Domain {
    /// Return the name of the domain
    ///
    pub fn domain(&self) -> &str {
        &self.meta.domain
    }

    /// Return the UUID of the domain.
    ///
    pub fn id(&self) -> &Uuid {
        &self.meta.id
    }

    /// Return the name of the domain
    ///
    /// This is an alias for [`Domain::domain`]. I should pick one probably.
    ///
    pub fn name(&self) -> &str {
        &self.meta.domain
    }

    /// Return the domain description
    ///
    pub fn description(&self) -> &str {
        &self.meta.description
    }

    pub fn extents(&self) -> &[u16; 2] {
        &self.meta.extents
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
    pub fn merlin(&self) -> &MerlinStore {
        &self.merlin
    }

    /// Return a mutable reference to the drawing store
    ///
    /// This returns a reference to the [`ObjectStore`] that contains the domain
    /// model UI instances.
    pub fn merlin_mut(&mut self) -> &mut MerlinStore {
        &mut self.merlin
    }

    pub fn path(&self) -> &PathBuf {
        &self.meta.path
    }

    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();

        let sarzak = SarzakStore::load(path)?;
        let merlin = MerlinStore::load(path)?;

        let file = fs::File::open(path.join("metadata.json"))?;
        let reader = io::BufReader::new(file);
        let meta: MetaData = serde_json::from_reader(reader)?;

        let domain = Domain {
            meta,
            sarzak,
            merlin,
        };

        Ok(domain)
    }

    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(path.parent().unwrap())?;

        self.sarzak.persist(path)?;
        self.merlin.persist(path)?;

        let path = path.join("metadata.json");
        let file = fs::File::create(path)?;
        let writer = io::BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &self.meta)?;

        Ok(())
    }

    pub fn persist_bincode<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(path.parent().unwrap())?;

        self.sarzak.persist_bincode(path)?;

        Ok(())
    }
}

impl From<DomainV1> for Domain {
    fn from(domain: DomainV1) -> Self {
        let sarzak = domain.sarzak().into();
        let drawing: DrawingStore = domain.drawing().into();
        let merlin = (&drawing, &sarzak).into();

        let domain = Domain {
            meta: MetaData {
                version: VERSION.to_owned(),
                domain: domain.domain().to_owned(),
                id: domain.id().to_owned(),
                description: domain.description().to_owned(),
                extents: domain.extents().to_owned(),
                view: domain.view().to_owned(),
                path: domain.path().to_owned(),
            },
            sarzak,
            merlin,
        };

        domain
    }
}
