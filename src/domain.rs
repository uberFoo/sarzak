//! Sarzak Domain
//!
//! A Domain is a container for items that all participate in the same abstraction.
//! Currently that means a model.
use std::path::Path;

use log;
use nut::{
    codegen::{
        DrawingObjectStore as FromDrawingStore, Extrude, SarzakObjectStore as FromSarzakStore,
    },
    sarzak::SarzakModel as FromModel,
};
use snafu::prelude::*;
use uuid::Uuid;

use crate::{
    error::{DomainBuilderSnafu, FileOpenSnafu, Result},
    v1::{
        domain::Domain as DomainV1,
        drawing::{
            store::ObjectStore as DrawingV1Store,
            types::{Context as DrawingContext, ObjectUi, RelationshipUi},
        },
        sarzak::{
            store::ObjectStore as SarzakV1Store,
            types::{Attribute, Context as SarzakContext, Object, Relationship, Subtype, Type},
        },
    },
    v2::domain::Domain as DomainV2,
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
/// # use sarzak::v1::sarzak::{Cardinality, ONE, MANY};
/// # use sarzak::domain::DomainBuilder;
/// # const SARZAK_MODEL:&str = "models/sarzak.json";
/// let sarzak = DomainBuilder::new()
///     .cuckoo_model(SARZAK_MODEL).unwrap()
///     .pre_load(|sarzak_from, drawing_from, mut sarzak_to, mut drawing_to| {
///         // Create instances of primitives missing from nut::sarzak that
///         // the extrusion process depends upon.
///         sarzak_to.inter_cardinality(Cardinality::One(ONE));
///         sarzak_to.inter_cardinality(Cardinality::Many(MANY));
///     })
///     .build_v1().unwrap();
///
/// let store = sarzak.sarzak();
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
    pre_load: Option<
        Box<dyn Fn(&FromSarzakStore, &FromDrawingStore, &mut SarzakV1Store, &mut DrawingV1Store)>,
    >,
    post_load: Option<Box<dyn Fn(&mut SarzakV1Store, &mut DrawingV1Store)>>,
}

impl DomainBuilder {
    /// Create a new instance of DomainBuilder
    ///
    pub fn new() -> Self {
        Self {
            nut_model: None,
            pre_load: None,
            post_load: None,
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

    /// Pre-load function
    ///
    /// This is where one would insert instances into either the sarzak or the
    /// drawing store (or both) before the application domain is loaded.
    /// This is in fact exactly how [those stores are populated][✨].
    ///
    /// [✨]: crate::domain::Domain::init_sarzak
    pub fn pre_load<F>(mut self, pre_load: F) -> Self
    where
        F: Fn(&FromSarzakStore, &FromDrawingStore, &mut SarzakV1Store, &mut DrawingV1Store)
            + 'static,
    {
        self.pre_load = Some(Box::new(pre_load));

        self
    }

    /// Post-load function
    ///
    /// This is where one has the opportunity to create instances in the
    /// application domain.
    pub fn post_load<F>(mut self, post_load: F) -> Self
    where
        F: Fn(&mut SarzakV1Store, &mut DrawingV1Store) + 'static,
    {
        self.post_load = Some(Box::new(post_load));

        self
    }

    fn _build_v1(self) -> DomainV1 {
        let model = self.nut_model.unwrap();

        let mut sarzak = SarzakV1Store::new();
        let mut drawing = DrawingV1Store::new();

        // Run the pre_extrude function, if there is one.
        if let Some(ref func) = self.pre_load {
            log::debug!("executing preload function");
            func(&model.sarzak, &model.drawing, &mut sarzak, &mut drawing);
        }

        log::debug!("loading and converting domain: {}", model.domain);
        // This is where the real work happens.
        extrude_cuckoo_domain(&model.sarzak, &model.drawing, &mut sarzak, &mut drawing);

        // Run the post_extrude function, if it exists.
        if let Some(ref func) = self.post_load {
            log::debug!("executing postload function");
            func(&mut sarzak, &mut drawing);
        }

        DomainV1::new(model, sarzak, drawing)
    }

    /// The final step
    ///
    /// Return the newly packaged domain
    pub fn build_v1(self) -> Result<DomainV1> {
        ensure!(
            self.nut_model.is_some(),
            DomainBuilderSnafu {
                message: "you must specify a cuckoo model using `DomainBuilder::cuckoo_model`"
            }
        );

        Ok(self._build_v1())
    }

    /// The final step
    ///
    /// Return the newly packaged domain
    pub fn build_v2(self) -> Result<DomainV2> {
        ensure!(
            self.nut_model.is_some(),
            DomainBuilderSnafu {
                message: "you must specify a cuckoo model using `DomainBuilder::cuckoo_model`"
            }
        );

        Ok(self._build_v1().into())
    }
}

fn extrude_cuckoo_domain(
    sarzak_from: &FromSarzakStore,
    drawing_from: &FromDrawingStore,
    sarzak_to: &mut SarzakV1Store,
    drawing_to: &mut DrawingV1Store,
) {
    crate::v1::sarzak::init_instances(sarzak_to);

    // Extrude the instances in the sarzak domain
    let mut context = SarzakContext {
        from: sarzak_from,
        to: sarzak_to,
    };
    SarzakV1Store::extrude(sarzak_from.clone(), &mut context);

    // I added a new type, and now we need to create instances of it.
    // Note that we are doing this all in the context of the new
    // domain, since it's now complete. This is really a post-extrusion
    // step, and maybe belongs there. It's here because this is a legit
    // part of the model, that really could have been done by cuckoo, had
    // I had the forethought.
    //
    // It's pretty neat that I can use all the macro-goodness in the new
    // version.
    //
    // We are doing all the cloning so that we can mutably borrow `context.to`.
    let objects: Vec<(Uuid, Object)> = sarzak_to
        .iter_object()
        .into_iter()
        .map(|(u, o)| (*u, o.clone()))
        .collect();
    for (_id, obj) in &objects {
        // Create on owned type for each object
        let ty = Type::Object(obj.id);
        sarzak_to.inter_ty(ty);
    }

    crate::v1::drawing::init_instances(drawing_to);

    // Extrude the instances in the drawing domain
    let mut context = DrawingContext {
        from: drawing_from,
        to: drawing_to,
        sarzak: sarzak_to,
    };
    DrawingV1Store::extrude(drawing_from.clone(), &mut context);
}

/// Extrude the ObjectStore containing the sarzak domain
///
/// We are extruding from the hand written `nut::sarzak::ObjectStore`, to the
/// generated `sarzak::sarzak::ObjectStore`. Note that there are some major
/// differences between the two, despite the similarity in names. In this
/// method we are only extruding the sarzak domain artifacts. There is a different
/// method that extrudes the artifacts from the drawing domain.
///
/// It's in this method that we need to start with the "root" objects and extrude
/// them. The "leaf" objects will be taken care of by the root extrusion.
/// There are two instances where this doesn't work: [`Attribute`]s and [`Subtype`]s.
/// In each case they are on the many side of a 1-M binary relationship. This
/// means that they are formalizing the relationship, and this is the only one
/// that they participate in.
///
/// This makes them leaves, without a root. We need to just inter them here.
impl Extrude<FromSarzakStore, SarzakContext<'_>> for SarzakV1Store {
    fn extrude(from: FromSarzakStore, context: &mut SarzakContext) -> Self {
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

/// Extrude the ObjectStore containing the sarzak domain
///
/// We are extruding from the hand written `nut::sarzak::ObjectStore`, to the
/// generated `sarzak::sarzak::ObjectStore`. Note that there are some major
/// differences between the two, despite the similarity in names. In this
/// method we are extruding the the artifacts from the drawing domain. That
/// means points, and x, and y, and whatnot.
///
/// As above we insert anything that doesn't have an incoming relationship.
impl Extrude<FromDrawingStore, DrawingContext<'_>> for DrawingV1Store {
    fn extrude(from: FromDrawingStore, context: &mut DrawingContext) -> Self {
        // Extrude RelationshipUI
        //
        for (_id, rui) in from.iter_relationship_ui() {
            let new = RelationshipUi::extrude(rui.clone(), context);
            context.to.inter_relationship_ui(new);
        }

        // Extrude ObjectUi
        //
        for (_id, oui) in from.iter_object_ui() {
            let new = ObjectUi::extrude(oui.clone(), context);
            context.to.inter_object_ui(new);
        }

        context.to.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;

    #[test]
    fn test_load_sarzak() {
        let _ = env_logger::builder().is_test(true).try_init();

        let sarzak = DomainBuilder::new()
            .cuckoo_model("models/sarzak.json")
            .unwrap()
            .build_v1();
        assert!(sarzak.is_ok());
    }

    #[test]
    fn test_builder() {
        let _ = env_logger::builder().is_test(true).try_init();

        let err = DomainBuilder::new().build_v1();
        assert!(err.is_err());

        let ok = DomainBuilder::new()
            .cuckoo_model("models/drawing.json")
            .unwrap()
            .build_v1();
        assert!(ok.is_ok());
    }
}
