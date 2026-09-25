(* SPDX-License-Identifier: GPL-3.0-only
   Original exercise *)

(** Exercise 4.49: Alyssa's sentence generation. Her [parse-word]
    ignores the unparsed input, draws a word of the demanded part of
    speech from [an-element-of], and always succeeds; [parse] is then
    called with the empty input, whose trailing distinctness
    requirement passes vacuously, and the driver's [try_again] walks
    the generated sentences. The demonstration pins the first six
    sentences, which descend the grammar's first alternatives -- the
    boring, badly sampled output the footnote describes and exercise
    4.50's [ramb] addresses. *)

module Eval = Sicp_ch4.Sec_4_3
module Eval_error = Sicp_common.Eval_error
module Value = Sicp_common.Value

let show = function
  | Ok v -> Value.to_string v
  | Error e -> "Error: " ^ Eval_error.to_string e
;;

let program =
  {|
(define (require p) (if (not p) (amb)))
(define (an-element-of items)
  (require (not (null? items)))
  (amb (car items) (an-element-of (cdr items))))
(define *unparsed* '())
(define nouns '(noun student professor cat class))
(define verbs '(verb studies lectures eats sleeps))
(define articles '(article the a))
(define prepositions '(prep for to in by with))
(define (parse-word word-list)
  (let ((found-word (an-element-of (cdr word-list))))
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
;;

let ex_4_49 () =
  let env = Eval.the_global_environment () in
  let (_ : (Value.t, Eval_error.t) result) = Eval.run_program env program in
  let first = Eval.run env "(parse '())" in
  let second = Eval.try_again () in
  let third = Eval.try_again () in
  let fourth = Eval.try_again () in
  let fifth = Eval.try_again () in
  let sixth = Eval.try_again () in
  List.map show [ first; second; third; fourth; fifth; sixth ]
;;
