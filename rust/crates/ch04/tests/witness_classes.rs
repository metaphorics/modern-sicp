// SPDX-License-Identifier: GPL-3.0-only

//! The grammar's §6 rejection classes, asserted on the contract's own
//! witness programs: a host-valid but excluded form reports
//! `Unsupported`, a borrow violation reports `Ownership`, and both
//! rejections happen before any guest effect.

use std::path::{Path, PathBuf};

use sicp_runtime::host::admit;
use sicp_runtime::host::diag::DiagKind;

const WITNESSES: &str = "spec/host-subsets/rust/programs/witnesses";

fn repo_root() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    for ancestor in manifest.ancestors() {
        if ancestor.join(WITNESSES).is_dir() {
            return ancestor.to_path_buf();
        }
    }
    panic!("{WITNESSES} not found above {}", manifest.display());
}

fn read_witness(name: &str) -> String {
    std::fs::read_to_string(repo_root().join(WITNESSES).join(name))
        .unwrap_or_else(|err| panic!("witness {name} is readable: {err}"))
}

#[test]
fn raw_pointer_witness_reports_unsupported() {
    let source = read_witness("unsupported-raw-pointer.rs");
    let diag = admit(&source).expect_err("the raw-pointer witness is outside the subset");
    assert_eq!(diag.kind, DiagKind::Unsupported);
}

#[test]
fn borrow_error_witness_reports_ownership() {
    let source = read_witness("borrow-error.rs");
    let diag = admit(&source).expect_err("the borrow witness violates the exclusive-borrow rule");
    assert_eq!(diag.kind, DiagKind::Ownership);
}

#[test]
fn excluded_primitive_type_reports_unsupported() {
    let source = "type Scalar = f64;\nfn main() {\n    let scalar: Scalar = 1;\n    println!(\"{scalar}\");\n}\n";
    let diag = admit(source).expect_err("`f64` is a host-valid excluded type");
    assert_eq!(diag.kind, DiagKind::Unsupported);
}

#[test]
fn float_literal_reports_unsupported() {
    let diag = admit("fn main() { 1.6e4 }").expect_err("float literals are excluded");
    assert_eq!(diag.kind, DiagKind::Unsupported);
}
