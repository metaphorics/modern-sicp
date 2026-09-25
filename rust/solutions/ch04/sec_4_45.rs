// SPDX-License-Identifier: GPL-3.0-only
// Original exercise

//! The reference solution of exercise 4.45: the five parses of "The
//! professor lectures to the student in the class with the cat" under
//! the section's extended grammar, where both noun phrases and verb
//! phrases extend by prepositional phrases. Each `try-again` moves one
//! attachment: the two `in`/`with` phrases attach inside the `to`
//! phrase's noun phrase, inside each other, or one on each side, and
//! after the fifth the search runs dry.

use ch04::eval_support::{AMB_SEED, Amb, SchemeError, setup_amb_environment, with_eval_stack};

mod ex_4_45 {
    use super::*;

    /// The section's parser in its final, fully extended shape, and the
    /// exercise's sentence.
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
        "(define (parse-simple-noun-phrase) \
         (list 'simple-noun-phrase (parse-word articles) (parse-word nouns)))",
        "(define (parse-noun-phrase) \
         (define (maybe-extend noun-phrase) \
         (amb noun-phrase \
         (maybe-extend (list 'noun-phrase noun-phrase (parse-prepositional-phrase))))) \
         (maybe-extend (parse-simple-noun-phrase)))",
        "(define (parse-verb-phrase) \
         (define (maybe-extend verb-phrase) \
         (amb verb-phrase \
         (maybe-extend (list 'verb-phrase verb-phrase (parse-prepositional-phrase))))) \
         (maybe-extend (parse-word verbs)))",
        "(define (parse-prepositional-phrase) \
         (list 'prep-phrase (parse-word prepositions) (parse-noun-phrase)))",
        "(define (parse-sentence) \
         (list 'sentence (parse-noun-phrase) (parse-verb-phrase)))",
        "(define (parse input) \
         (set! *unparsed* input) \
         (let ((sent (parse-sentence))) \
         (require (null? *unparsed*)) \
         sent))",
    ];

    const SENTENCE: &str =
        "(parse '(the professor lectures to the student in the class with the cat))";

    /// Every parse of the exercise's sentence, printed, then the
    /// exhaustion report.
    ///
    /// # Panics
    /// Panics when a resumed parse raises an object error.
    #[must_use]
    pub fn parses() -> (Vec<String>, bool) {
        with_eval_stack(move || {
            let amb = Amb::new(AMB_SEED).expect("the seed is nonzero");
            let env = setup_amb_environment();
            for line in PARSER_LINES {
                amb.run(line, &env).expect("the definition runs");
            }
            let mut out = Vec::new();
            let first = amb.run(SENTENCE, &env);
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
fn ex_4_45() {
    let (parses, exhausted) = ex_4_45::parses();
    // The book's five parses, then no more.
    assert_eq!(parses.len(), 5);
    assert!(exhausted);
    // Parse 1: both phrases modify the verb phrase, deepest first.
    assert!(parses[0].starts_with(
        "(sentence (simple-noun-phrase (article the) (noun professor)) \
         (verb-phrase (verb-phrase (verb-phrase (verb lectures)"
    ));
    assert!(
        parses[0]
            .contains("(prep-phrase (prep to) (simple-noun-phrase (article the) (noun student)))")
    );
    // Parse 2: "with the cat" attaches inside "in the class"'s noun
    // phrase.
    assert!(parses[1].contains(
        "(prep-phrase (prep in) (noun-phrase (simple-noun-phrase (article the) \
         (noun class)) (prep-phrase (prep with) (simple-noun-phrase (article the) \
         (noun cat)))))"
    ));
    // Parse 3: "in the class" attaches inside "to the student"'s noun
    // phrase.
    assert!(parses[2].contains(
        "(prep-phrase (prep to) (noun-phrase (simple-noun-phrase (article the) \
         (noun student)) (prep-phrase (prep in) (simple-noun-phrase (article the) \
         (noun class)))))"
    ));
    // Parse 4: both attach at the student's noun phrase, left-nested.
    assert!(parses[3].contains(
        "(prep-phrase (prep to) (noun-phrase (noun-phrase (simple-noun-phrase \
         (article the) (noun student)) (prep-phrase (prep in) (simple-noun-phrase \
         (article the) (noun class)))) (prep-phrase (prep with) \
         (simple-noun-phrase (article the) (noun cat)))))"
    ));
    // Parse 5: same attachment, right-nested.
    assert!(parses[4].contains(
        "(prep-phrase (prep to) (noun-phrase (simple-noun-phrase (article the) \
         (noun student)) (prep-phrase (prep in) (noun-phrase (simple-noun-phrase \
         (article the) (noun class)) (prep-phrase (prep with) \
         (simple-noun-phrase (article the) (noun cat)))))))"
    ));
    // All five begin from the same subject and verb.
    for parse in &parses {
        assert!(parse.starts_with("(sentence (simple-noun-phrase (article the) (noun professor))"));
    }
}
