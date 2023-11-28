// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"field_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::expression::Expression;
use crate::v2::lu_dog_vec_tracy::types::expression::ExpressionEnum;
use crate::v2::lu_dog_vec_tracy::types::named_field_expression::NamedFieldExpression;
use crate::v2::lu_dog_vec_tracy::types::struct_expression::StructExpression;
use crate::v2::lu_dog_vec_tracy::types::unnamed_field_expression::UnnamedFieldExpression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-hybrid-documentation"}}}
/// A Struct Field Expression
///
/// This assigns a value to a field in a structure.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FieldExpression {
    pub subtype: FieldExpressionEnum,
    pub id: usize,
    /// R38: [`FieldExpression`] '' [`Expression`]
    pub expression: usize,
    /// R26: [`FieldExpression`] 'belongs to a' [`StructExpression`]
    pub woog_struct: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum FieldExpressionEnum {
    NamedFieldExpression(usize),
    UnnamedFieldExpression(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-implementation"}}}
impl FieldExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-impl-new_named_field_expression"}}}
    /// Inter a new FieldExpression in the store, and return it's `id`.
    pub fn new_named_field_expression(
        expression: &Rc<RefCell<Expression>>,
        woog_struct: &Rc<RefCell<StructExpression>>,
        subtype: &Rc<RefCell<NamedFieldExpression>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<FieldExpression>> {
        store.inter_field_expression(|id| {
            Rc::new(RefCell::new(FieldExpression {
                expression: expression.borrow().id,
                woog_struct: woog_struct.borrow().id,
                subtype: FieldExpressionEnum::NamedFieldExpression(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-impl-new_unnamed_field_expression"}}}
    /// Inter a new FieldExpression in the store, and return it's `id`.
    pub fn new_unnamed_field_expression(
        expression: &Rc<RefCell<Expression>>,
        woog_struct: &Rc<RefCell<StructExpression>>,
        subtype: &Rc<RefCell<UnnamedFieldExpression>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<FieldExpression>> {
        store.inter_field_expression(|id| {
            Rc::new(RefCell::new(FieldExpression {
                expression: expression.borrow().id,
                woog_struct: woog_struct.borrow().id,
                subtype: FieldExpressionEnum::UnnamedFieldExpression(subtype.borrow().id), // b
                id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R38(1-*)
    pub fn r38_expression<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<Expression>>> {
        span!("r38_expression");
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-struct-impl-nav-forward-to-woog_struct"}}}
    /// Navigate to [`StructExpression`] across R26(1-*)
    pub fn r26_struct_expression<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<StructExpression>>> {
        span!("r26_struct_expression");
        vec![store.exhume_struct_expression(&self.woog_struct).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::FieldExpression(id) = expression.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"field_expression-implementation"}}}
impl PartialEq for FieldExpression {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
            && self.expression == other.expression
            && self.woog_struct == other.woog_struct
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
