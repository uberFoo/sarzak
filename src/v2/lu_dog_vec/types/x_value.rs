// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"x_value-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::block::Block;
use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::span::Span;
use crate::v2::lu_dog_vec::types::value_type::ValueType;
use crate::v2::lu_dog_vec::types::variable::Variable;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-documentation"}}}
/// A Value
///
/// A value has a Type.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct XValue {
    pub subtype: XValueEnum,
    pub id: usize,
    /// R33: [`XValue`] '' [`Block`]
    pub block: usize,
    /// R24: [`XValue`] 'is decoded by a' [`ValueType`]
    pub ty: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum XValueEnum {
    Expression(usize),
    Variable(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-implementation"}}}
impl XValue {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-new_expression"}}}
    /// Inter a new XValue in the store, and return it's `id`.
    pub fn new_expression(
        block: &Rc<RefCell<Block>>,
        ty: &Rc<RefCell<ValueType>>,
        subtype: &Rc<RefCell<Expression>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<XValue>> {
        store.inter_x_value(|id| {
            Rc::new(RefCell::new(XValue {
                block: block.borrow().id,
                ty: ty.borrow().id,
                subtype: XValueEnum::Expression(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-new_variable"}}}
    /// Inter a new XValue in the store, and return it's `id`.
    pub fn new_variable(
        block: &Rc<RefCell<Block>>,
        ty: &Rc<RefCell<ValueType>>,
        subtype: &Rc<RefCell<Variable>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<XValue>> {
        store.inter_x_value(|id| {
            Rc::new(RefCell::new(XValue {
                block: block.borrow().id,
                ty: ty.borrow().id,
                subtype: XValueEnum::Variable(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-forward-to-block"}}}
    /// Navigate to [`Block`] across R33(1-*)
    pub fn r33_block<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Block>>> {
        span!("r33_block");
        vec![store.exhume_block(&self.block).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R24(1-*)
    pub fn r24_value_type<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r24_value_type");
        vec![store.exhume_value_type(&self.ty).unwrap()]
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-backward-1_M-to-z_some"}}}
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-struct-impl-nav-backward-1_Mc-to-span"}}}
    /// Navigate to [`Span`] across R63(1-Mc)
    pub fn r63_span<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Span>>> {
        span!("r63_span");
        store
            .iter_span()
            .filter(|span| span.borrow().x_value == Some(self.id))
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"x_value-implementation"}}}
impl PartialEq for XValue {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype && self.block == other.block && self.ty == other.ty
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
