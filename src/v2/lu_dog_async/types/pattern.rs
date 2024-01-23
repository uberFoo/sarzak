// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"pattern-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::x_match::XMatch;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
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
    pub async fn new(
        expression: &Arc<RwLock<Expression>>,
        match_expr: &Arc<RwLock<Expression>>,
        x_match: &Arc<RwLock<XMatch>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Pattern>> {
        let expression = expression.read().await.id;
        let x_match = x_match.read().await.id;
        let match_expr = match_expr.read().await.id;
        store
            .inter_pattern(|id| {
                Arc::new(RwLock::new(Pattern {
                    id,
                    expression,
                    match_expr,
                    x_match,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-impl-nav-forward-to-expression"}}}
    /// Navigate to [`Expression`] across R92(1-*)
    pub async fn r92_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> impl futures::Stream<Item = Arc<RwLock<Expression>>> + '_ {
        stream::iter(vec![store.exhume_expression(&self.expression).await.unwrap()].into_iter())
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-impl-nav-forward-assoc-to-match_expr"}}}
    /// Navigate to [`Expression`] across R87(1-*)
    pub async fn r87_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        vec![store.exhume_expression(&self.match_expr).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"pattern-struct-impl-nav-forward-assoc-to-x_match"}}}
    /// Navigate to [`XMatch`] across R87(1-*)
    pub async fn r87_x_match<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<XMatch>>> {
        vec![store.exhume_x_match(&self.x_match).await.unwrap()]
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
