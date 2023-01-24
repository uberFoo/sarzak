//! Sarzak Domain
//!
//! A Domain is a container for items that all participate in the same abstraction.
//! Currently that means a model.
use std::path::Path;

use serde::{Deserialize, Serialize};
use snafu::prelude::*;

use nut::{
    codegen::{
        DrawingObjectStore as FromDrawingObjectStore, Extrude,
        SarzakObjectStore as FromSarzakObjectStore,
    },
    sarzak::SarzakModel as FromModel,
};

use crate::{
    drawing::{store::ObjectStore as DrawingObjectStore, types::Context as DrawingContext},
    error::{DomainBuilderSnafu, FileOpenSnafu, Result},
    sarzak::{
        store::ObjectStore as SarzakObjectStore,
        types::{
            Attribute, Cardinality, Conditionality, Context as SarzakContext, Object, Relationship,
            Subtype, Type, BOOLEAN, CONDITIONAL, FLOAT, INTEGER, MANY, ONE, STRING, UNCONDITIONAL,
            UUID,
        },
    },
    VERSION,
};

/// Domain Builder
///
/// Loading a domain isn't that complicated really: there's currently only a
/// single  file type from which to load one. Although, that _is_ going to
/// change once `Nutter` is running. The main reason for a builder is so that we
/// can affect the instances in the domain in a single convenient place.
///
/// There are two opportunities to insert (there is no removal mechanism in the
/// [`ObjectStore`][os] interface, and maybe that needs to be addressed) instances
/// into the store. Before extrusion, and after extrusion. Remember that extrusion
/// is the means by which instances in `nut::sarzak::ObjectStore` are converted
/// to instances in `sarzak::sarzak::ObjectStore`.
///
/// # Pre-extrusion
///
/// Pre-extrusion modification is necessary for base functionality in order to
/// pre-populate the store with primitives that we need in the store during
/// extrusion. For example, when extruding [`Attribute`] we need to be able to
/// point at an instance of [`Type`]. Type however consists of `const` `Uuid`s,
/// which don't yet exist in the new store. So we create them before doing
/// extrusion.
///
/// ```
/// # use sarzak::sarzak::{Cardinality, ONE, MANY};
/// # use sarzak::domain::DomainBuilder;
/// # const SARZAK_MODEL:&str = "models/sarzak.json";
/// let sarzak = DomainBuilder::new()
///     .cuckoo_model(SARZAK_MODEL).unwrap()
///     .pre_extrude(|mut sarzak, mut drawing| {
///         // Create instances of primitives missing from nut::sarzak that
///         // the extrusion process depends upon.
///         sarzak.inter_cardinality(Cardinality::One(ONE));
///         sarzak.inter_cardinality(Cardinality::Many(MANY));
///     })
///     .build().unwrap();
///
/// let store = sarzak.get_sarzak();
/// assert_eq!(&Cardinality::One(ONE), store.exhume_cardinality(&ONE).unwrap())
///```
///
/// I think that pre-extrusion is not going to be generally interesting. I think
/// that the generally interesting functionality is going to be post-extrusion.
/// It is during post-extrusion that one has the opportunity to create instances
/// that don't exist in the source model. This has two applications.
///
/// # Post-extrusion
///
/// When loading the metamodel we have the opportunity to create instances of
/// [`State`][s] and it's related objects. This is significant because we don't
/// yet have a tool for creating states and [`Event`][e]s and whatnot. This is
/// a gateway to allow us to start working on action semantics without needing
/// a tool.
///
/// When loading an application domain post-extrusion allows one to initialize
/// the system with instances that should exist at start-up. It's also a place
/// where initialization events may be created and sent.
///
/// [os]: crate::sarzak::ObjectStore
/// [s]: crate::sarzak::State
/// [e]: crate::sarzak::Event
pub struct DomainBuilder {
    nut_model: Option<FromModel>,
    pre_extrude: Option<Box<dyn Fn(&mut SarzakObjectStore, &mut DrawingObjectStore)>>,
    post_extrude: Option<Box<dyn Fn(&mut SarzakObjectStore, &mut DrawingObjectStore)>>,
}

impl DomainBuilder {
    /// Create a new instance of DomainBuilder
    ///
    pub fn new() -> Self {
        Self {
            nut_model: None,
            pre_extrude: None,
            post_extrude: None,
        }
    }

    /// Specify an input model
    ///
    /// Currently the only input we know how to process is a cuckoo model.
    pub fn cuckoo_model<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        let nut_model = FromModel::load_cuckoo_model(&path).context(FileOpenSnafu {
            path: path.as_ref(),
        })?;

        self.nut_model = Some(nut_model);
        Ok(self)
    }

    /// Pre-extrusion function
    ///
    /// This is where one would insert instances into the store upon which
    /// extrusion will be reliant.
    pub fn pre_extrude<F>(mut self, pre_extrude: F) -> Self
    where
        F: Fn(&mut SarzakObjectStore, &mut DrawingObjectStore) + 'static,
    {
        self.pre_extrude = Some(Box::new(pre_extrude));

        self
    }

    /// Post-extrusion function
    ///
    /// This is where one has the opportunity to create instances in the
    /// application domain.
    pub fn post_extrude<F>(mut self, post_extrude: F) -> Self
    where
        F: Fn(&mut SarzakObjectStore, &mut DrawingObjectStore) + 'static,
    {
        self.post_extrude = Some(Box::new(post_extrude));

        self
    }

    /// The final step
    ///
    /// Return the newly packaged domain
    pub fn build(mut self) -> Result<Domain> {
        ensure!(
            self.nut_model.is_some(),
            DomainBuilderSnafu {
                message: "you must specify a cucko model using `DomainBuilder::cuckoo_model`"
            }
        );
        let model = self.nut_model.unwrap();

        let mut sarzak = SarzakObjectStore::new();
        let mut drawing = DrawingObjectStore::new();

        // Run the pre_extrude function, if there is one.
        if let Some(ref func) = self.pre_extrude {
            func(&mut sarzak, &mut drawing);
        }

        // Convert the stores from nut-style to sarzak-style. We can't use into()
        // this time around because of the pre extrusion step. Note the name. 😀
        let mut context = SarzakContext {
            from: &model.sarzak,
            to: &mut sarzak,
        };
        SarzakObjectStore::extrude(model.sarzak.clone(), &mut context);

        // Run the postload function, if it exists.
        if let Some(ref func) = self.post_extrude {
            func(&mut sarzak, &mut drawing);
        }

        Ok(Domain::new(
            model.domain,
            model.description,
            sarzak,
            drawing,
        ))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Domain {
    version: String,
    domain: String,
    description: String,
    sarzak: SarzakObjectStore,
    drawing: DrawingObjectStore,
}

impl Domain {
    /// Create a new Domain
    ///
    /// This is used by the [`DomainBuilder`] to initialize a domain. It's not
    /// a generally useful means of creating a domain.
    pub(crate) fn new(
        domain: String,
        description: String,
        sarzak: SarzakObjectStore,
        drawing: DrawingObjectStore,
    ) -> Self {
        let domain = Domain {
            version: VERSION.to_owned(),
            domain: domain,
            description: description,
            sarzak,
            drawing,
        };

        domain
    }

    /// Create a Domain from a cuckoo file
    ///
    /// This is a funky method. It uses [`DomainBuilder`] to specifically build
    /// the sarzak metamodel.
    ///
    /// The name should change. This should be generally used to load a cuckoo
    /// model. How does a user use this for their application domain? Hm...
    pub fn init_sarzak<P: AsRef<Path>>(path: P) -> Result<Self> {
        DomainBuilder::new()
            .cuckoo_model(path)?
            .pre_extrude(|mut sarzak, mut drawing| {
                // Create instances of primitives missing from nut::sarzak that
                // the extrusion process depends upon.
                sarzak.inter_cardinality(Cardinality::One(ONE));
                sarzak.inter_cardinality(Cardinality::Many(MANY));

                sarzak.inter_conditionality(Conditionality::Conditional(CONDITIONAL));
                sarzak.inter_conditionality(Conditionality::Unconditional(UNCONDITIONAL));

                sarzak.inter_ty(Type::Integer(INTEGER));
                sarzak.inter_ty(Type::Boolean(BOOLEAN));
                sarzak.inter_ty(Type::Float(FLOAT));
                sarzak.inter_ty(Type::String(STRING));
                sarzak.inter_ty(Type::Uuid(UUID));
            })
            .build()
    }

    /// Return a reference to the sarzak store
    ///
    /// This returns a reference to the [`ObjectStore`] that contains the domain
    /// model instances.
    pub fn get_sarzak(&self) -> &SarzakObjectStore {
        &self.sarzak
    }
}

/// Extrude the ObjectStore containing the sarzak domain
///
/// We are extruding from the hand written `nut::sarzak::ObjectStore`, to the
/// generated `sarzak::sarzak::ObjectStore`. Note that there are some major
/// differences between the two, despite the similarity in names.
///
/// It's in this method that we need to start with the "root" objects and extrude
/// them. The "leaf" objects will be taken care of by the root extrusion.
/// There are two instances where this doesn't work: [`Attribute`]s and [`Subtype`]s.
/// In each case they are on the many side of a 1-M binary relationship. This
/// means that they are formalizing the relationship, and this is the only one
/// that they participate in.
///
/// This makes them leaves, without a root. We need to just inter them here.
impl Extrude<FromSarzakObjectStore, SarzakContext<'_>> for SarzakObjectStore {
    fn extrude(from: FromSarzakObjectStore, context: &mut SarzakContext) -> Self {
        // Iterate over the Attributes and extrude them.
        //
        for (_id, attr) in from.iter_attribute() {
            let new = Attribute::extrude(attr.clone(), context);
            context.to.inter_attribute(new);
        }

        // Iterate over the Subtypes and extrude them.
        //
        for (_id, sub) in from.iter_subtype() {
            let new = Subtype::extrude(sub.clone(), context);
            context.to.inter_subtype(new);
        }

        // Iterate over the Objects, and extrude them.
        //
        for (_id, obj) in from.iter_object() {
            let new = Object::extrude(obj.clone(), context);
            context.to.inter_object(new);
        }

        // Iterate over the Relationships, and extrude them.
        //
        for (_id, rel) in from.iter_relationship() {
            let new = Relationship::extrude(rel.clone(), context);
            context.to.inter_relationship(new);
        }

        // What's with all this cloning all of a sudden? I'm cloning all over
        // the place...
        context.to.clone()
    }
}

impl Extrude<FromDrawingObjectStore, DrawingContext<'_>> for DrawingObjectStore {
    fn extrude(from: FromDrawingObjectStore, context: &mut DrawingContext) -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_sarzak() {
        let sarzak = Domain::init_sarzak("models/sarzak.json");
        assert!(sarzak.is_ok());

        dbg!(sarzak);
    }

    #[test]
    fn test_builder() {
        let err = DomainBuilder::new().build();
        assert!(err.is_err());

        let ok = DomainBuilder::new()
            .cuckoo_model("models/sarzak.json")
            .unwrap()
            .build();
        assert!(ok.is_ok());
    }
}
