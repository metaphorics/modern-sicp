// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.50 and the tailored exercise
//! 4.50a. `ramb` is the engine's second choice form: it searches its
//! alternatives in the order a seeded xorshift draw shuffles them
//! instead of left to right, so Alyssa's generator escapes the boring
//! first-alternative recursion. Exercise 4.50a threads the seed through
//! the driver -- the evaluator holds the generator, no global state --
//! and pins the reproducibility: two drivers from one seed ramble
//! identically, a different seed rambles differently.

use ch04::eval_support::{AMB_SEED, Amb, SchemeError, setup_amb_environment, with_eval_stack};

mod ex_4_50 {
    use super::*;

    /// Alyssa's generator with `ramb-element` in `parse-word`.
    const PARSER_LINES: &[&str] = &[
        "(define (require p) (if (not p) (amb)))",
        "(define (ramb-element items) \
         (require (not (null? items))) \
         (ramb (car items) (ramb-element (cdr items))))",
        "(define nouns '(noun student professor cat class))",
        "(define verbs '(verb studies lectures eats sleeps))",
        "(define articles '(article the a))",
        "(define (parse-word word-list) \
         (list (car word-list) (ramb-element (cdr word-list))))",
        "(define (parse-simple-noun-phrase) \
         (list 'simple-noun-phrase (parse-word articles) (parse-word nouns)))",
        "(define (parse-sentence) \
         (list 'sentence (parse-simple-noun-phrase) (parse-word verbs)))",
    ];

    /// The first `n` rambled sentences from one seed.
    ///
    /// # Panics
    /// Panics when generation raises an object error.
    #[must_use]
    pub fn rambled(seed: u64, n: usize) -> Vec<String> {
        with_eval_stack(move || {
            let amb = Amb::new(seed).expect("the seed is nonzero");
            let env = setup_amb_environment();
            for line in PARSER_LINES {
                amb.run(line, &env).expect("the definition runs");
            }
            let mut out = Vec::new();
            let first = amb
                .run("(parse-sentence)", &env)
                .expect("a sentence generates");
            out.push(sicp_runtime::print_value(&first));
            while out.len() < n {
                match amb.try_again() {
                    Ok(value) => out.push(sicp_runtime::print_value(&value)),
                    Err(SchemeError::Backtrack) => break,
                    Err(error) => panic!("generation raised: {error}"),
                }
            }
            out
        })
    }
}

#[test]
fn ex_4_50() {
    // The rambled order varies the sentences the deterministic search
    // would open with: the first sentence under the session seed is not
    // the (the student studies) the left-to-right search answers.
    let first = ex_4_50::rambled(AMB_SEED, 3);
    assert_eq!(
        first,
        vec![
            "(sentence (simple-noun-phrase (article a) (noun professor)) (verb studies))",
            "(sentence (simple-noun-phrase (article a) (noun professor)) (verb eats))",
            "(sentence (simple-noun-phrase (article a) (noun professor)) (verb sleeps))",
        ]
    );
}

#[test]
fn ex_4_50a() {
    // The seed is threaded through the driver: two evaluators from one
    // seed ramble identically.
    let left = ex_4_50::rambled(AMB_SEED, 3);
    let right = ex_4_50::rambled(AMB_SEED, 3);
    assert_eq!(left, right);
    // A different seed draws a different order.
    let other = ex_4_50::rambled(7, 3);
    assert_ne!(left, other);
    // And the variant still covers the language: the third seed's
    // sentences are well-formed parses of the same grammar.
    let third = ex_4_50::rambled(31, 3);
    for sentence in third {
        assert!(sentence.starts_with("(sentence (simple-noun-phrase (article "));
    }
    let _ = SchemeError::Backtrack;
}
