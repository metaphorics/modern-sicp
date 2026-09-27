// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.41: an ordinary Rust program
//! solves the multiple-dwelling puzzle. No `amb` and no search engine:
//! the host enumerates the assignments itself, rejects the ones a
//! restriction rules out, and keeps the survivor. The answer agrees
//! with the nondeterministic procedure of section 4.3.2.

mod ex_4_41 {
    /// One assignment: the floor each person lives on, people in the
    /// book's order.
    type Assignment = [u8; 5];

    const NAMES: [&str; 5] = ["baker", "cooper", "fletcher", "miller", "smith"];

    /// The restrictions of the puzzle, over one assignment.
    fn holds(a: Assignment) -> bool {
        let [baker, cooper, fletcher, miller, smith] = a;
        let distinct = a.iter().collect::<std::collections::HashSet<_>>().len() == 5;
        distinct
            && baker != 5
            && cooper != 1
            && fletcher != 5
            && fletcher != 1
            && miller > cooper
            && i16::from(smith) - i16::from(fletcher) != 1
            && i16::from(smith) - i16::from(fletcher) != -1
            && i16::from(fletcher) - i16::from(cooper) != 1
            && i16::from(fletcher) - i16::from(cooper) != -1
    }

    /// Every solution: the host walks all 5^5 assignments.
    #[must_use]
    pub fn solutions() -> Vec<Assignment> {
        let mut out = Vec::new();
        for baker in 1..=5u8 {
            for cooper in 1..=5u8 {
                for fletcher in 1..=5u8 {
                    for miller in 1..=5u8 {
                        for smith in 1..=5u8 {
                            let a = [baker, cooper, fletcher, miller, smith];
                            if holds(a) {
                                out.push(a);
                            }
                        }
                    }
                }
            }
        }
        out
    }

    /// The printed form the nondeterministic section answers with.
    #[must_use]
    pub fn printed(a: Assignment) -> String {
        let pairs: Vec<String> = a
            .iter()
            .zip(NAMES)
            .map(|(floor, name)| format!("({name} {floor})"))
            .collect();
        format!("({})", pairs.join(" "))
    }
}

#[test]
fn ex_4_41() {
    let solutions = ex_4_41::solutions();
    // Exactly one assignment satisfies the puzzle, the book's answer.
    assert_eq!(solutions.len(), 1);
    assert_eq!(
        ex_4_41::printed(solutions[0]),
        "((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"
    );
}
