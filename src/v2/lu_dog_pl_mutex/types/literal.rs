// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-use-statements"}}}
use parking_lot::Mutex;
use std::sync::Arc;
use uuid::Uuid;

use crate::v2::lu_dog_pl_mutex::types::boolean_literal::BooleanLiteral;
use crate::v2::lu_dog_pl_mutex::types::expression::Expression;
use crate::v2::lu_dog_pl_mutex::types::expression::ExpressionEnum;
use crate::v2::lu_dog_pl_mutex::types::float_literal::FloatLiteral;
use crate::v2::lu_dog_pl_mutex::types::integer_literal::IntegerLiteral;
use crate::v2::lu_dog_pl_mutex::types::string_literal::StringLiteral;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_pl_mutex::store::ObjectStore as LuDogPlMutexStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-documentation"}}}
/// A Literal Expression
///
/// This is any literal value in the program.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Literal {
    pub subtype: LiteralEnum,
    pub bogus: bool,
    pub id: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum LiteralEnum {
    BooleanLiteral(Uuid),
    FloatLiteral(Uuid),
    IntegerLiteral(Uuid),
    StringLiteral(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-implementation"}}}
impl Literal {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_boolean_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_boolean_literal(
        bogus: bool,
        subtype: &Arc<Mutex<BooleanLiteral>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Literal>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Literal {
            bogus: bogus,
            subtype: LiteralEnum::BooleanLiteral(subtype.lock().id), // b
            id,
        }));
        store.inter_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_float_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_float_literal(
        bogus: bool,
        subtype: &Arc<Mutex<FloatLiteral>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Literal>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Literal {
            bogus: bogus,
            subtype: LiteralEnum::FloatLiteral(subtype.lock().id), // b
            id,
        }));
        store.inter_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_integer_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_integer_literal(
        bogus: bool,
        subtype: &Arc<Mutex<IntegerLiteral>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Literal>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Literal {
            bogus: bogus,
            subtype: LiteralEnum::IntegerLiteral(subtype.lock().id), // b
            id,
        }));
        store.inter_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_string_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_string_literal(
        bogus: bool,
        subtype: &Arc<Mutex<StringLiteral>>,
        store: &mut LuDogPlMutexStore,
    ) -> Arc<Mutex<Literal>> {
        let id = Uuid::new_v4();
        let new = Arc::new(Mutex::new(Literal {
            bogus: bogus,
            subtype: LiteralEnum::StringLiteral(subtype.lock().id), // b
            id,
        }));
        store.inter_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogPlMutexStore,
    ) -> Vec<Arc<Mutex<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Literal(id) = expression.lock().subtype {
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
