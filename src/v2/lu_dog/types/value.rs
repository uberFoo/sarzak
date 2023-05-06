// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-use-statements"}}}
use std::sync::{Arc, RwLock};

use uuid::Uuid;

use crate::v2::lu_dog::types::block::Block;
use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::value_type::ValueType;
use crate::v2::lu_dog::types::variable::Variable;
use crate::v2::lu_dog::types::z_some::ZSome;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-hybrid-documentation"}}}
/// A Value
///
/// A value has a Type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Value {
    pub subtype: ValueEnum,
    pub id: Uuid,
    /// R33: [`Value`] '' [`Block`]
    pub block: Uuid,
    /// R24: [`Value`] 'is decoded by a' [`ValueType`]
    pub ty: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ValueEnum {
    Expression(Uuid),
    Variable(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-implementation"}}}
impl Value {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-new_expression"}}}
    /// Inter a new Value in the store, and return it's `id`.
    pub fn new_expression(
        block: &Arc<RwLock<Block>>,
        ty: &Arc<RwLock<ValueType>>,
        subtype: &Arc<RwLock<Expression>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Value>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Value {
            block: block.read().unwrap().id,
            ty: ty.read().unwrap().id(),
            subtype: ValueEnum::Expression(subtype.read().unwrap().id()),
            id,
        }));
        store.inter_value(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-new_variable"}}}
    /// Inter a new Value in the store, and return it's `id`.
    pub fn new_variable(
        block: &Arc<RwLock<Block>>,
        ty: &Arc<RwLock<ValueType>>,
        subtype: &Arc<RwLock<Variable>>,
        store: &mut LuDogStore,
    ) -> Arc<RwLock<Value>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Value {
            block: block.read().unwrap().id,
            ty: ty.read().unwrap().id(),
            subtype: ValueEnum::Variable(subtype.read().unwrap().id),
            id,
        }));
        store.inter_value(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R33(1-*)
    pub fn r33_block<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<Block>>> {
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R24(1-*)
    pub fn r24_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<ValueType>>> {
        vec![store.exhume_value_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-nav-backward-1_M-to-z_some"}}}
    /// Navigate to [`ZSome`] across R23(1-M)
    pub fn r23_z_some<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<RwLock<ZSome>>> {
        store
            .iter_z_some()
            .filter(|z_some| z_some.read().unwrap().inner == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
