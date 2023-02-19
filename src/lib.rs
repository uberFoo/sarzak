pub mod domain;
pub mod error;
pub mod mc;
pub mod v1;
pub mod v2;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use v1::drawing;
pub use v1::sarzak;
pub use v2::woog;
