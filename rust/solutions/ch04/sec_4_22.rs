// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.22: let in the analyzed evaluator, through the analyzer
// seam: the rewrite happens once, at analysis time..

use ch04::eval_support::*;

mod ex_4_22 {
    use super::*;

    /// The analyzed evaluator that also accepts `let`: a let analyzes
    /// into the analysis of its combination.
    pub struct LetAnalyzer {
        bodies: Bodies,
    }

    impl Default for LetAnalyzer {
        fn default() -> Self {
            Self {
                bodies: Rc::new(RefCell::new(HashMap::new())),
            }
        }
    }

    impl Analyzer for LetAnalyzer {
        fn analyze(&self, exp: &Value) -> Result<Exec, SchemeError> {
            if is_let(exp) {
                let rewritten = let_to_combination(exp)?;
                return self.analyze(&rewritten);
            }
            self.base_analyze(exp)
        }

        fn bodies(&self) -> &Bodies {
            &self.bodies
        }
    }

    /// Answers the analyzed value of a plain let and of a lambda body
    /// whose internal let runs twice under one analysis.
    pub fn answers() -> Result<Vec<String>, SchemeError> {
        let base = AnalyzerBase::default();
        let plain = read("(let ((x 3) (y 4)) (+ x y))").expect("read");
        let rejected = base
            .eval_exp(&plain, &setup_environment())
            .expect_err("the base analyzer has no let");
        assert!(rejected.to_string().contains("let"), "{rejected}");

        let env = setup_environment();
        let analyzer = LetAnalyzer::default();
        let definition = read("(define (g n) (let ((m (* n 2))) (+ m 1)))").expect("read");
        analyzer.eval_exp(&definition, &env)?;
        let call = read("(g 2)").expect("read");
        let first = analyzer.eval_exp(&call, &env)?;
        let again = analyzer.eval_exp(&call, &env)?;
        let plain_value = analyzer.eval_exp(&plain, &env)?;
        Ok(vec![
            print_value(&plain_value),
            print_value(&first),
            print_value(&again),
        ])
    }
}

#[test]
fn ex_4_22() {
    let values = ex_4_22::answers().expect("runs");
    // The analyzed let answers like the base evaluator's...
    assert_eq!(values, vec!["7", "5", "5"]);
    // ...and the lambda's internal let was analyzed once but runs for
    // every call, twice here with the same answer.
}
