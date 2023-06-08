pub mod domain;
pub mod error;
pub mod mc;
pub mod v1;
pub mod v2;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use v2::drawing;
pub use v2::merlin;
pub use v2::sarzak;
pub use v2::woog;

cfg_if::cfg_if! {
    if #[cfg(feature = "lu-dog-rc")] {
        pub use v2::lu_dog;
    } else if #[cfg(feature = "lu-dog-rwlock")] {
        pub use v2::lu_dog_rwlock as lu_dog;
    } else if #[cfg(feature = "lu-dog-pl-mutex")] {
        pub use v2::lu_dog_pl_mutex as lu_dog;
    } else if #[cfg(feature = "lu-dog-vanilla")] {
        pub use v2::lu_dog_vanilla as lu_dog;
    }
}
