// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

//! Feature-based re-export of common collection components.
//!
//! When `std` feature is enabled, this module exports collections from the Rust standard library.
//! When `alloc` feature is enabled, same collected are provided without relying on the Rust
//! standard library.

#[cfg(feature = "alloc")]
pub use alloc::collections::{BTreeMap, BTreeSet};

#[cfg(feature = "alloc")]
pub use alloc::vec::{self as vec, Vec};

#[cfg(feature = "alloc")]
pub use hashbrown::HashMap;

#[cfg(feature = "std")]
pub use std::collections::{BTreeMap, BTreeSet, HashMap};

#[cfg(feature = "std")]
pub use std::vec::{self as vec, Vec};
