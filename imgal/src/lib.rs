#![doc(
    html_logo_url = "https://raw.githubusercontent.com/imgal-sc/imgal/refs/heads/main/docs/assets/imgal_logo.svg"
)]
//! The `imgal` (**Im**a**g**e **A**lgorithm **L**ibrary) crate is a fast and
//! open-source scientific image processing and algorithm library.This library
//! is directly inspired by [imagej-ops](https://github.com/imagej/imagej-ops/),
//! [SciJava Ops](https://github.com/scijava/scijava),
//! [ImgLib2](https://github.com/imglib/imglib2), the ImageJ2 ecosystem.
//! `imgal` library aims to offer users access to fast and well documented
//! image algorithms.
//!
//! ## Crate Status
//!
//! This crate is being iterated on and is constantly evolving.
pub mod colocalization;
pub mod distribution;
pub mod error;
pub mod filter;
pub mod image;
pub mod integration;
pub mod kernel;
pub mod parameter;
pub mod phasor;
pub mod simulation;
pub mod statistics;
pub mod threshold;
pub mod traits;
