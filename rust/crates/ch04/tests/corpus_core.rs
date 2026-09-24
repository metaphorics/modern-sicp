// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The core conformance corpus: every `core` row of
//! `spec/scheme-subset/manifest.txt` whose evaluators include this
//! edition runs through the section 4.1 evaluator, and the rendered
//! output compares byte for byte against `expected/core/*.txt` from
//! line 3 on, the two license header lines being common.

use std::path::{Path, PathBuf};

use ch04::sec_4_1::run_program;

struct Row {
    capability: String,
    program: String,
    expected: String,
    mode: String,
    evaluators: String,
}

fn repo_root() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    for ancestor in manifest.ancestors() {
        if ancestor.join("spec/scheme-subset/manifest.txt").is_file() {
            return ancestor.to_path_buf();
        }
    }
    panic!("spec/scheme-subset not found above {}", manifest.display());
}

fn manifest_rows() -> Vec<Row> {
    let text = std::fs::read_to_string(repo_root().join("spec/scheme-subset/manifest.txt"))
        .expect("manifest is readable");
    let mut rows = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') || line.starts_with(';') || line.trim().is_empty() {
            continue;
        }
        let cells: Vec<&str> = line.split('\t').collect();
        let [capability, program, expected, mode, evaluators] = cells[..] else {
            panic!("malformed manifest row: {line}");
        };
        rows.push(Row {
            capability: capability.to_owned(),
            program: program.to_owned(),
            expected: expected.to_owned(),
            mode: mode.to_owned(),
            evaluators: evaluators.to_owned(),
        });
    }
    rows
}

fn core_rows_for_this_edition() -> Vec<Row> {
    manifest_rows()
        .into_iter()
        .filter(|row| row.capability == "core")
        .filter(|row| {
            row.evaluators == "all" || row.evaluators.split(' ').any(|name| name == "rust")
        })
        .collect()
}

fn expected_text(path: &str) -> String {
    let full = repo_root().join("spec/scheme-subset/expected").join(path);
    let text = std::fs::read_to_string(&full).unwrap_or_else(|error| {
        panic!("expected file {} is unreadable: {error}", full.display());
    });
    // The expected file begins with the two common license header
    // lines; the program's output starts at line 3.
    let mut out = String::new();
    for line in text.lines().skip(2) {
        out.push_str(line);
        out.push('\n');
    }
    out
}

#[test]
fn corpus_core_matches_expected_output() {
    let rows = core_rows_for_this_edition();
    assert_eq!(rows.len(), 21, "the manifest lists 21 core rows");
    for row in &rows {
        let source = std::fs::read_to_string(
            repo_root()
                .join("spec/scheme-subset/programs")
                .join(&row.program),
        )
        .unwrap_or_else(|error| panic!("{} is unreadable: {error}", row.program));
        let actual = run_program(&source);
        let want = expected_text(&row.expected);
        assert_eq!(actual, want, "program {} (mode {})", row.program, row.mode);
    }
}
