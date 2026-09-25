// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.47: Louis Reasoner's
//! `parse-verb-phrase`. In the written order it parses: the first
//! alternative consumes a verb, so the ordinary parses come out. But
//! the second alternative recurses before consuming anything, so once
//! a parse has been answered the next `try-again` descends
//! `parse-verb-phrase` endlessly on the exhausted input -- the unbounded
//! recursion Louis's version falls into. (Interchanging the two
//! `amb` expressions is worse: the first alternative then recurses
//! before any word is consumed, and even the first parse never
//! returns.) The demonstration bounds the recursion with a depth
//! parameter, which turns the divergence into the exhaustion the test
//! pins.

use ch04::eval_support::{AMB_SEED, Amb, SchemeError, setup_amb_environment, with_eval_stack};

mod ex_4_47 {
    use super::*;

    /// The section's parser with Louis's definition beside a depth-
    /// capped copy of it and of the interchanged order.
    const PARSER_LINES: &[&str] = &[
        "(define (require p) (if (not p) (amb)))",
        "(define nouns '(noun student professor cat class))",
        "(define verbs '(verb studies lectures eats sleeps))",
        "(define articles '(article the a))",
        "(define prepositions '(prep for to in by with))",
        "(define *unparsed* '())",
        "(define (memq x xs) \
         (cond ((null? xs) #f) \
         ((eq? x (car xs)) xs) \
         (else (memq x (cdr xs)))))",
        "(define (parse-word word-list) \
         (require (not (null? *unparsed*))) \
         (require (memq (car *unparsed*) (cdr word-list))) \
         (let ((found-word (car *unparsed*))) \
         (set! *unparsed* (cdr *unparsed*)) \
         (list (car word-list) found-word)))",
        "(define (parse-noun-phrase) \
         (list 'noun-phrase (parse-word articles) (parse-word nouns)))",
        "(define (parse-prepositional-phrase) \
         (list 'prep-phrase (parse-word prepositions) (parse-noun-phrase)))",
        "(define (parse-verb-phrase-louis) \
         (amb (parse-word verbs) \
         (list 'verb-phrase \
         (parse-verb-phrase-louis) \
         (parse-prepositional-phrase))))",
        "(define (parse-verb-phrase-capped depth) \
         (amb (parse-word verbs) \
         (if (< depth 25) \
         (list 'verb-phrase \
         (parse-verb-phrase-capped (+ depth 1)) \
         (parse-prepositional-phrase)) \
         (amb))))",
        "(define (parse-verb-phrase-swapped depth) \
         (amb (if (< depth 25) \
         (list 'verb-phrase \
         (parse-verb-phrase-swapped (+ depth 1)) \
         (parse-prepositional-phrase)) \
         (amb)) \
         (parse-word verbs)))",
        "(define (parse-with verb-parser input) \
         (set! *unparsed* input) \
         (let ((sent (list 'sentence (parse-noun-phrase) (verb-parser)))) \
         (require (null? *unparsed*)) \
         sent))",
    ];

    /// The first result of one parser on "the cat eats"; when `resume`
    /// is set, also whether the next `try-again` runs dry. Louis's
    /// unguarded form must never be resumed: its `try-again` is the
    /// divergence the exercise describes, and it would overflow the
    /// host stack instead of terminating.
    ///
    /// # Panics
    /// Panics when a parser raises an object error.
    #[must_use]
    pub fn parse_with(parser: &str, resume: bool) -> (Option<String>, Option<bool>) {
        let call = format!("(parse-with {parser} '(the cat eats))");
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            let env = setup_amb_environment();
            for line in PARSER_LINES {
                amb.run(line, &env).expect("the definition runs");
            }
            let mut first = None;
            let mut exhausted = None;
            match amb.run(&call, &env) {
                Ok(value) => first = Some(sicp_runtime::print_value(&value)),
                Err(SchemeError::Backtrack) => exhausted = Some(true),
                Err(error) => panic!("the parse raised: {error}"),
            }
            if resume && first.is_some() {
                exhausted = Some(matches!(amb.try_again(), Err(SchemeError::Backtrack)));
            }
            (first, exhausted)
        })
    }
}

#[test]
fn ex_4_47() {
    // Louis's version answers the ordinary first parse. Its `try-again`
    // is never taken in the test: it is the unbounded recursion the
    // exercise predicts, which diverges instead of terminating.
    let (first, _) = ex_4_47::parse_with("(lambda () (parse-verb-phrase-louis))", false);
    assert_eq!(
        first,
        Some("(sentence (noun-phrase (article the) (noun cat)) (verb eats))".to_owned())
    );
    // The depth-capped copy shows what that recursion does: the
    // try-again descends to the cap consuming nothing, then the search
    // runs dry -- no second parse exists in Louis's order.
    let (capped_first, capped_exhausted) =
        ex_4_47::parse_with("(lambda () (parse-verb-phrase-capped 0))", true);
    assert_eq!(capped_first, first);
    assert_eq!(capped_exhausted, Some(true));
    // Interchanged, the recursion precedes any consumption: even the
    // capped copy burns its whole depth budget before the verb, and the
    // first parse only completes through the second alternative.
    let (swapped_first, _) =
        ex_4_47::parse_with("(lambda () (parse-verb-phrase-swapped 0))", false);
    assert_eq!(swapped_first, first);
}
