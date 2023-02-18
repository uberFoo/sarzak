//! Domain for drawing boxen and lines.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"v2::drawing-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::drawing-module-definition"}}}
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

pub use crate::v2::drawing::anchor::Anchor;
pub use crate::v2::drawing::associative_ui::AssociativeUi;
pub use crate::v2::drawing::binary_ui::BinaryUi;
pub use crate::v2::drawing::bottom::BOTTOM;
pub use crate::v2::drawing::edge::Edge;
pub use crate::v2::drawing::isa_ui::IsaUi;
pub use crate::v2::drawing::left::LEFT;
pub use crate::v2::drawing::object_edge::ObjectEdge;
pub use crate::v2::drawing::object_ui::ObjectUi;
pub use crate::v2::drawing::point::Point;
pub use crate::v2::drawing::relationship_ui::RelationshipUi;
pub use crate::v2::drawing::right::RIGHT;
pub use crate::v2::drawing::subtype_anchors::SubtypeAnchors;
pub use crate::v2::drawing::top::TOP;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
