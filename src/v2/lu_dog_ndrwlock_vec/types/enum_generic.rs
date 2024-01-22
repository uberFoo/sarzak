// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enum_generic-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::enumeration::Enumeration;
use crate::v2::lu_dog_ndrwlock_vec::types::value_type::ValueType;
use crate::v2::lu_dog_ndrwlock_vec::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnumGeneric {
    pub id: usize,
    pub name: String,
    /// R104: [`EnumGeneric`] 'parameterizes' [`Enumeration`]
    pub woog_enum: usize,
    /// R106: [`EnumGeneric`] 'next' [`EnumGeneric`]
    pub next: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-implementation"}}}
impl EnumGeneric {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-new"}}}
    /// Inter a new 'Enum Generic' in the store, and return it's `id`.
    pub fn new(
        name: String,
        woog_enum: &Arc<RwLock<Enumeration>>,
        next: Option<&Arc<RwLock<EnumGeneric>>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<EnumGeneric>> {
        store.inter_enum_generic(|id| {
            Arc::new(RwLock::new(EnumGeneric {
                id,
                name: name.to_owned(),
                woog_enum: woog_enum.read().unwrap().id,
                next: next.map(|enum_generic| enum_generic.read().unwrap().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-forward-to-is_paramaterized_by"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-forward-to-woog_enum"}}}
    /// Navigate to [`Enumeration`] across R104(1-*)
    pub fn r104_enumeration<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Enumeration>>> {
        vec![store.exhume_enumeration(&self.woog_enum).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`EnumGeneric`] across R106(1-*c)
    pub fn r106_enum_generic<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<EnumGeneric>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_enum_generic(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-backward-one-bi-cond-to-enum_generic"}}}
    /// Navigate to [`EnumGeneric`] across R106(1c-1c)
    pub fn r106c_enum_generic<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<EnumGeneric>>> {
        let enum_generic = store
            .iter_enum_generic()
            .find(|enum_generic| enum_generic.read().unwrap().next == Some(self.id));
        match enum_generic {
            Some(ref enum_generic) => vec![enum_generic.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-struct-impl-nav-backward-one-to-enumeration"}}}
    /// Navigate to [`Enumeration`] across R105(1-1)
    pub fn r105_enumeration<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Enumeration>>> {
        vec![store
            .iter_enumeration()
            .find(|enumeration| enumeration.read().unwrap().first_generic == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        vec![store
            .iter_value_type()
            .find(|value_type| {
                if let ValueTypeEnum::EnumGeneric(id) = value_type.read().unwrap().subtype {
                    id == self.id
                } else {
                    false
                }
            })
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_generic-implementation"}}}
impl PartialEq for EnumGeneric {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.woog_enum == other.woog_enum && self.next == other.next
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
