pub mod domain;
pub mod drawing_v1;
pub mod error;
pub mod mc;
pub mod sarzak_v1;
pub mod woog_v1;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use drawing_v1 as drawing;
pub use sarzak_v1 as sarzak;
pub use woog_v1 as woog;
