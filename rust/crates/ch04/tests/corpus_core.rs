// SPDX-License-Identifier: GPL-3.0-only

//! The core conformance corpus: every `core` row of
//! `spec/host-subsets/rust/manifest.tsv` runs through the section 4.1
//! evaluator, and the rendered stdout compares byte for byte against
//! the pinned native toolchain's run of the same source (the
//! interpreted-versus-native gate of the edition contract, §10).
//! A row whose teaching run traps must trap exactly where the native
//! run fails: stdout bytes and failure agree.

use std::path::{Path, PathBuf};
use std::process::Command;

use ch04::sec_4_1::run_source;

const MANIFEST: &str = "spec/host-subsets/rust/manifest.tsv";
const PROGRAMS: &str = "spec/host-subsets/rust";

/// The evaluator thread's stack: deep guest recursion (core/19)
/// overflows a default test-thread stack, as it would the main
/// thread's; the reservation is virtual.
const EVALUATOR_STACK_BYTES: usize = 1 << 30;

fn repo_root() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    for ancestor in manifest.ancestors() {
        if ancestor.join(MANIFEST).is_file() {
            return ancestor.to_path_buf();
        }
    }
    panic!("{MANIFEST} not found above {}", manifest.display());
}

fn core_artifacts(root: &Path) -> Vec<String> {
    let text = std::fs::read_to_string(root.join(MANIFEST)).expect("manifest is readable");
    let mut rows = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let mut cells = line.split('\t');
        let (Some(id), Some(artifact)) = (cells.next(), cells.next()) else {
            panic!("malformed manifest row: {line}");
        };
        if id.starts_with("core/") {
            rows.push(artifact.to_owned());
        }
    }
    rows
}

fn native_stdout(root: &Path, artifact: &str, index: usize) -> (Vec<u8>, bool) {
    let source = root.join(PROGRAMS).join(artifact);
    let binary =
        std::env::temp_dir().join(format!("modern-sicp-corpus-{}-{index}", std::process::id()));
    let compile = Command::new("rustc")
        .args(["--edition", "2024", "-C", "overflow-checks=on", "-o"])
        .arg(&binary)
        .arg(&source)
        .output()
        .expect("the pinned toolchain compiles the corpus");
    assert!(
        compile.status.success(),
        "native compile of {artifact}: {}",
        String::from_utf8_lossy(&compile.stderr)
    );
    let run = Command::new(&binary)
        .output()
        .expect("the native corpus binary runs");
    (run.stdout, !run.status.success())
}

#[test]
fn corpus_core_matches_expected_output() {
    let root = repo_root();
    let rows = core_artifacts(&root);
    assert_eq!(rows.len(), 21, "the manifest lists 21 core rows");
    for (index, artifact) in rows.iter().enumerate() {
        let source = std::fs::read_to_string(root.join(PROGRAMS).join(artifact))
            .unwrap_or_else(|error| panic!("{artifact} is unreadable: {error}"));
        let teaching = std::thread::Builder::new()
            .name("corpus-evaluator".to_owned())
            .stack_size(EVALUATOR_STACK_BYTES)
            .spawn(move || run_source(&source))
            .expect("the evaluator thread starts")
            .join()
            .expect("the evaluator thread runs");
        let (native_out, native_failed) = native_stdout(&root, artifact, index);
        match teaching {
            Ok(outcome) => {
                // A trapping row (core/20) must trap exactly where the
                // native run fails: same stdout bytes, same failure.
                assert_eq!(
                    outcome.trap.is_some(),
                    native_failed,
                    "program {artifact} disagrees with native on failure"
                );
                assert_eq!(
                    outcome.stdout.as_bytes(),
                    native_out.as_slice(),
                    "program {artifact}"
                );
            }
            Err(diag) => panic!("program {artifact} is rejected: {diag:?}"),
        }
    }
}
