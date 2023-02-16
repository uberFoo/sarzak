//! Domain for drawing boxen and lines.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"drawing-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"drawing-module-definition"}}}
pub mod anchor;
pub mod associative;
pub mod associative_ui;
pub mod binary;
pub mod binary_ui;
pub mod bottom;
pub mod edge;
pub mod isa;
pub mod isa_ui;
pub mod left;
pub mod object;
pub mod object_edge;
pub mod object_ui;
pub mod point;
pub mod relationship_ui;
pub mod right;
pub mod subtype_anchors;
pub mod top;

pub use crate::drawing::anchor::Anchor;
pub use crate::drawing::associative::ASSOCIATIVE;
pub use crate::drawing::associative_ui::AssociativeUi;
pub use crate::drawing::binary::Binary;
pub use crate::drawing::binary_ui::BinaryUi;
pub use crate::drawing::bottom::BOTTOM;
pub use crate::drawing::edge::Edge;
pub use crate::drawing::isa::Isa;
pub use crate::drawing::isa_ui::IsaUi;
pub use crate::drawing::left::LEFT;
pub use crate::drawing::object::Object;
pub use crate::drawing::object_edge::ObjectEdge;
pub use crate::drawing::object_ui::ObjectUi;
pub use crate::drawing::point::Point;
pub use crate::drawing::relationship_ui::RelationshipUi;
pub use crate::drawing::right::RIGHT;
pub use crate::drawing::subtype_anchors::SubtypeAnchors;
pub use crate::drawing::top::TOP;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
