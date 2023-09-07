// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"enum_field-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-use-statements"}}}
use async_std::sync::Arc;
use async_std::sync::RwLock;
use futures::stream::{self, StreamExt};
use tracy_client::span;
use uuid::Uuid;

use crate::v2::lu_dog_async::types::enumeration::Enumeration;
use crate::v2::lu_dog_async::types::expression::Expression;
use crate::v2::lu_dog_async::types::expression::ExpressionEnum;
use crate::v2::lu_dog_async::types::field_access_target::FieldAccessTarget;
use crate::v2::lu_dog_async::types::field_access_target::FieldAccessTargetEnum;
use crate::v2::lu_dog_async::types::plain::Plain;
use crate::v2::lu_dog_async::types::struct_field::StructField;
use crate::v2::lu_dog_async::types::tuple_field::TupleField;
use serde::{Deserialize, Serialize};

use crate::v2::lu_dog_async::store::ObjectStore as LuDogAsyncStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-hybrid-documentation"}}}
/// A field on an Enumeration
///
/// Note that there are three sorts of fields. Tuple, Struct, and “plain?”.
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-hybrid-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnumField {
    pub subtype: EnumFieldEnum,
    pub id: usize,
    pub name: String,
    /// R88: [`EnumField`] 'belongs to an' [`Enumeration`]
    pub woog_enum: usize,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-hybrid-enum-definition"}}}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum EnumFieldEnum {
    Plain(usize),
    StructField(usize),
    TupleField(usize),
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-implementation"}}}
impl EnumField {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_plain"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub async fn new_plain(
        name: String,
        woog_enum: &Arc<RwLock<Enumeration>>,
        subtype: &Arc<RwLock<Plain>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<EnumField>> {
        let s_id = subtype.read().await.id;
        let woog_enum = woog_enum.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_enum_field(|id| {
                Arc::new(RwLock::new(EnumField {
                    name: name.to_owned(),
                    woog_enum,
                    subtype: EnumFieldEnum::Plain(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_struct_field"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub async fn new_struct_field(
        name: String,
        woog_enum: &Arc<RwLock<Enumeration>>,
        subtype: &Arc<RwLock<StructField>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<EnumField>> {
        let s_id = subtype.read().await.id;
        let woog_enum = woog_enum.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_enum_field(|id| {
                Arc::new(RwLock::new(EnumField {
                    name: name.to_owned(),
                    woog_enum,
                    subtype: EnumFieldEnum::StructField(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-new_tuple_field"}}}
    /// Inter a new EnumField in the store, and return it's `id`.
    pub async fn new_tuple_field(
        name: String,
        woog_enum: &Arc<RwLock<Enumeration>>,
        subtype: &Arc<RwLock<TupleField>>,
        store: &mut LuDogAsyncStore,
    ) -> Arc<RwLock<EnumField>> {
        let s_id = subtype.read().await.id;
        let woog_enum = woog_enum.read().await.id;
        let subtype = subtype.read().await.id;
        store
            .inter_enum_field(|id| {
                Arc::new(RwLock::new(EnumField {
                    name: name.to_owned(),
                    woog_enum,
                    subtype: EnumFieldEnum::TupleField(subtype),
                    id,
                }))
            })
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-struct-impl-nav-forward-to-woog_enum"}}}
    /// Navigate to [`Enumeration`] across R88(1-*)
    pub async fn r88_enumeration<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Enumeration>>> {
        span!("r88_enumeration");
        vec![store.exhume_enumeration(&self.woog_enum).await.unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-impl-nav-subtype-to-supertype-expression"}}}
    // Navigate to [`Expression`] across R15(isa)
    pub async fn r15_expression<'a>(
        &'a self,
        store: &'a LuDogAsyncStore,
    ) -> Vec<Arc<RwLock<Expression>>> {
        span!("r15_expression");
        store
            .iter_expression()
            .await
            .filter_map(|expression| async move {
                if let ExpressionEnum::EnumField(id) = expression.read().await.subtype {
                    Some(expression.clone())
                } else {
                    None
                }
            })
            .collect()
            .await
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-impl-nav-subtype-to-supertype-field_access_target"}}}
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
                if let FieldAccessTargetEnum::EnumField(id) =
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
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"enum_field-implementation"}}}
impl PartialEq for EnumField {
    fn eq(&self, other: &Self) -> bool {
        self.subtype == other.subtype
            && self.name == other.name
            && self.woog_enum == other.woog_enum
    }
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
