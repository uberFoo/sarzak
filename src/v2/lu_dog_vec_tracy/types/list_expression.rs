// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"list_expression-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_vec_tracy::types::expression::Expression;
use crate::v2::lu_dog_vec_tracy::types::expression::ExpressionEnum;
use crate::v2::lu_dog_vec_tracy::types::list_element::ListElement;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec_tracy::store::ObjectStore as LuDogVecTracyStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-documentation"}}}
/// A list of expressions
///
/// E.g., `let a = [0, 1, 2, 3];`
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListExpression {
    pub id: usize,
    /// R54: [`ListExpression`] 'contains' [`ListElement`]
    pub elements: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-implementation"}}}
impl ListExpression {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-impl-new"}}}
    /// Inter a new 'List Expression' in the store, and return it's `id`.
    pub fn new(
        elements: Option<&Rc<RefCell<ListElement>>>,
        store: &mut LuDogVecTracyStore,
    ) -> Rc<RefCell<ListExpression>> {
        store.inter_list_expression(|id| {
            Rc::new(RefCell::new(ListExpression {
                id,
                elements: elements.map(|list_element| list_element.borrow().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-impl-nav-forward-cond-to-elements"}}}
    /// Navigate to [`ListElement`] across R54(1-*c)
    pub fn r54_list_element<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<ListElement>>> {
        span!("r54_list_element");
        match self.elements {
            Some(ref elements) => vec![store.exhume_list_element(&elements).unwrap()],
            // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
            // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-struct-impl-nav-backward-one-bi-cond-to-format_string"}}}
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogVecTracyStore,
    ) -> Vec<Rc<RefCell<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::ListExpression(id) = expression.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_expression-implementation"}}}
impl PartialEq for ListExpression {
    fn eq(&self, other: &Self) -> bool {
        self.elements == other.elements
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
