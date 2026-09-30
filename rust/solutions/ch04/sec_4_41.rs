// SPDX-License-Identifier: GPL-3.0-only

//! The reference solution of exercise 4.41: an ordinary Rust program
//! solves the multiple-dwelling puzzle.

/// Shared typed support for this exercise.
pub mod support;

const SOLVER: &str = "
fn adjacent(left: i64, right: i64) -> bool {
    let difference = left - right;
    difference == 1 || difference == -1
}

fn main() {
    let mut solutions: Vec<String> = Vec::new();
    for baker in 1..6 {
        for cooper in 1..6 {
            for fletcher in 1..6 {
                for miller in 1..6 {
                    for smith in 1..6 {
                        if baker == cooper || baker == fletcher || baker == miller || baker == smith {
                            continue;
                        }
                        if cooper == fletcher || cooper == miller || cooper == smith {
                            continue;
                        }
                        if fletcher == miller || fletcher == smith || miller == smith {
                            continue;
                        }
                        if baker == 5 || cooper == 1 || fletcher == 1 || fletcher == 5 {
                            continue;
                        }
                        if miller <= cooper || adjacent(smith, fletcher) || adjacent(fletcher, cooper) {
                            continue;
                        }
                        solutions.push(format!(\"{} {} {} {} {}\", baker, cooper, fletcher, miller, smith));
                    }
                }
            }
        }
    }
    for solution in &solutions {
        println!(\"{}\", solution);
    }
    println!(\"{}\", solutions.len());
}
";

#[test]
fn ex_4_41() {
    let outcome = support::direct(SOLVER).expect("ordinary program runs");
    assert_eq!(outcome.stdout, "3 2 4 5 1\n1\n");
}
