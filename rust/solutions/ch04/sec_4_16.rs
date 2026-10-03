// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.16: scan out internal
//! definitions into explicit unassigned bindings and assignments.

/// Shared typed support for this exercise.
pub mod support;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Step {
    Unassigned(String),
    Assign(String, String),
    Read(String),
}

fn scan_out(definitions: &[(String, String)], body: &str) -> Vec<Step> {
    let mut steps: Vec<Step> = definitions
        .iter()
        .map(|(name, _)| Step::Unassigned(name.clone()))
        .collect();
    steps.extend(
        definitions
            .iter()
            .map(|(name, value)| Step::Assign(name.clone(), value.clone())),
    );
    steps.push(Step::Read(body.to_owned()));
    steps
}

/// Whether evaluating `value` reads a still-unassigned name: a
/// `calls f` mention is a deferred call (a lambda body, evaluated
/// later), while any other mention reads now.
fn reads_unassigned(value: &str, pending: &[String]) -> bool {
    pending
        .iter()
        .any(|name| value.contains(name) && !value.contains(&format!("calls {name}")))
}

fn reads_before_assignment(steps: &[Step]) -> Option<String> {
    // Every name starts unassigned; an assignment evaluates its value
    // first (a premature read) and only then assigns the name.
    let mut pending = Vec::new();
    for step in steps {
        match step {
            Step::Unassigned(name) => pending.push(name.clone()),
            Step::Assign(name, value) => {
                if reads_unassigned(value, &pending) {
                    return Some(value.clone());
                }
                pending.retain(|bound| bound != name);
            }
            Step::Read(value) => {
                if reads_unassigned(value, &pending) {
                    return Some(value.clone());
                }
            }
        }
    }
    None
}

#[test]
fn ex_4_16() {
    let definitions = vec![
        ("even?".to_owned(), "calls odd?".to_owned()),
        ("odd?".to_owned(), "calls even?".to_owned()),
    ];
    let steps = scan_out(&definitions, "even?(10)");
    assert!(matches!(steps[0], Step::Unassigned(_)));
    assert!(matches!(steps[2], Step::Assign(_, _)));
    assert!(reads_before_assignment(&steps).is_none());

    let premature = vec![
        ("a".to_owned(), "reads b".to_owned()),
        ("b".to_owned(), "3".to_owned()),
    ];
    assert!(reads_before_assignment(&scan_out(&premature, "a")).is_some());
}
