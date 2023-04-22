// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"value-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-use-statements"}}}
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
        block: &Block,
        ty: &ValueType,
        subtype: &Expression,
        store: &mut LuDogStore,
    ) -> Value {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id();
        let new = Value {
            block: block.id,
            ty: ty.id(),
            subtype: ValueEnum::Expression(subtype.id()),
            id,
        };
        store.inter_value(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-new_expression_"}}}
    /// Inter a new Value in the store, and return it's `id`.
    pub fn new_expression_(block: &Block, ty: &ValueType, subtype: &Expression) -> Value {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id();
        let new = Value {
            block: block.id,
            ty: ty.id(),
            subtype: ValueEnum::Expression(subtype.id()),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-new_variable"}}}
    /// Inter a new Value in the store, and return it's `id`.
    pub fn new_variable(
        block: &Block,
        ty: &ValueType,
        subtype: &Variable,
        store: &mut LuDogStore,
    ) -> Value {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Value {
            block: block.id,
            ty: ty.id(),
            subtype: ValueEnum::Variable(subtype.id),
            id,
        };
        store.inter_value(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-new_variable_"}}}
    /// Inter a new Value in the store, and return it's `id`.
    pub fn new_variable_(block: &Block, ty: &ValueType, subtype: &Variable) -> Value {
        // 🚧 I'm not using id below with subtype because that's rendered where it doesn't know
        // about this local. This should be fixed in the near future.
        let id = subtype.id;
        let new = Value {
            block: block.id,
            ty: ty.id(),
            subtype: ValueEnum::Variable(subtype.id),
            id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R33(1-*)
    pub fn r33_block<'a>(&'a self, store: &'a LuDogStore) -> Vec<&Block> {
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R24(1-*)
    pub fn r24_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<&ValueType> {
        vec![store.exhume_value_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-nav-backward-1_M-to-some"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"value-struct-impl-nav-backward-1_M-to-z_some"}}}
    /// Navigate to [`ZSome`] across R23(1-M)
    pub fn r23_z_some<'a>(&'a self, store: &'a LuDogStore) -> Vec<&ZSome> {
        store
            .iter_z_some()
            .filter_map(|z_some| {
                if z_some.inner == self.id {
                    Some(z_some)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
