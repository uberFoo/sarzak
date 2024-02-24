// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"literal-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-use-statements"}}}
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

use crate::v2::lu_dog_rwlock::types::boolean_literal::BooleanLiteral;
use crate::v2::lu_dog_rwlock::types::char_literal::CharLiteral;
use crate::v2::lu_dog_rwlock::types::expression::Expression;
use crate::v2::lu_dog_rwlock::types::expression::ExpressionEnum;
use crate::v2::lu_dog_rwlock::types::float_literal::FloatLiteral;
use crate::v2::lu_dog_rwlock::types::format_string::FormatString;
use crate::v2::lu_dog_rwlock::types::integer_literal::IntegerLiteral;
use crate::v2::lu_dog_rwlock::types::string_literal::StringLiteral;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_rwlock::store::ObjectStore as LuDogRwlockStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-enum-documentation"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-hybrid-documentation"}}}
/// A Literal Expression
///
/// This is any literal value in the program.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-enum-definition"}}}
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
    CharLiteral(Uuid),
    FloatLiteral(Uuid),
    FormatString(Uuid),
    IntegerLiteral(Uuid),
    StringLiteral(Uuid),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-implementation"}}}
impl Literal {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-new-impl"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_boolean_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_boolean_literal(
        bogus: bool,
        subtype: &Arc<RwLock<BooleanLiteral>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Literal>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Literal {
            bogus: bogus,
            subtype: LiteralEnum::BooleanLiteral(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_char_literal"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_char_literal(
        bogus: bool,
        subtype: &Arc<RwLock<CharLiteral>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Literal>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Literal {
            bogus: bogus,
            subtype: LiteralEnum::CharLiteral(subtype.read().unwrap().id), // b
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
        subtype: &Arc<RwLock<FloatLiteral>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Literal>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Literal {
            bogus: bogus,
            subtype: LiteralEnum::FloatLiteral(subtype.read().unwrap().id), // b
            id,
        }));
        store.inter_literal(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-struct-impl-new_format_string"}}}
    /// Inter a new Literal in the store, and return it's `id`.
    pub fn new_format_string(
        bogus: bool,
        subtype: &Arc<RwLock<FormatString>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Literal>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Literal {
            bogus: bogus,
            subtype: LiteralEnum::FormatString(subtype.read().unwrap().id), // b
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
        subtype: &Arc<RwLock<IntegerLiteral>>,
        store: &mut LuDogRwlockStore,
    ) -> Arc<RwLock<Literal>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Literal {
            bogus: bogus,
            subtype: LiteralEnum::IntegerLiteral(subtype.read().unwrap().id), // b
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
        subtype: &Arc<RwLock<StringLiteral>>,
        store: &mut LuDogRwlockStore,
        // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
        // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"literal-get-id-impl"}}}
    ) -> Arc<RwLock<Literal>> {
        let id = Uuid::new_v4();
        let new = Arc::new(RwLock::new(Literal {
            bogus: bogus,
            subtype: LiteralEnum::StringLiteral(subtype.read().unwrap().id), // b
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
        store: &'a LuDogRwlockStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store
            .iter_expression()
            .find(|expression| {
                if let ExpressionEnum::Literal(id) = expression.read().unwrap().subtype {
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
