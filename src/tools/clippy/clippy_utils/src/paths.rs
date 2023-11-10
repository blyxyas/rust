//! This module contains paths to types and functions Clippy needs to know
//! about.
//!
//! Whenever possible, please consider diagnostic items over hardcoded paths.
//! See <https://github.com/rust-lang/rust-clippy/issues/5393> for more information.

#[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
pub const FUTURES_IO_ASYNCREADEXT: [&str; 3] = ["futures_util", "io", "AsyncReadExt"];
#[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
pub const FUTURES_IO_ASYNCWRITEEXT: [&str; 3] = ["futures_util", "io", "AsyncWriteExt"];
pub const ITERTOOLS_NEXT_TUPLE: [&str; 3] = ["itertools", "Itertools", "next_tuple"];
pub const PARKING_LOT_MUTEX_GUARD: [&str; 3] = ["lock_api", "mutex", "MutexGuard"];
pub const PARKING_LOT_RWLOCK_READ_GUARD: [&str; 3] = ["lock_api", "rwlock", "RwLockReadGuard"];
pub const PARKING_LOT_RWLOCK_WRITE_GUARD: [&str; 3] = ["lock_api", "rwlock", "RwLockWriteGuard"];
pub const REGEX_BUILDER_NEW: [&str; 3] = ["regex", "RegexBuilder", "new"];
pub const REGEX_BYTES_BUILDER_NEW: [&str; 4] = ["regex", "bytes", "RegexBuilder", "new"];
pub const REGEX_BYTES_NEW: [&str; 4] = ["regex", "bytes", "Regex", "new"];
pub const REGEX_BYTES_SET_NEW: [&str; 4] = ["regex", "bytes", "RegexSet", "new"];
pub const REGEX_NEW: [&str; 3] = ["regex", "Regex", "new"];
pub const REGEX_SET_NEW: [&str; 3] = ["regex", "RegexSet", "new"];
pub const SERDE_DESERIALIZE: [&str; 3] = ["serde", "de", "Deserialize"];
pub const SERDE_DE_VISITOR: [&str; 3] = ["serde", "de", "Visitor"];
#[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
pub const TOKIO_IO_ASYNCREADEXT: [&str; 5] = ["tokio", "io", "util", "async_read_ext", "AsyncReadExt"];
#[expect(clippy::invalid_paths)] // internal lints do not know about all external crates
pub const TOKIO_IO_ASYNCWRITEEXT: [&str; 5] = ["tokio", "io", "util", "async_write_ext", "AsyncWriteExt"];
