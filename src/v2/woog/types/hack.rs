// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"hack-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hack-use-statements"}}}
use uuid::Uuid;

use crate::v2::woog::types::literal::Literal;
use serde::{Deserialize, Serialize};

use crate::v2::woog::store::ObjectStore as WoogStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hack-struct-documentation"}}}
/// This is a super hack. A super great one!
///
/// I'm gonig to use this to cheat while I get expressions working. All I have to do is stuff
/// a string value in here and puke it out, without having to build and parse the expression
///.
///
/// In the mentime, I'll be working towards building and parsing the AST for expressions, and
/// a whole lot more!
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hack-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Hack {
    pub id: Uuid,
    pub value: String,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hack-implementation"}}}
impl Hack {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hack-struct-impl-new"}}}
    /// Inter a new 'Hack' in the store, and return it's `id`.
    pub fn new(value: String, store: &mut WoogStore) -> Hack {
        let id = Uuid::new_v4();
        let new = Hack {
            id: id,
            value: value,
        };
        store.inter_hack(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hack-struct-impl-new_"}}}
    /// Inter a new 'Hack' in the store, and return it's `id`.
    pub fn new_(value: String) -> Hack {
        let id = Uuid::new_v4();
        let new = Hack {
            id: id,
            value: value,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"hack-impl-nav-subtype-to-supertype-literal"}}}
    // Navigate to [`Literal`] across R32(isa)
    pub fn r32_literal<'a>(&'a self, store: &'a WoogStore) -> Vec<&Literal> {
        vec![store.exhume_literal(&self.id).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
