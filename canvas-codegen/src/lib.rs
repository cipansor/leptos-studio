//! Turn a canvas component tree into source code for eleven targets.
//!
//! A [`CanvasComponent`] tree describes a UI the way a visual builder holds it:
//! a list of components, each with a style, an optional animation and nested
//! children. This crate walks that tree and emits source for a target framework
//! or format. It needs no browser and no wasm, so it runs in a CLI, a build
//! script or a server as readily as in a wasm front end.
//!
//! The generators share one trait, [`CodeGenerator`], so a caller can choose a
//! target at runtime. A generator writes no product name into its output unless
//! asked to through [`GeneratorConfig`].
//!
//! [`CanvasComponent`]: domain::CanvasComponent
#![doc = include_str!("../README.md")]

pub mod config;
pub mod domain;
pub mod generator;
pub mod generators;
pub mod preset;
pub mod utils;

pub use config::GeneratorConfig;
pub use generator::CodeGenerator;
pub use preset::ExportPreset;
