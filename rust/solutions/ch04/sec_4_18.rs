// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.18: the alternative scan-out
//! defers initializers and makes the premature-read restriction real.

/// Shared typed support for this exercise.
pub mod support;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Step {
    Unassigned(String),
    Evaluate(String),
    Assign(String),
}

fn alternative_scan_out(definitions: &[(String, String)]) -> Vec<Step> {
    let mut steps: Vec<Step> = definitions
        .iter()
        .map(|(name, _)| Step::Unassigned(name.clone()))
        .collect();
    // Exercise 4.18's alternative: all initializers evaluate while
    // every name is still unassigned; only then do the assignments run.
    for (_, value) in definitions {
        steps.push(Step::Evaluate(value.clone()));
    }
    for (name, _) in definitions {
        steps.push(Step::Assign(name.clone()));
    }
    steps
}

fn premature_read(steps: &[Step]) -> Option<String> {
    let mut pending = Vec::new();
    for step in steps {
        match step {
            Step::Unassigned(name) => pending.push(name.clone()),
            Step::Evaluate(value) => {
                if pending.iter().any(|name| value.contains(name)) {
                    return Some(value.clone());
                }
            }
            Step::Assign(name) => pending.retain(|bound| bound != name),
        }
    }
    None
}

#[test]
fn ex_4_18() {
    let definitions = vec![
        ("u".to_owned(), "5".to_owned()),
        ("v".to_owned(), "reads u".to_owned()),
    ];
    assert!(premature_read(&alternative_scan_out(&definitions)).is_some());
}
