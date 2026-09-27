// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Section 0.9: Modules, crates, and reading this book.

/// The name Cargo gives this crate, baked in at compile time from the
/// manifest; running `cargo run --example sec_0_9_workspace` prints it,
/// the way this section's listing runs from the workspace it describes.
#[must_use]
pub fn crate_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}
