//! Types for instances of the "Woog" domain
//! # Domain Description
//!
//! Domain for generating code.
//!
//!
//! # Contents
//!
//! The following types are defined herein:
//!    * [`BORROWED`]
//!    * [`CRATE`]
//!    * [`Mutability`]
//!    * [`MUTABLE`]
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
use crate::woog_v1::store::ObjectStore;
use crate::woog_v1::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

// Imported Objects
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imported-objects"}}}
use crate::sarzak_v1::types::Object;
use crate::sarzak_v1::types::Type;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imported-objects"}}}

/// Borrowed
///
/// The type is declared as borrowed.
///
/// ❗️{"singleton_object": true}
///
//
pub const BORROWED: Uuid = uuid!["bc521a10-95c0-55b3-9387-5e2691291f9c"];

/// Crate Visibility
///
/// The item is visibile within the crate.
///
/// ❗️{"singleton_object": true}
///
//
pub const CRATE: Uuid = uuid!["496576fa-9b03-5488-8364-0ec9371a7570"];

/// Type Mutability
///
/// This is tied closely with Rust. There are two possible options: mutable and borrowed.
///
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"mutability-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Mutability {
    /// `Borrowed(Borrowed)`,
    ///
    Borrowed(Uuid),
    /// `Mutable(Mutable)`,
    ///
    Mutable(Uuid),
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"{}-enum-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"Mutability-enum-get-id-impl"}}}
impl Mutability {
    pub fn get_id(&self) -> Uuid {
        match *self {
            Self::Borrowed(z) => z,
            Self::Mutable(z) => z,
        }
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"Mutability-enum-get-id-impl"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"mutability-test_default"}}}
impl Mutability {
    pub fn test_default(store: &mut ObjectStore) -> Self {
        let test = Self::Borrowed(BORROWED);

        store.inter_mutability(test.clone());

        test
    }
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"mutability-test_default"}}}

/// Mutable
///
/// The type is declared as `mut`.
///
/// ❗️{"singleton_object": true}
///
//
pub const MUTABLE: Uuid = uuid!["5380da57-2761-5f77-864e-2eb366eeb83d"];

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
    /// # use sarzak::sarzak_v1::Type;
    /// # use sarzak::woog_v1::Visibility;
    /// # use sarzak::woog_v1::Mutability;
    /// # use sarzak::woog_v1::Parameter;
    /// # use sarzak::woog_v1::ObjectMethod;
    /// # use sarzak::sarzak_v1::Object;
    /// # let mut store = sarzak::woog_v1::ObjectStore::new();
    /// # let mut sarzak_store = sarzak::sarzak_v1::ObjectStore::new();
    ///
    /// let mutability_qxl = Mutability::test_default(&mut store);
    /// let type_fgd = Type::test_default(&mut sarzak_store);
    ///
    /// let visibility_ssb = Visibility::test_default(&mut store);
    /// let telling_jellyfish = "condemned_zinc".to_owned();
    /// let parameter = Parameter::new(&mut store, &mutability_qxl, None, &type_fgd, &visibility_ssb, telling_jellyfish);
    /// let dispensable_teeth = "thundering_purpose".to_owned();
    /// let upset_stage = "windy_meal".to_owned();
    /// let heavy_jam = "dashing_show".to_owned();
    /// let object_ndm = Object::default();
    ///
    /// let type_qsj = Type::test_default(&mut sarzak_store);
    ///
    /// let visibility_vmr = Visibility::test_default(&mut store);
    /// let vigorous_son = "reminiscent_cry".to_owned();
    /// let messy_destruction = "highfalutin_leg".to_owned();
    ///
    /// let object_method = ObjectMethod::new(&mut store, Some(&parameter), &object_ndm, &type_qsj, &visibility_vmr, vigorous_son, messy_destruction);
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
    /// pub mutability: `Mutability`,
    ///
    pub mutability: Uuid,
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
    /// # use sarzak::sarzak_v1::Type;
    /// # use sarzak::woog_v1::Mutability;
    /// # use sarzak::woog_v1::Parameter;
    /// # use sarzak::woog_v1::Visibility;
    /// # let mut store = sarzak::woog_v1::ObjectStore::new();
    /// # let mut sarzak_store = sarzak::sarzak_v1::ObjectStore::new();
    ///
    /// let mutability_qhl = Mutability::test_default(&mut store);
    /// let type_bgv = Type::test_default(&mut sarzak_store);
    ///
    /// let visibility_avh = Visibility::test_default(&mut store);
    /// let important_trade = "cloudy_stocking".to_owned();
    ///
    /// let parameter = Parameter::new(&mut store, &mutability_qhl, None, &type_bgv, &visibility_avh, important_trade);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        mutability: &Mutability,
        next: Option<&Parameter>,
        ty: &Type,
        visibility: &Visibility,
        name: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            //             format!("{:?}::{:?}::{}::", next, ty, name,).as_bytes(), //⚡️
            //             format!("{:?}::{:?}::{:?}::{}::", next, ty, visibility, name,).as_bytes(), //⚡️
            format!(
                "{:?}::{:?}::{:?}::{:?}::{}::",
                mutability, next, ty, visibility, name,
            )
            .as_bytes(),
        );
        let new = Self {
            id,
            mutability: mutability.get_id(),
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
