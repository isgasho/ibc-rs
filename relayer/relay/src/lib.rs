#![forbid(unsafe_code)]
#![deny(
    warnings,
    // missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    rust_2018_idioms
)]

//! IBC Relayer implementation

pub mod config;
pub mod error;
