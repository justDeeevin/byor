#[cfg(feature = "lock")]
mod lock;

#[cfg(feature = "channel")]
mod channel;

#[cfg(any(feature = "exec", feature = "local-exec"))]
mod executor;

#[cfg(feature = "fs")]
mod fs;

#[cfg(all(
    feature = "fd",
    any(
        unix,
        target_os = "hermit",
        target_os = "trusty",
        target_os = "wasi",
        target_os = "motor"
    )
))]
mod fd;

#[cfg(feature = "time")]
mod time;

#[cfg(feature = "net")]
mod net;

#[cfg(feature = "process")]
mod process;
