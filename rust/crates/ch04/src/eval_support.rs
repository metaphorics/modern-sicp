// SPDX-License-Identifier: GPL-3.0-only

//! Shared helpers for the chapter 4 runs and tests: the check-then-run
//! compositions over the direct and analyzed engines, and the
//! transcript comparisons the cross-engine agreement cases hold.

use sicp_runtime::host::diag::Diag;

pub use crate::sec_4_1::{
    Direct, Plan, admit, analyze, run, run_analyzed, run_program, run_source,
};
pub use sicp_runtime::host::check::CheckedProgram;
pub use sicp_runtime::host::ops::{Flow, RunOutcome, TrapReport};
pub use sicp_runtime::host::value::{HostValue, Trap};
pub use sicp_runtime::host::{hir, ops, value};

/// Runs one source text on both direct and analyzed engines and
/// answers their ordered transcripts.
///
/// # Errors
/// The admission [`Diag`] when the source is rejected before any
/// effect, as grammar §1 requires.
pub fn both_transcripts(source: &str) -> Result<(String, String), Diag> {
    let program = admit(source)?;
    let direct = run(&program);
    let analyzed = run_analyzed(&program);
    Ok((direct.stdout, analyzed.stdout))
}

/// The one-line trap rendering the corpus's error cases observe.
#[must_use]
pub fn trap_line(outcome: &RunOutcome) -> Option<String> {
    outcome
        .trap
        .as_ref()
        .map(|report| format!("trap: {:?}", report.trap))
}
