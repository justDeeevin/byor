#[cfg(feature = "lock")]
pub mod lock;

#[cfg(feature = "channel")]
pub mod channel;

#[cfg(feature = "exec")]
pub mod executor;

#[cfg(feature = "fs")]
pub mod fs;

macro_rules! any_feature {
  ($($item:tt)*) => {
    #[cfg(any(feature = "lock", feature = "channel", feature = "exec", feature = "fs"))]
    $($item)*
  }
}

any_feature! { pub mod runtime; }

mod implement;
