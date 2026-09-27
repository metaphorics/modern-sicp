// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.48: an extended grammar.
//! Adjectives join the noun phrase and adverbs the verb phrase as
//! explicit `amb` alternatives -- zero, one, or two modifiers before the
//! article, zero or one adverb after the verb -- so the extension stays
//! the section's shape (a choice among phrase shapes) while every
//! alternative consumes input and the parses stay finite.

use ch04::eval_support::{AMB_SEED, Amb, SchemeError, setup_amb_environment, with_eval_stack};

mod ex_4_48 {
    use super::*;

    /// The extended parser.
    const PARSER_LINES: &[&str] = &[
        "(define (require p) (if (not p) (amb)))",
        "(define nouns '(noun student professor cat class))",
        "(define verbs '(verb studies lectures eats sleeps))",
        "(define articles '(article the a))",
        "(define adjectives '(adj quick brown sleepy))",
        "(define adverbs '(adv quickly slowly))",
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
        "(define (parse-simple-noun-phrase) \
         (list 'simple-noun-phrase \
         (parse-word articles) \
         (amb '() \
         (list (parse-word adjectives)) \
         (list (parse-word adjectives) (parse-word adjectives))) \
         (parse-word nouns)))",
        "(define (parse-verb-phrase) \
         (list 'verb-phrase \
         (parse-word verbs) \
         (amb '() (list (parse-word adverbs)))))",
        "(define (parse-sentence) \
         (list 'sentence (parse-simple-noun-phrase) (parse-verb-phrase)))",
        "(define (parse input) \
         (set! *unparsed* input) \
         (let ((sent (parse-sentence))) \
         (require (null? *unparsed*)) \
         sent))",
    ];

    /// Every parse of one sentence, printed, and whether the search
    /// then ran dry.
    ///
    /// # Panics
    /// Panics when a parse raises an object error.
    #[must_use]
    pub fn parses(sentence: &str) -> (Vec<String>, bool) {
        let call = format!("(parse '({sentence}))");
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            let env = setup_amb_environment();
            for line in PARSER_LINES {
                amb.run(line, &env).expect("the definition runs");
            }
            let mut out = Vec::new();
            let first = amb.run(&call, &env);
            let mut exhausted = matches!(first, Err(SchemeError::Backtrack));
            if let Ok(value) = first {
                out.push(sicp_runtime::print_value(&value));
            }
            loop {
                match amb.try_again() {
                    Ok(value) => out.push(sicp_runtime::print_value(&value)),
                    Err(SchemeError::Backtrack) => {
                        exhausted = true;
                        break;
                    }
                    Err(error) => panic!("the resumed parse raised: {error}"),
                }
            }
            (out, exhausted)
        })
    }
}

#[test]
fn ex_4_48() {
    // The full sentence parses with two adjectives and one adverb, and
    // that is its only shape -- every other alternative mismatches the
    // input, so the search runs dry after it.
    let (parses, exhausted) = ex_4_48::parses("the quick brown cat eats slowly");
    assert_eq!(
        parses,
        [
            "(sentence (simple-noun-phrase (article the) ((adj quick) (adj brown)) \
          (noun cat)) (verb-phrase (verb eats) ((adv slowly))))"
        ]
        .map(|s| s.replace("          ", " "))
    );
    assert!(exhausted);
    // A shorter sentence admits only the adjective parse: the bare
    // article reading dies on "sleepy", the two-adjective reading dies
    // on "cat", and the search reports exhaustion after the one parse.
    let (parses, exhausted) = ex_4_48::parses("the sleepy cat eats");
    assert_eq!(parses.len(), 1);
    assert!(parses[0].contains("(simple-noun-phrase (article the) ((adj sleepy)) (noun cat))"));
    assert!(exhausted);
}
