// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_value-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog::types::block::Block;
use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::span::Span;
use crate::v2::lu_dog::types::value_type::ValueType;
use crate::v2::lu_dog::types::variable::Variable;
use crate::v2::lu_dog::types::z_some::ZSome;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-documentation"}}}
/// A Value
///
/// A value has a Type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct XValue {
    pub subtype: XValueEnum,
    pub id: Uuid,
    /// R33: [`XValue`] '' [`Block`]
    pub block: Uuid,
    /// R24: [`XValue`] 'is decoded by a' [`ValueType`]
    pub ty: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum XValueEnum {
    Expression(Uuid),
    Variable(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-implementation"}}}
impl XValue {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-new_expression"}}}
    /// Inter a new XValue in the store, and return it's `id`.
    pub fn new_expression(
        block: &Arc<Mutex<Block>>,
        ty: &Arc<Mutex<ValueType>>,
        subtype: &Arc<Mutex<Expression>>,
        store: &mut LuDogStore,
    ) -> Arc<Mutex<XValue>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(XValue {
            block: block.lock().id,
            ty: ty.lock().id(),
            subtype: XValueEnum::Expression(subtype.lock().id()),
            id,
        }));
        store.inter_x_value(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-new_variable"}}}
    /// Inter a new XValue in the store, and return it's `id`.
    pub fn new_variable(
        block: &Arc<Mutex<Block>>,
        ty: &Arc<Mutex<ValueType>>,
        subtype: &Arc<Mutex<Variable>>,
        store: &mut LuDogStore,
    ) -> Arc<Mutex<XValue>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(XValue {
            block: block.lock().id,
            ty: ty.lock().id(),
            subtype: XValueEnum::Variable(subtype.lock().id),
            id,
        }));
        store.inter_x_value(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R33(1-*)
    pub fn r33_block<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<Mutex<Block>>> {
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R24(1-*)
    pub fn r24_value_type<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<Mutex<ValueType>>> {
        vec![store.exhume_value_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-backward-1_M-to-z_some"}}}
    /// Navigate to [`ZSome`] across R23(1-M)
    pub fn r23_z_some<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<Mutex<ZSome>>> {
        store
            .iter_z_some()
            .filter(|z_some| z_some.lock().inner == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R63(1-Mc)
    pub fn r63_span<'a>(&'a self, store: &'a LuDogStore) -> Vec<Arc<Mutex<Span>>> {
        store
            .iter_span()
            .filter_map(|span| {
                if span.lock().x_value == Some(self.id) {
                    Some(span)
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
