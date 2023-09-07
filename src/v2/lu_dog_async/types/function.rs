// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"function-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::body::Body;
use crate::v2::lu_dog_async::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_async::types::field_access_target::FieldAccessTargetEnum;
use crate::v2::lu_dog_async::types::implementation_block::ImplementationBlock;
use crate::v2::lu_dog_async::types::item::Item;
use crate::v2::lu_dog_async::types::item::ItemEnum;
use crate::v2::lu_dog_async::types::parameter::Parameter;
use crate::v2::lu_dog_async::types::value_type::ValueType;
use crate::v2::lu_dog_async::types::value_type::ValueTypeEnum;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-documentation"}}}
/// A Function
///
/// Inputs, outputs. Stuff happens.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Function {
    pub id: usize,
    pub name: String,
    /// R19: [`Function`] 'executes statements in a' [`Body`]
    pub body: usize,
    /// R82: [`Function`] 'may have a first parameter' [`Parameter`]
    pub first_param: Option<usize>,
    /// R9: [`Function`] 'may be contained in an' [`ImplementationBlock`]
    pub impl_block: Option<usize>,
    /// R10: [`Function`] 'returns' [`ValueType`]
    pub return_type: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-implementation"}}}
impl Function {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-new"}}}
    /// Inter a new 'Function' in the store, and return it's `id`.
    pub async fn new(
        name: String,
        body: &Arc<RwLock<Body>>,
        first_param: Option<&Arc<RwLock<Parameter>>>,
        impl_block: Option<&Arc<RwLock<ImplementationBlock>>>,
        return_type: &Arc<RwLock<ValueType>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<Function>> {
        let parameter = match first_param {
            Some(parameter) => Some(parameter.read().await.id),
            None => None,
        };
        let return_type = return_type.read().await.id;
        let implementation_block = match impl_block {
            Some(implementation_block) => Some(implementation_block.read().await.id),
            None => None,
        };
        let body = body.read().await.id;
        store
            .inter_function(|id| {
                Arc::new(RwLock::new(Function {
                    id,
                    name: name.to_owned(),
                    body,
                    first_param: parameter,
                    impl_block: implementation_block,
                    return_type,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-body"}}}
    /// Navigate to [`Body`] across R19(1-*)
    pub async fn r19_body<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Body>>> {
        span!("r19_body");
        vec![store.exhume_body(&self.body).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-first_param"}}}
    /// Navigate to [`Parameter`] across R82(1-*c)
    pub async fn r82_parameter<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Parameter>>> {
        span!("r82_parameter");
        match self.first_param {
            Some(ref first_param) => vec![store.exhume_parameter(first_param).await.unwrap()],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-cond-to-impl_block"}}}
    /// Navigate to [`ImplementationBlock`] across R9(1-*c)
    pub async fn r9_implementation_block<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ImplementationBlock>>> {
        span!("r9_implementation_block");
        match self.impl_block {
            Some(ref impl_block) => {
                vec![store.exhume_implementation_block(impl_block).await.unwrap()]
            }
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-forward-to-return_type"}}}
    /// Navigate to [`ValueType`] across R10(1-*)
    pub async fn r10_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r10_value_type");
        vec![store.exhume_value_type(&self.return_type).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-struct-impl-nav-backward-1_M-to-parameter"}}}
    /// Navigate to [`Parameter`] across R13(1-M)
    pub async fn r13_parameter<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Parameter>>> {
        span!("r13_parameter");
        store
            .iter_parameter()
            .await
            .filter_map(|parameter| async {
                if parameter.read().await.function == self.id {
                    Some(parameter)
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-impl-nav-subtype-to-supertype-field_access_target"}}}
    // Navigate to [`FieldAccessTarget`] across R67(isa)
    pub async fn r67_field_access_target<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<FieldAccessTarget>>> {
        span!("r67_field_access_target");
        store
            .iter_field_access_target()
            .await
            .filter_map(|field_access_target| async move {
                if let FieldAccessTargetEnum::Function(id) =
                    field_access_target.read().await.subtype
                {
                    Some(field_access_target.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-impl-nav-subtype-to-supertype-item"}}}
    // Navigate to [`Item`] across R6(isa)
    pub async fn r6_item<'a>(&'a self, store: &'a LuDogAsyncStore) -> Vec<Arc<RwLock<Item>>> {
        span!("r6_item");
        store
            .iter_item()
            .await
            .filter_map(|item| async move {
                if let ItemEnum::Function(id) = item.read().await.subtype {
                    Some(item.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-impl-nav-subtype-to-supertype-value_type"}}}
    // Navigate to [`ValueType`] across R1(isa)
    pub async fn r1_value_type<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<ValueType>>> {
        span!("r1_value_type");
        store
            .iter_value_type()
            .await
            .filter_map(|value_type| async move {
                if let ValueTypeEnum::Function(id) = value_type.read().await.subtype {
                    Some(value_type.clone())
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
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"function-implementation"}}}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.body == other.body
            && self.first_param == other.first_param
            && self.impl_block == other.impl_block
            && self.return_type == other.return_type
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
