// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"struct_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog::types::data_structure::DataStructure;
use crate::v2::lu_dog::types::expression::Expression;
use crate::v2::lu_dog::types::expression::ExpressionEnum;
use crate::v2::lu_dog::types::field_expression::FieldExpression;
use crate::v2::lu_dog::types::x_path::XPath;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog::store::ObjectStore as LuDogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-documentation"}}}
/// A Structure Expression
///
/// This is how we create instances in dwarf.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StructExpression {
    pub bug: Uuid,
    pub id: Uuid,
    /// R39: [`StructExpression`] '' [`DataStructure`]
    pub data: Uuid,
    /// R96: [`StructExpression`] 'has a' [`XPath`]
    pub x_path: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-implementation"}}}
impl StructExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-new"}}}
    /// Inter a new 'Struct Expression' in the store, and return it's `id`.
    pub fn new(
        bug: Uuid,
        data: &Rc<RefCell<DataStructure>>,
        x_path: &Rc<RefCell<XPath>>,
        store: &mut LuDogStore,
    ) -> Rc<RefCell<StructExpression>> {
        let id = Uuid::new_v4();
        let new = Rc::new(RefCell::new(StructExpression {
            bug,
            id,
            data: data.borrow().id,
            x_path: x_path.borrow().id,
        }));
        store.inter_struct_expression(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-nav-forward-to-woog_struct"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-nav-forward-to-data"}}}
    /// Navigate to [`DataStructure`] across R39(1-*)
    pub fn r39_data_structure<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<DataStructure>>> {
        vec![store.exhume_data_structure(&self.data).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-nav-forward-to-x_path"}}}
    /// Navigate to [`XPath`] across R96(1-*)
    pub fn r96_x_path<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<XPath>>> {
        vec![store.exhume_x_path(&self.x_path).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-struct-impl-nav-backward-1_M-to-field_expression"}}}
    /// Navigate to [`FieldExpression`] across R26(1-M)
    pub fn r26_field_expression<'a>(
        &'a self,
        store: &'a LuDogStore,
    ) -> Vec<Rc<RefCell<FieldExpression>>> {
        store
            .iter_field_expression()
            .filter(|field_expression| field_expression.borrow().woog_struct == self.id)
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"struct_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::StructExpression(id) = expression.borrow().subtype {
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
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
