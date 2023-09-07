// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"tuple_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::enum_field::EnumField;
use crate::v2::lu_dog_async::types::enum_field::EnumFieldEnum;
use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-struct-documentation"}}}
/// A field that is a tuple.
///
/// Currently in this implementation we are cheating, as we don’t yet actually have tuples
/// . So this is limited to a single item.
///
/// Note the `hack` attribute. What’s happening is that during generic substitution?, expansion
/// ?, whatever. During that we are cloning the enum, and it’s fields. This is to create a
///  new type. When we do this we don’t want the store optimizing away a duplicate Tuple Field
/// .
///
/// I deb thee hack because I think the right thing to do is something else, I’m just not
///  sure what it is yet.
///
/// I renamed it to `xyzzy`, because I think `hack` does magic in the compiler.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TupleField {
    pub id: usize,
    pub xyzzy: Uuid,
    /// R90: [`TupleField`] 'is constructed via' [`Expression`]
    pub expression: Option<usize>,
    /// R86: [`TupleField`] 'must have a type' [`ValueType`]
    pub ty: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-implementation"}}}
impl TupleField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-struct-impl-new"}}}
    /// Inter a new 'Tuple Field' in the store, and return it's `id`.
    pub async fn new(
        xyzzy: Uuid,
        expression: Option<&Arc<RwLock<Expression>>>,
        ty: &Arc<RwLock<ValueType>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<TupleField>> {
        let expression = match expression {
            Some(expression) => Some(expression.read().await.id),
            None => None,
        };
        let ty = ty.read().await.id;
        store
            .inter_tuple_field(|id| {
                Arc::new(RwLock::new(TupleField {
                    id,
                    xyzzy,
                    expression,
                    ty,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-struct-impl-nav-forward-cond-to-expression"}}}
    /// Navigate to [`Expression`] across R90(1-*c)
    pub async fn r90_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r90_expression");
        match self.expression {
            Some(ref expression) => vec![store.exhume_expression(expression).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-struct-impl-nav-forward-to-ty"}}}
    /// Navigate to [`ValueType`] across R86(1-*)
    pub async fn r86_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r86_value_type");
        vec![store.exhume_value_type(&self.ty).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-impl-nav-subtype-to-supertype-enum_field"}}}
    // Navigate to [`EnumField`] across R85(isa)
    pub async fn r85_enum_field<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<EnumField>>> {
        span!("r85_enum_field");
        store
            .iter_enum_field()
            .await
            .filter_map(|enum_field| async move {
                if let EnumFieldEnum::TupleField(id) = enum_field.read().await.subtype {
                    Some(enum_field.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"tuple_field-implementation"}}}
impl PartialEq for TupleField {
    fn eq(&self, other: &Self) -> bool {
        self.xyzzy == other.xyzzy && self.expression == other.expression && self.ty == other.ty
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
