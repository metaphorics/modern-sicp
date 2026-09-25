// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.49: Alyssa's generator. Her
//! `parse-word` ignores the input sentence and answers an ambiguous
//! word: the part-of-speech tag with `an-element-of` over the word
//! list. The parser then generates instead of parsing, and repeated
//! `try-again`s walk the sentences in the search's order -- the boring
//! first-alternative recursion the book's footnote describes.

use ch04::eval_support::{AMB_SEED, Amb, SchemeError, setup_amb_environment, with_eval_stack};

mod ex_4_49 {
    use super::*;

    /// The generating grammar.
    const PARSER_LINES: &[&str] = &[
        "(define (require p) (if (not p) (amb)))",
        "(define (an-element-of items) \
         (require (not (null? items))) \
         (amb (car items) (an-element-of (cdr items))))",
        "(define nouns '(noun student professor cat class))",
        "(define verbs '(verb studies lectures eats sleeps))",
        "(define articles '(article the a))",
        "(define (parse-word word-list) \
         (list (car word-list) (an-element-of (cdr word-list))))",
        "(define (parse-simple-noun-phrase) \
         (list 'simple-noun-phrase (parse-word articles) (parse-word nouns)))",
        "(define (parse-sentence) \
         (list 'sentence (parse-simple-noun-phrase) (parse-word verbs)))",
    ];

    /// The first `n` generated sentences.
    ///
    /// # Panics
    /// Panics when generation raises an object error.
    #[must_use]
    pub fn sentences(n: usize) -> Vec<String> {
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
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
fn ex_4_49() {
    // The first half-dozen: every verb with the first article-noun
    // pair, then the second noun with the first verb.
    assert_eq!(
        ex_4_49::sentences(6),
        vec![
            "(sentence (simple-noun-phrase (article the) (noun student)) (verb studies))",
            "(sentence (simple-noun-phrase (article the) (noun student)) (verb lectures))",
            "(sentence (simple-noun-phrase (article the) (noun student)) (verb eats))",
            "(sentence (simple-noun-phrase (article the) (noun student)) (verb sleeps))",
            "(sentence (simple-noun-phrase (article the) (noun professor)) (verb studies))",
            "(sentence (simple-noun-phrase (article the) (noun professor)) (verb lectures))",
        ]
    );
}
