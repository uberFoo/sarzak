// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"item-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-use-statements"}}}
use uuid::{uuid, Uuid};
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-const-documentation"}}}
/// An Item
///
/// This [entails a lot](https://doc.rust-lang.org/reference/items.html) of  syntax that I'm
/// just rolling up into one for now. We'll see for how long I can manage to get away with this
///. 😎
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"item-const-definition"}}}
pub const ITEM: Uuid = uuid!["1e17c1b0-db8f-5bc2-9e01-404271d70afb"];
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
