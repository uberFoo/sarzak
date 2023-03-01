pub mod domain;
pub mod error;
pub mod mc;
pub mod v1;
pub mod v2;
// pub mod woog_2;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use v2::drawing;
pub use v2::sarzak;
pub use v2::woog;
