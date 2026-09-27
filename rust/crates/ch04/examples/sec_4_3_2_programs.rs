// SPDX-License-Identifier: GPL-3.0-only
// Adapted from the Scheme programs in SICP section 4.3

//! Section 4.3.2: nondeterministic programs. The multiple-dwelling
//! puzzle answers from enumeration plus restrictions, and the parser
//! turns ambiguous sentences into searches whose `try-again` yields the
//! alternative parses.

use ch04::eval_support::{amb_answers, amb_session};

const DWELLING: &str = r"
(define (require p) (if (not p) (amb)))
(define (distinct? items)
  (cond ((null? items) #t)
        ((null? (cdr items)) #t)
        ((member (car items) (cdr items)) #f)
        (else (distinct? (cdr items)))))
(define (member x xs)
  (cond ((null? xs) #f)
        ((equal? x (car xs)) xs)
        (else (member x (cdr xs)))))
(define (multiple-dwelling)
  (let ((baker (amb 1 2 3 4 5)) (cooper (amb 1 2 3 4 5))
        (fletcher (amb 1 2 3 4 5)) (miller (amb 1 2 3 4 5))
        (smith (amb 1 2 3 4 5)))
    (require (distinct? (list baker cooper fletcher miller smith)))
    (require (not (= baker 5)))
    (require (not (= cooper 1)))
    (require (not (= fletcher 5)))
    (require (not (= fletcher 1)))
    (require (> miller cooper))
    (require (not (= (abs (- smith fletcher)) 1)))
    (require (not (= (abs (- fletcher cooper)) 1)))
    (list (list 'baker baker) (list 'cooper cooper)
          (list 'fletcher fletcher) (list 'miller miller)
          (list 'smith smith))))
(multiple-dwelling)";

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

fn main() {
    // The puzzle's unique answer.
    let dwelling_lines: Vec<&str> = DWELLING.lines().collect();
    let answers = amb_answers(&dwelling_lines);
    println!("{answers:?}");
    // => ["((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"]
    assert_eq!(
        answers,
        vec!["((baker 3) (cooper 2) (fletcher 4) (miller 5) (smith 1))"]
    );

    // The ambiguous sentence: one parse first, the alternative on
    // try-again -- the attachment of "with the cat" is the difference
    // between them.
    let parses: Vec<String> = {
        let mut lines: Vec<&str> = PARSER_LINES.to_vec();
        lines.push("(parse '(the professor lectures to the student with the cat))");
        lines.push("try-again");
        let values: Vec<String> = amb_session(&lines)
            .lines()
            .filter_map(|line| line.strip_prefix(";;; Amb-Eval value: "))
            .map(str::to_owned)
            .collect();
        values[values.len() - 2..].to_vec()
    };
    println!("{} parses", parses.len());
    for parse in &parses {
        println!("{parse}");
    }
    assert_eq!(parses.len(), 2);
    assert!(parses[0].contains(
        "(verb-phrase (verb-phrase (verb lectures) (prep-phrase (prep to) \
         (simple-noun-phrase (article the) (noun student)))) (prep-phrase (prep with) \
         (simple-noun-phrase (article the) (noun cat))))"
    ));
    assert!(parses[1].contains(
        "(prep-phrase (prep to) (noun-phrase (simple-noun-phrase (article the) \
         (noun student)) (prep-phrase (prep with) (simple-noun-phrase (article the) \
         (noun cat)))))"
    ));

    // The section's simple sentence parses under the extended grammar.
    let session = {
        let mut lines: Vec<&str> = PARSER_LINES.to_vec();
        lines.push("(parse '(the cat eats))");
        amb_session(&lines)
    };
    println!("{session}");
    assert!(session.contains(
        ";;; Amb-Eval value: (sentence (simple-noun-phrase (article the) (noun cat)) (verb eats))\n"
    ));
}
