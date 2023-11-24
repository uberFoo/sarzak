// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"list_element-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_vec::types::list_expression::ListExpression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListElement {
    pub id: usize,
    pub position: i64,
    /// R55: [`ListElement`] 'points at an' [`Expression`]
    pub expression: usize,
    /// R53: [`ListElement`] 'follows' [`ListElement`]
    pub next: Option<usize>,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-implementation"}}}
impl ListElement {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-new"}}}
    /// Inter a new 'List Element' in the store, and return it's `id`.
    pub fn new(
        position: i64,
        expression: &Rc<RefCell<Expression>>,
        next: Option<&Rc<RefCell<ListElement>>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<ListElement>> {
        store.inter_list_element(|id| {
            Rc::new(RefCell::new(ListElement {
                id,
                position,
                expression: expression.borrow().id,
                next: next.map(|list_element| list_element.borrow().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R55(1-*)
    pub fn r55_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`ListElement`] across R53(1-*c)
    pub fn r53_list_element<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<ListElement>>> {
        match self.next {
            Some(ref next) => vec![store.exhume_list_element(&next).unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-backward-one-bi-cond-to-list_element"}}}
    /// Navigate to [`ListElement`] across R53(1c-1c)
    pub fn r53c_list_element<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<ListElement>>> {
        let list_element = store
            .iter_list_element()
            .find(|list_element| list_element.borrow().next == Some(self.id));
        match list_element {
            Some(ref list_element) => vec![list_element.clone()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-backward-one-to-list_expression"}}}
    /// Navigate to [`ListExpression`] across R54(1-1)
    pub fn r54_list_expression<'a>(
        &'a self,
        store: &'a LuDogVecStore,
    ) -> Vec<Rc<RefCell<ListExpression>>> {
        vec![store
            .iter_list_expression()
            .find(|list_expression| list_expression.borrow().elements == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::ListElement(id) = expression.borrow().subtype {
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-implementation"}}}
impl PartialEq for ListElement {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.expression == other.expression
            && self.next == other.next
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
