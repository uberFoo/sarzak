// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"list_element-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-use-statements"}}}
use no_deadlocks::RwLock;
use std::sync::Arc;
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_ndrwlock_vec::types::expression::Expression;
use crate::v2::lu_dog_ndrwlock_vec::types::expression::ExpressionEnum;
use crate::v2::lu_dog_ndrwlock_vec::types::list_expression::ListExpression;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_ndrwlock_vec::store::ObjectStore as LuDogNdrwlockVecStore;
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
        expression: &Arc<RwLock<Expression>>,
        next: Option<&Arc<RwLock<ListElement>>>,
        store: &mut LuDogNdrwlockVecStore,
    ) -> Arc<RwLock<ListElement>> {
        store.inter_list_element(|id| {
            Arc::new(RwLock::new(ListElement {
                id,
                position,
                expression: expression.read().unwrap().id,
                next: next.map(|list_element| list_element.read().unwrap().id),
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R55(1-*)
    pub fn r55_expression<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r55_expression");
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-struct-impl-nav-forward-cond-to-next"}}}
    /// Navigate to [`ListElement`] across R53(1-*c)
    pub fn r53_list_element<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<ListElement>>> {
        span!("r53_list_element");
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
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<ListElement>>> {
        span!("r53_list_element");
        let list_element = store
            .iter_list_element()
            .find(|list_element| list_element.read().unwrap().next == Some(self.id));
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
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<ListExpression>>> {
        span!("r54_list_expression");
        vec![store
            .iter_list_expression()
            .find(|list_expression| list_expression.read().unwrap().elements == Some(self.id))
            .unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"list_element-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogNdrwlockVecStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::ListElement(id) = expression.read().unwrap().subtype {
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
