// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"tuple_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::enum_field::EnumField;
use crate::v2::lu_dog_vec::types::enum_field::EnumFieldEnum;
use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::value_type::ValueType;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-struct-documentation"}}}
/// A field that is a tuple.
///
/// Currently in this implementation we are cheating, as we don’t yet actually have tuples
/// . So this is limited to a single item.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TupleField {
    pub id: usize,
    /// R90: [`TupleField`] 'is constructed via' [`Expression`]
    pub expression: Option<usize>,
    /// R86: [`TupleField`] 'must have a type' [`ValueType`]
    pub ty: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-implementation"}}}
impl TupleField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-struct-impl-new"}}}
    /// Inter a new 'Tuple Field' in the store, and return it's `id`.
    pub fn new(
        expression: Option<&Rc<RefCell<Expression>>>,
        ty: &Rc<RefCell<ValueType>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<TupleField>> {
        store.inter_tuple_field(|id| {
            Rc::new(RefCell::new(TupleField {
                id,
                expression: expression.map(|expression| expression.borrow().id),
                ty: ty.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-struct-impl-nav-forward-cond-to-static_method"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-struct-impl-nav-forward-cond-to-expression"}}}
    /// Navigate to [`Expression`] across R90(1-*c)
    pub fn r90_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        span!("r90_expression");
        match self.expression {
            Some(ref expression) => vec![store.exhume_expression(&expression).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R86(1-*)
    pub fn r86_value_type<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<ValueType>>> {
        span!("r86_value_type");
        vec![store.exhume_value_type(&self.ty).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-impl-nav-subtype-to-supertype-enum_field"}}}
    // Navigate to [`EnumField`] across R85(isa)
    pub fn r85_enum_field<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<EnumField>>> {
        span!("r85_enum_field");
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-impl-nav-subtype-to-supertype-enum_field"}}}
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-impl-nav-subtype-to-supertype-enum_field"}}}
        vec![store
            .iter_enum_field()
            .find(|enum_field| {
                if let EnumFieldEnum::TupleField(id) = enum_field.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-implementation"}}}
impl PartialEq for TupleField {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression && self.ty == other.ty
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
