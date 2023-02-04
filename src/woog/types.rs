//! Types for instances of the "Woog" domain
//! # Domain Description
//!
//! Domain for generating code.
//!
//!
//! # Contents
//!
//! The following types are defined herein:
//!    * [`CRATE`]
//!    * [`ObjectMethod`]
//!    * [`Parameter`]
//!    * [`PRIVATE`]
//!    * [`PUBLIC`]
//!    * [`Visibility`]
//!
//! # Generated Code -- edit _with care_.
//!
//! Don't mess with anything between `{"magic":"","kind":"CriticalBlockBegin"}`
//! and `{"magic":"","kind":"CriticalBlockEnd"}`. Otherwise, you should be free
//! to go wild. Happy hacking!
//!
//! Use the following invocation to reproduce:
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
//! ```shell
//!  sarzak gen -d woog sarzak -d true -m true -i true -e false
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"1.0.0"}
use serde::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imports"}}}
use crate::woog::store::ObjectStore;
use crate::woog::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

// Imported Objects
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imported-objects"}}}
use crate::sarzak::types::Object;
use crate::sarzak::types::Type;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imported-objects"}}}

/// Crate Visibility
///
/// The item is visibile within the crate.
///
/// ❗️{"singleton_object": true}
///
//
pub const CRATE: Uuid = uuid!["496576fa-9b03-5488-8364-0ec9371a7570"];

/// Method
///
/// This represents a function's signature. We don't (yet) care about the body of the function
///. We are however very interested in it's type, which implies parameters and their types,
/// as well as our return type.
///
/// Looking at this more closely, I think that this should be related to a parameter list, and
/// the list related to the string of parameters. It may just be a nit, but it does bother me
/// a bit. I'll come back and fix it once it's less troublesome to generate this domain.
///
/// The function in question is a method, hanging off of an [`Object`][o].
///
/// [o][damn, now I need a documentation server].
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_method-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ObjectMethod {
    /// pub description: `std::string::String`,
    ///
    pub description: std::string::String,
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
    /// Imported from the sarzak domain.
    /// [`nut::sarzak::Object`]
    ///
    pub object: Uuid,
    /// pub param: `Option<Parameter>`,
    ///
    pub param: Option<Uuid>,
    /// Imported from the sarzak domain.
    /// [`nut::sarzak::Type`]
    ///
    pub ty: Uuid,
    /// pub visibility: `Visibility`,
    ///
    pub visibility: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object_method-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_method-new_impl"}}}
impl ObjectMethod {
    /// Inter a new ObjectMethod and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Type;
    /// # use sarzak::woog::Visibility;
    /// # use sarzak::woog::ObjectMethod;
    /// # use sarzak::woog::Parameter;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::woog::ObjectStore::new();
    /// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let type_uep = Type::test_default(&mut sarzak_store);
    ///
    /// let visibility_umv = Visibility::test_default(&mut store);
    /// let handsomely_steam = "protective_brain".to_owned();
    /// let parameter = Parameter::new(&mut store, None, &type_uep, &visibility_umv, handsomely_steam);
    /// let full_agreement = "unbecoming_songs".to_owned();
    /// let slippery_polish = "hanging_actor".to_owned();
    /// let easy_opinion = "homely_memory".to_owned();
    /// let object_bbl = Object::default();
    ///
    /// let type_ftz = Type::test_default(&mut sarzak_store);
    ///
    /// let visibility_nbf = Visibility::test_default(&mut store);
    /// let adhesive_quicksand = "green_snails".to_owned();
    /// let axiomatic_snake = "future_fan".to_owned();
    ///
    /// let object_method = ObjectMethod::new(&mut store, Some(&parameter), &object_bbl, &type_ftz, &visibility_nbf, adhesive_quicksand, axiomatic_snake);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        param: Option<&Parameter>,
        object: &Object,
        ty: &Type,
        visibility: &Visibility,
        name: std::string::String,
        description: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                //                 "{:?}::{:?}::{:?}::{}::{}::", //⚡️
                //                 param, object, ty, name, description, //⚡️
                "{:?}::{:?}::{:?}::{:?}::{}::{}::",
                param, object, ty, visibility, name, description,
            )
            .as_bytes(),
        );
        let new = Self {
            id,
            param: param.map(|o| o.id),
            object: object.id,
            ty: ty.get_id(),
            visibility: visibility.get_id(),
            name,
            description,
        };

        store.inter_object_method(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object_method-new_impl"}}}
}

/// Parameter
///
/// A parameter is an input to a function.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"parameter-struct-definition"}}}
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Parameter {
    /// pub id: `Uuid`,
    ///
    pub id: Uuid,
    /// pub name: `std::string::String`,
    ///
    pub name: std::string::String,
    /// pub next: `Option<Parameter>`,
    ///
    pub next: Option<Uuid>,
    /// Imported from the sarzak domain.
    /// [`nut::sarzak::Type`]
    ///
    pub ty: Uuid,
    /// pub visibility: `Visibility`,
    ///
    pub visibility: Uuid,
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"parameter-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"parameter-new_impl"}}}
impl Parameter {
    /// Inter a new Parameter and return it's `id`
    ///
    // {"magic":"","kind":{"IgnoreBlockBegin":{}}}
    /// # Example
    ///
    ///```
    /// # use sarzak::woog::Parameter;
    /// # use sarzak::sarzak::Type;
    /// # use sarzak::woog::Visibility;
    /// # let mut store = sarzak::woog::ObjectStore::new();
    /// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();

    ///
    /// let type_mmv = Type::test_default(&mut sarzak_store);
    ///
    /// let visibility_hpe = Visibility::test_default(&mut store);
    /// let flashy_sense = "last_weight".to_owned();
    ///
    /// let parameter = Parameter::new(&mut store, None, &type_mmv, &visibility_hpe, flashy_sense);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        next: Option<&Parameter>,
        ty: &Type,
        visibility: &Visibility,
        name: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            //             format!("{:?}::{:?}::{}::", next, ty, name,).as_bytes(), //⚡️
            format!("{:?}::{:?}::{:?}::{}::", next, ty, visibility, name,).as_bytes(),
        );
        let new = Self {
            id,
            next: next.map(|o| o.id),
            ty: ty.get_id(),
            visibility: visibility.get_id(),
            name,
        };

        store.inter_parameter(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"parameter-new_impl"}}}
}

/// Private Visibility
///
/// ❗️{"singleton_object": true}
///
//
pub const PRIVATE: Uuid = uuid!["beb60e80-67f8-54d2-a446-115683cf3b55"];

/// Public Visibility
///
/// ❗️{"singleton_object": true}
///
//
pub const PUBLIC: Uuid = uuid!["7e3aceaf-bc07-5661-b27e-5ec8ab1963fe"];

/// Item Visibility
///
/// This is a _very_ Rust-centric type. It represents the visibility levels that Rust surfaces
///.
///
/// Private is the default, and requires no modifiers. Public is the most open, and indicated
/// by prefixing the item with "pub". In the middle is "pub(crate)", which makes the item public
/// within the crate.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"visibility-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Visibility {
    /// `Private(Private)`,
    ///
    Private(Uuid),
    /// `Public(Public)`,
    ///
    Public(Uuid),
    /// `Crate(Crate)`,
    ///
    Crate(Uuid),
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"{}-enum-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"Visibility-enum-get-id-impl"}}}
impl Visibility {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::Private(z) => z,
            Self::Public(z) => z,
            Self::Crate(z) => z,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"Visibility-enum-get-id-impl"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"visibility-test_default"}}}
impl Visibility {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Private(PRIVATE);

        store.inter_visibility(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"visibility-test_default"}}}
