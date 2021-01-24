//! # lorri
//! lorri is a wrapper over Nix to abstract project-specific build
//! configuration and patterns in to a declarative configuration.

#![warn(missing_docs)]

#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;

pub mod bash;
pub mod build_loop;
pub mod builder;
pub mod cas;
pub mod changelog;
pub mod cli;
pub mod constants;
pub mod daemon;
pub mod error;
pub mod locate_file;
pub mod logging;
pub mod nix;
pub mod ops;
pub mod osstrlines;
pub mod pathreduction;
pub mod project;
pub mod socket;
pub mod thread;
pub mod watch;

// This and the following module declaration together publicly export the
// contents of the generated module "org_nixos_lorri" as "proto", which is a
// much nicer module name.
#[allow(missing_docs, clippy::all, unused_imports)]
mod org_nixos_lorri;

#[allow(missing_docs)]
pub mod proto {
    // Code generated from org.nixos.lorri.varlink
    pub use super::org_nixos_lorri::*;
}

// This and the following module declaration together publicly export the
// contents of the generated module "org_nixos_lorri_internal" as
// "internal_proto", which is a much nicer module name.
#[allow(missing_docs, clippy::all, unused_imports)]
mod org_nixos_lorri_internal;

#[allow(missing_docs)]
pub(crate) mod internal_proto {
    // Code generated from org.nixos.lorri.internal.varlink
    pub use super::org_nixos_lorri_internal::*;
}

use std::path::{Path, PathBuf};

// OUT_DIR and build_rev.rs are generated by cargo, see ../build.rs
include!(concat!(env!("OUT_DIR"), "/build_rev.rs"));

/// A .nix file.
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct NixFile(PathBuf);

impl NixFile {
    /// Underlying `Path`.
    pub fn as_path(&self) -> &Path {
        &self.0
    }

    /// Display the underlying path
    pub fn display(&self) -> std::path::Display {
        self.0.display()
    }
}

impl From<PathBuf> for NixFile {
    fn from(p: PathBuf) -> NixFile {
        NixFile(p)
    }
}

impl From<String> for NixFile {
    fn from(s: String) -> Self {
        NixFile(PathBuf::from(s))
    }
}

impl slog::Value for NixFile {
    fn serialize(
        &self,
        _record: &slog::Record,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        serializer.emit_arguments(key, &format_args!("{}", self.as_path().display()))
    }
}

/// A .drv file (generated by `nix-instantiate`).
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct DrvFile(PathBuf);

impl DrvFile {
    /// Underlying `Path`.
    pub fn as_path(&self) -> &Path {
        self.0.as_ref()
    }
}

impl From<PathBuf> for DrvFile {
    fn from(p: PathBuf) -> DrvFile {
        DrvFile(p)
    }
}
