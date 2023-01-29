//! Types for instances of the "Woog" domain
//! # Domain Description
//!
//! Domain for generating code.
//!
//!
//! # Contents
//!
//! The following types are defined herein:
//!    * [`ObjectMethod`]
//!    * [`Parameter`]
//!
//! # Generated Code -- edit _with care_.
//!
//! Don't mess with anything between `{"magic":"","kind":"CriticalBlockBegin"}`
//! and `{"magic":"","kind":"CriticalBlockEnd"}`. Otherwise, you should be free
//! to go wild. Happy hacking!
//!
//! Use the following invocation to reproduce:
// {"magic":"","kind":"IgnoreBlockBegin"}
//! ```shell
//!  sarzak gen -d woog sarzak -e false -m true -d true -i true
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"1.0.0"}
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imports"}}}
use crate::woog::store::ObjectStore;
use crate::woog::UUID_NS;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imports"}}}

// Imported Objects
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"imported-objects"}}}
// use sarzak::sarzak::Object; //⚡️
// use sarzak::sarzak::Type; //⚡️
// use crate::sarzak::types::Object; //⚡️
// use crate::sarzak::types::Type; //⚡️
// use sarzak::sarzak::Object; //⚡️
// use sarzak::sarzak::Type; //⚡️
use crate::sarzak::types::Object;
use crate::sarzak::types::Type;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"imported-objects"}}}

/// Method
///
/// This represents a function's signature. We don't (yet) care about the body of the function
///. We are however very interested in it's type, which implies parameters and their types,
/// as well as our return type.
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
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"object_method-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"object_method-new_impl"}}}
impl ObjectMethod {
    /// Inter a new ObjectMethod and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin" "is_uber": true}
    /// # Example
    ///
    ///```
    /// # use sarzak::sarzak::Type;
    /// # use sarzak::woog::ObjectMethod;
    /// # use sarzak::woog::Parameter;
    /// # use sarzak::sarzak::Object;
    /// # let mut store = sarzak::woog::ObjectStore::new();
    /// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let type_gnb = Type::test_default(&mut sarzak_store);
    ///
    /// let adventurous_faucet = "jumpy_cat".to_owned();
    /// let parameter = Parameter::new(&mut store, None, &type_gnb, adventurous_faucet);
    /// let left_seashore = "oval_hope".to_owned();
    /// let irritating_snow = "obese_action".to_owned();
    /// let debonair_robin = "salty_fruit".to_owned();
    /// let object_sxc = Object::default();
    ///
    /// let type_yjb = Type::test_default(&mut sarzak_store);
    ///
    /// let wandering_fire = "regular_fly".to_owned();
    /// let gaping_lip = "hungry_stem".to_owned();
    ///
    /// let object_method = ObjectMethod::new(&mut store, Some(&parameter), &object_sxc, &type_yjb, wandering_fire, gaping_lip);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        param: Option<&Parameter>,
        object: &Object,
        ty: &Type,
        name: std::string::String,
        description: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!(
                "{:?}::{:?}::{:?}::{}::{}::",
                param, object, ty, name, description,
            )
            .as_bytes(),
        );
        let new = Self {
            id,
            param: param.map(|o| o.id),
            object: object.id,
            ty: ty.get_id(),
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
}
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"parameter-struct-definition"}}}

// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"parameter-new_impl"}}}
impl Parameter {
    /// Inter a new Parameter and return it's `id`
    ///
    // {"magic":"","kind":"IgnoreBlockBegin" "is_uber": true}
    /// # Example
    ///
    ///```
    /// # use sarzak::woog::Parameter;
    /// # use sarzak::sarzak::Type;
    /// # let mut store = sarzak::woog::ObjectStore::new();
    /// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
    ///
    /// let type_wft = Type::test_default(&mut sarzak_store);
    ///
    /// let noxious_calendar = "pointless_stream".to_owned();
    ///
    /// let parameter = Parameter::new(&mut store, None, &type_wft, noxious_calendar);
    ///```
    // {"magic":"","kind":"IgnoreBlockEnd"}
    pub fn new(
        store: &mut ObjectStore,
        next: Option<&Parameter>,
        ty: &Type,
        name: std::string::String,
    ) -> Self {
        let id = Uuid::new_v5(
            &UUID_NS,
            format!("{:?}::{:?}::{}::", next, ty, name,).as_bytes(),
        );
        let new = Self {
            id,
            next: next.map(|o| o.id),
            ty: ty.get_id(),
            name,
        };

        store.inter_parameter(new.clone());

        new
    }
    // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"parameter-new_impl"}}}
}
