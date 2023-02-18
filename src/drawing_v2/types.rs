//! Domain for drawing boxen and lines.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"drawing_v2-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"drawing_v2-module-definition"}}}
pub mod anchor;
pub mod associative_ui;
pub mod binary_ui;
pub mod bottom;
pub mod edge;
pub mod isa_ui;
pub mod left;
pub mod object_edge;
pub mod object_ui;
pub mod point;
pub mod relationship_ui;
pub mod right;
pub mod subtype_anchors;
pub mod top;

pub use crate::drawing_v2::anchor::Anchor;
pub use crate::drawing_v2::associative_ui::AssociativeUi;
pub use crate::drawing_v2::binary_ui::BinaryUi;
pub use crate::drawing_v2::bottom::BOTTOM;
pub use crate::drawing_v2::edge::Edge;
pub use crate::drawing_v2::isa_ui::IsaUi;
pub use crate::drawing_v2::left::LEFT;
pub use crate::drawing_v2::object_edge::ObjectEdge;
pub use crate::drawing_v2::object_ui::ObjectUi;
pub use crate::drawing_v2::point::Point;
pub use crate::drawing_v2::relationship_ui::RelationshipUi;
pub use crate::drawing_v2::right::RIGHT;
pub use crate::drawing_v2::subtype_anchors::SubtypeAnchors;
pub use crate::drawing_v2::top::TOP;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
