#![feature(rustc_private)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    rust_2018_idioms,
    unused_lifetimes,
    unused_qualifications
)]
#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    rustc::diagnostic_outside_of_impl,
    rustc::untranslatable_diagnostic
)]
#![deny(clippy::derive_deserialize_allowing_unknown)]

// use rustc_data_structures;
// use rustc_errors;
// use rustc_hir;
// use rustc_middle;
// use rustc_session;
// use rustc_span;

mod conf;
mod metadata;
pub mod types;

pub use conf::{Conf, get_configuration_metadata, lookup_conf_file, sanitize_explanation};
pub use metadata::ClippyConfiguration;
