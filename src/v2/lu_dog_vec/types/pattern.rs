// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"pattern-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-use-statements"}}}
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use crate::v2::lu_dog_vec::types::expression::Expression;
use crate::v2::lu_dog_vec::types::x_match::XMatch;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_vec::store::ObjectStore as LuDogVecStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-documentation"}}}
/// The pattern is a specification for extracting data from a type. It’s sort of a reverse
///  impression of what you are looking for. If the shape of the impression matches the scrutinee
/// , then they “fit” and the pattern’s lvalues will be populated with data from the scrutinee
/// .
///
/// There are a bunch of diffirent kinds of patterns. Literal, ident, struct, tuple, etc. Modeling
///  this will take a lot of room and time.
///
/// Doing this I’m going to cheat a bit and store the code that does matching as a string
///  on this object during compilation. During runtime the string will be evaluated (either as
///  dwrf, or perhasps using a small VM. Or maybe use the built-in VM. It should be able to handle
///  all that we need. This way, I don’t have to model all the bits because they are encoded
///  in the code attribute.
///
/// So I guess that means I’ll be writing assembly code...
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pattern {
    pub id: usize,
    /// R92: [`Pattern`] 'executes' [`Expression`]
    pub expression: usize,
    /// R87: [`Expression`] '🚧 Comments are out of order — see sarzak#14.' [`Expression`]
    pub match_expr: usize,
    /// R87: [`XMatch`] '🚧 Comments are out of order — see sarzak#14.' [`XMatch`]
    pub x_match: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-implementation"}}}
impl Pattern {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-impl-new"}}}
    /// Inter a new 'Pattern' in the store, and return it's `id`.
    pub fn new(
        expression: &Rc<RefCell<Expression>>,
        match_expr: &Rc<RefCell<Expression>>,
        x_match: &Rc<RefCell<XMatch>>,
        store: &mut LuDogVecStore,
    ) -> Rc<RefCell<Pattern>> {
        store.inter_pattern(|id| {
            Rc::new(RefCell::new(Pattern {
                id,
                expression: expression.borrow().id,
                match_expr: match_expr.borrow().id,
                x_match: x_match.borrow().id,
            }))
        })
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-impl-nav-backward-1_M-to-x_match"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-impl-nav-forward-assoc-to-x_match"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-impl-nav-forward-assoc-to-pattern"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R92(1-*)
    pub fn r92_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store.exhume_expression(&self.expression).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-impl-nav-forward-assoc-to-match_expr"}}}
    /// Navigate to [`Expression`] across R87(1-*)
    pub fn r87_expression<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<Expression>>> {
        vec![store.exhume_expression(&self.match_expr).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-impl-nav-forward-assoc-to-pattern"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-impl-nav-forward-assoc-to-x_match"}}}
    /// Navigate to [`XMatch`] across R87(1-*)
    pub fn r87_x_match<'a>(&'a self, store: &'a LuDogVecStore) -> Vec<Rc<RefCell<XMatch>>> {
        vec![store.exhume_x_match(&self.x_match).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-implementation"}}}
impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.expression == other.expression
            && self.match_expr == other.match_expr
            && self.x_match == other.x_match
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
