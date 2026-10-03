// SPDX-License-Identifier: GPL-3.0-only

//! Runtime behavior of admitted operations whose checked type and whose
//! execution once disagreed: what the checker admits, both section 4.1
//! engines must run to the answer the pinned native toolchain gives.

use ch04::sec_4_1::{run, run_analyzed};
use sicp_runtime::host::admit;

/// Runs `source` on the direct and the analyzed engine, requires the two
/// to agree, and answers the stdout.
fn stdout_of(source: &str) -> String {
    let program = admit(source).expect("the program is admitted");
    let direct = run(&program);
    let analyzed = run_analyzed(&program);
    assert!(
        direct.trap.is_none(),
        "direct run trapped: {:?}",
        direct.trap
    );
    assert!(
        analyzed.trap.is_none(),
        "analyzed run trapped: {:?}",
        analyzed.trap
    );
    assert_eq!(direct.stdout, analyzed.stdout, "the two engines disagree");
    direct.stdout
}

#[test]
fn vec_with_capacity_takes_a_usize_capacity() {
    let source = r#"
fn main() {
    let mut v: Vec<i64> = Vec::with_capacity(4);
    v.push(7);
    println!("{}", v.len());
}
"#;
    assert_eq!(stdout_of(source), "1\n");
}

#[test]
fn hashmap_iter_yields_key_and_value_references() {
    let source = r#"
use std::collections::HashMap;
fn main() {
    let mut m: HashMap<String, i64> = HashMap::new();
    m.insert(String::from("x"), 5);
    m.insert(String::from("y"), 6);
    let mut total: i64 = 0;
    let mut letters: usize = 0;
    for (key, value) in m.iter() {
        total += *value;
        letters += key.len();
    }
    println!("{} {}", total, letters);
    let mut lone: HashMap<String, i64> = HashMap::new();
    lone.insert(String::from("only"), 1);
    for (key, value) in lone.iter() {
        println!("{} {}", key, value);
    }
}
"#;
    assert_eq!(stdout_of(source), "11 2\nonly 1\n");
}

#[test]
fn a_bare_none_pattern_matches_only_none() {
    let source = r#"
fn pick(o: Option<i64>) -> i64 {
    match o {
        None => 1,
        Some(_) => 2,
    }
}
fn main() {
    println!("{}", pick(Some(3)));
    println!("{}", pick(None));
}
"#;
    assert_eq!(stdout_of(source), "2\n1\n");
}

#[test]
fn moved_locals_leave_their_slots_only_where_rust_moves_them() {
    let source = r#"
fn consume(s: String) -> usize {
    s.len()
}
fn main() {
    let a = String::from("abc");
    let n = a.len();
    let b = a.clone();
    let m = consume(a);
    println!("{} {} {}", n, b, m);
    let t = (1i64, 2i64);
    let u = t;
    println!("{} {}", t.0, u.1);
}
"#;
    assert_eq!(stdout_of(source), "3 abc 3\n1 2\n");
}
