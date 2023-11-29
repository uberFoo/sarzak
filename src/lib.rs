#![allow(warnings)]

pub mod domain;
pub mod error;
pub mod mc;
pub mod v1;
pub mod v2;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use v2::drawing;
pub use v2::merlin;

cfg_if::cfg_if! {
    if #[cfg(feature = "sarzak-rwlock")] {
        pub use v2::sarzak;
    } else if #[cfg(feature = "sarzak-single")] {
        pub use v2::sarzak_single as sarzak;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "woog-rwlock")] {
        pub use v2::woog;
    } else if #[cfg(feature = "woog-single")] {
        pub use v2::woog_single as woog;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "lu-dog-rc")] {
        pub use v2::lu_dog;
    } else if #[cfg(feature = "lu-dog-vec")] {
        pub use v2::lu_dog_vec as lu_dog;
    } else if #[cfg(feature = "lu-dog-vec-tracy")] {
        pub use v2::lu_dog_vec_tracy as lu_dog;
    } else if #[cfg(feature = "lu-dog-async-vec")] {
        pub use v2::lu_dog_async as lu_dog;
    } else if #[cfg(feature = "lu-dog-rwlock-vec")] {
        pub use v2::lu_dog_rwlock_vec as lu_dog;
    } else if #[cfg(feature = "lu-dog-rwlock")] {
        pub use v2::lu_dog_rwlock as lu_dog;
    } else if #[cfg(feature = "lu-dog-ndrwlock-vec")] {
        pub use v2::lu_dog_ndrwlock_vec as lu_dog;
    }
}
