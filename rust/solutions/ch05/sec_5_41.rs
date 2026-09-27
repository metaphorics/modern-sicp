// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 5.41: `find-variable` and its
//! lexical addresses.

use ch05::sec_5_5::{Cenv, LexicalAddress, find_variable};

mod ex_5_41 {
    //! Exercise 5.41: locate names in the compile-time environment.

    use super::*;

    fn render(address: LexicalAddress) -> String {
        match address {
            LexicalAddress::Found(frame, displacement) => format!("({frame} {displacement})"),
            LexicalAddress::NotFound => "not-found".to_owned(),
        }
    }

    pub fn ex_5_41() -> Vec<String> {
        let cenv: Cenv = vec![
            vec!["y".to_owned(), "z".to_owned()],
            vec![
                "a".to_owned(),
                "b".to_owned(),
                "c".to_owned(),
                "d".to_owned(),
                "e".to_owned(),
            ],
            vec!["x".to_owned(), "y".to_owned()],
        ];
        vec![
            format!("c: {}", render(find_variable("c", &cenv))),
            format!("x: {}", render(find_variable("x", &cenv))),
            format!("w: {}", render(find_variable("w", &cenv))),
        ]
    }

    #[test]
    fn ex_5_41_check() {
        assert_eq!(ex_5_41(), vec!["c: (1 2)", "x: (2 0)", "w: not-found"]);
    }
}
