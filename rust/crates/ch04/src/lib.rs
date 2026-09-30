// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! Chapter 4 of the Rust edition: metalinguistic abstraction.

/// Shared metacircular-evaluator support: environments, procedure values, and name resolution.
pub mod eval_support;
/// Section 4.1: direct evaluation of the checked host subset and the 4.1.7 analyzer.
pub mod sec_4_1;
/// Section 4.2: the named lazy experiments over explicit `Thunk`/`Force` data.
pub mod sec_4_2;
/// Section 4.3: the named search experiment over explicit choice data.
pub mod sec_4_3;
/// Section 4.4: the query system over the explicit data language.
pub mod sec_4_4;
