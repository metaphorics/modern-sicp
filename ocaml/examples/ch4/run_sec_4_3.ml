(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

module Replay = Sicp_ch1.Replay
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value
module Eval = Sicp_ch4.Sec_4_3

let show = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let expect_value actual expected = Replay.expect (show actual) expected

(* The object-language definitions the section's programs share: the
   requirement combinator, the ambiguous generators, and the primality
   test the 4.3.1 examples run against. *)
let preamble =
  {|
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define (an-integer-between low high)
  (require (not (> low high)))
  (amb low (an-integer-between (+ low 1) high)))
(define (an-integer-starting-from n)
  (amb n (an-integer-starting-from (+ n 1))))
(define (prime? n)
  (define (divides? d) (= 0 (remainder n d)))
  (define (find-divisor d)
    (if (> (* d d) n) n (if (divides? d) d (find-divisor (+ d 1)))))
  (if (< n 2) #f (= n (find-divisor 2))))
(define (quotient x y) (if (< x y) 0 (+ 1 (quotient (- x y) y))))
(define (remainder x y) (- x (* y (quotient x y))))
(define (prime-sum-pair list1 list2)
  (let ((a (an-element-of list1)) (b (an-element-of list2)))
    (require (prime? (+ a b)))
    (list a b)))|}
;;

let () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env preamble in
  (* 4.3.1: the driver sample -- the first non-failing execution, then
     the try-again protocol, then a fresh problem. *)
  expect_value (Eval.run env "(prime-sum-pair '(1 3 5 8) '(20 35 110))") "(3 20)";
  expect_value (Eval.try_again ()) "(3 110)";
  expect_value (Eval.try_again ()) "(8 35)";
  expect_value (Eval.try_again ()) "Error: there are no more values";
  expect_value (Eval.run env "(prime-sum-pair '(19 27 30) '(11 36 58))") "(30 11)";
  expect_value (Eval.try_again ()) "Error: there are no more values";
  expect_value (Eval.try_again ()) "Error: there is no current problem";
  (* The undo trail: parse-word's set! is undone on backtracking, so a
     reparsed sentence starts from the whole input. *)
  let (_ : (Value.t, Eval_error.t) result) =
    Eval.run_program
      env
      {|
(define *unparsed* '())
(define nouns '(noun student professor cat class))
(define verbs '(verb studies lectures eats sleeps))
(define articles '(article the a))
(define prepositions '(prep for to in by with))
(define (parse-word word-list)
  (require (not (null? *unparsed*)))
  (require (member (car *unparsed*) (cdr word-list)))
  (let ((found-word (car *unparsed*)))
    (set! *unparsed* (cdr *unparsed*))
    (list (car word-list) found-word)))
(define (parse-simple-noun-phrase)
  (list 'simple-noun-phrase (parse-word articles) (parse-word nouns)))
(define (parse-noun-phrase)
  (define (maybe-extend noun-phrase)
    (amb noun-phrase
         (maybe-extend
          (list 'noun-phrase noun-phrase (parse-prepositional-phrase)))))
  (maybe-extend (parse-simple-noun-phrase)))
(define (parse-prepositional-phrase)
  (list 'prep-phrase (parse-word prepositions) (parse-noun-phrase)))
(define (parse-verb-phrase)
  (define (maybe-extend verb-phrase)
    (amb verb-phrase
         (maybe-extend
          (list 'verb-phrase verb-phrase (parse-prepositional-phrase)))))
  (maybe-extend (parse-word verbs)))
(define (parse-sentence)
  (list 'sentence (parse-noun-phrase) (parse-verb-phrase)))
(define (parse input)
  (set! *unparsed* input)
  (let ((sent (parse-sentence))) (require (null? *unparsed*)) sent))|}
  in
  expect_value
    (Eval.run env "(parse '(the professor lectures to the student with the cat))")
    "(sentence (simple-noun-phrase (article the) (noun professor)) (verb-phrase \
     (verb-phrase (verb lectures) (prep-phrase (prep to) (simple-noun-phrase (article \
     the) (noun student)))) (prep-phrase (prep with) (simple-noun-phrase (article the) \
     (noun cat)))))";
  expect_value
    (Eval.try_again ())
    "(sentence (simple-noun-phrase (article the) (noun professor)) (verb-phrase (verb \
     lectures) (prep-phrase (prep to) (noun-phrase (simple-noun-phrase (article the) \
     (noun student)) (prep-phrase (prep with) (simple-noun-phrase (article the) (noun \
     cat)))))))"
;;
